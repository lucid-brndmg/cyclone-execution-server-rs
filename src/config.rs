use serde::Deserialize;
use std::{env, io};
use std::path::PathBuf;
use regex::Regex;
use tracing::{info, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use crate::cli::CliArgs;
use crate::cyclone::{locate_cyclone_entry, prepare_cyclone_env};

fn cyclone_executable() -> String { "cyclone.jar".to_string() }

fn cyclone_trace_keyword() -> String { "Trace Generated:".to_string() }

fn cyclone_extension() -> String { ".cyclone".to_string() }

fn bool_true() -> bool {true}

fn cyclone_disabled_options() -> Vec<String> { Vec::new() } // {vec!["debug".to_string()]}

fn exec_id_length() -> usize {8}

fn exec_server_host() -> String {"127.0.0.1".to_string()}

fn default_log_level() -> String {"info".to_string()}

fn default_pattern_name() -> String {"#unnamed".to_string()}

fn default_log_rotation() -> String {"never".to_string()}

fn current_dir() -> String { ".".to_string() }

fn is_valid_level(level: &str) -> bool {
    return level == "debug" || level == "info" || level == "warn" || level == "error"
}

const ENV_PREFIX: &str = "CYCLONE_ES_";
const ENV_PREFIX_LEN: usize = ENV_PREFIX.len();
const SERVER_PORT_DEFAULT: u16 = 9000;
const LOG_FILENAME: &str = "cyclone-exec-server";
const DEFAULT_CYCLONE_FILE: &str = "cyclone.jar";

#[derive(Deserialize)]
pub struct ServerConfig {
    #[serde(default = "exec_server_host")]
    pub host: String,
    #[serde(default)] // 0
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: exec_server_host(),
            port: SERVER_PORT_DEFAULT
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneConfig {
    #[serde(default)]
    pub path: String,
    #[serde(default = "cyclone_executable")]
    pub executable: String,
    #[serde(default = "cyclone_trace_keyword")]
    pub trace_keyword: String,
    #[serde(default)]
    pub source_path: String,
    #[serde(default = "bool_true")]
    pub delete_after_exec: bool,
    #[serde(default = "cyclone_extension")]
    pub extension: String,
    #[serde(default = "cyclone_disabled_options")]
    pub disabled_options: Vec<String>,
    #[serde(default)]
    pub mandatory_timeout_ms: usize,
    #[serde(default = "bool_true")]
    pub append_env_path: bool,
    #[serde(default = "exec_id_length")]
    pub id_length: usize,
    #[serde(default)]
    pub censor_system_paths: bool,

    // extended configs
    #[serde(default)]
    pub silence_mode: bool,
    #[serde(default)]
    pub disable_syntax_check: bool
}

impl Default for CycloneConfig {
    fn default() -> Self {
        Self {
            path: String::default(),
            executable: cyclone_executable(),
            trace_keyword: cyclone_trace_keyword(),
            source_path: String::default(),
            delete_after_exec: bool_true(),
            extension: cyclone_extension(),
            disabled_options: cyclone_disabled_options(),
            mandatory_timeout_ms: 0,
            append_env_path: bool_true(),
            id_length: exec_id_length(),
            censor_system_paths: false,
            disable_syntax_check: false,
            silence_mode: false
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggerPattern {
    #[serde(default)]
    pub re: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub keep_file: bool,
    #[serde(default)]
    pub print_output: bool,
    #[serde(default)]
    pub print_input: bool,
    #[serde(default = "default_pattern_name")]
    pub name: String
}

pub struct CompiledExecutionLoggerPattern {
    pub re: Regex,
    pub level: tracing::Level,
    pub keep_file: bool,
    pub print_output: bool,
    pub print_input: bool,
    pub name: String
}

#[derive(Deserialize)]
pub struct LoggerConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "bool_true")]
    pub console: bool,
    #[serde(default)]
    pub execution_patterns: Option<Vec<LoggerPattern>>,
    #[serde(default = "current_dir")]
    pub directory: String,
    #[serde(default = "default_log_rotation")]
    pub rotation: String,
    #[serde(default)]
    pub file_prefix: String,
    #[serde(default)]
    pub no_color: bool
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            console: true,
            execution_patterns: None,
            directory: current_dir(),
            rotation: default_log_rotation(),
            file_prefix: String::default(),
            no_color: false
        }
    }
}

pub fn parse_log_level(level: &str) -> Option<tracing::Level> {
    match level.to_ascii_lowercase().trim() {
        "trace" => Some(tracing::Level::TRACE),
        "debug" => Some(tracing::Level::DEBUG),
        "info" => Some(tracing::Level::INFO),
        "warn" => Some(tracing::Level::WARN),
        "error" => Some(tracing::Level::ERROR),
        _ => None
    }
}

fn parse_fallback_log_level(level: &str) -> tracing::Level {
    if let Some(level) = parse_log_level(level) {
        level
    } else {
        println!("WARN: Invalid level: {level}, fallback to INFO");
        tracing::Level::INFO
    }
}

impl LoggerConfig {
    fn get_file_appender(&self) -> RollingFileAppender {
        RollingFileAppender::builder()
            .rotation(
                match self.rotation.trim().to_ascii_lowercase().as_str() {
                    "minutely" => Rotation::MINUTELY,
                    "hourly" => Rotation::HOURLY,
                    "daily" => Rotation::DAILY,
                    "never" | "" | _ => Rotation::NEVER,
                }
            )
            .filename_prefix(if self.file_prefix.is_empty() { LOG_FILENAME } else { &self.file_prefix })
            .filename_suffix("log") // log file names will be suffixed with `.log`
            .build(&self.directory) // try to build an appender that stores log files in `/var/log`\
            .expect("initializing rolling file appender failed")
    }

    pub fn initialize(&self) {
        let write_file = !self.directory.is_empty();
        let builder = tracing_subscriber::fmt()
            .with_max_level(parse_fallback_log_level(&self.level))
            .with_target(false)
            .with_ansi(!self.no_color);
        if self.console && write_file {
            builder
                .with_writer(
                    io::stdout
                        .and(self.get_file_appender())
                )
                .init()
        } else if self.console {
            builder
                .with_writer(io::stdout)
                .init()
        } else if write_file {
            builder
                .with_writer(self.get_file_appender())
                .init()
        } else {
            println!("WARN: Logger disabled, no logging message will be printed out!")
        };

        // tracing::subscriber::set_global_default(&server).unwrap();
    }

    pub fn compile_patterns(&self) -> Vec<CompiledExecutionLoggerPattern> {
        let mut compiled_patterns = Vec::new();
        if let Some(patterns) = &self.execution_patterns {
            for pattern in patterns {
                let level = &pattern.level;
                if let Some(level) = parse_log_level(level) {
                    let re_str = &pattern.re;
                    if let Ok(compiled_re) = Regex::new(re_str) {
                        compiled_patterns.push(CompiledExecutionLoggerPattern {
                            re: compiled_re,
                            level,
                            keep_file: pattern.keep_file,
                            print_input: pattern.print_input,
                            print_output: pattern.print_output,
                            name: pattern.name.to_string()
                        })
                    } else {
                        warn!("ignored execution logging pattern due to invalid regex: {}", re_str)
                    }
                } else {
                    warn!("ignored execution logging pattern due to invalid level: {}", level)
                }
            }
        }

        compiled_patterns
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cyclone: CycloneConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub logger: LoggerConfig
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cyclone: CycloneConfig::default(),
            server: ServerConfig::default(),
            logger: LoggerConfig::default()
        }
    }
}

impl Config {
    pub fn from_json(content: &str) -> Option<Self> {
        serde_json::from_str(content).ok()
    }

    pub fn load_env(&mut self) {
        let vars = env::vars();
        for (key, value) in vars {
            if !key.starts_with(ENV_PREFIX) {
                continue
            }
            let ck = &key[ENV_PREFIX_LEN..];
            match ck {
                "CYCLONE_PATH" => self.cyclone.path = value,
                "CYCLONE_EXECUTABLE" => self.cyclone.executable = value,
                "CYCLONE_APPEND_ENV" => self.cyclone.append_env_path = value.parse::<bool>().unwrap_or(false),
                "CYCLONE_SOURCE_PATH" => self.cyclone.source_path = value,
                "CYCLONE_TIMEOUT_MS" => self.cyclone.mandatory_timeout_ms = value.parse::<usize>().unwrap_or(0),
                "CYCLONE_ID_LENGTH" => self.cyclone.id_length = value.parse::<usize>().unwrap_or(exec_id_length()),
                "CYCLONE_TRACE_KEYWORD" => self.cyclone.trace_keyword = value,
                "CYCLONE_DEL_AFTER_EXEC" => self.cyclone.delete_after_exec = value.parse::<bool>().unwrap_or(false),
                "CYCLONE_EXTENSION" => self.cyclone.extension = value,
                "CYCLONE_DISABLED_OPTIONS" => self.cyclone.disabled_options = value
                    .split(",")
                    .map(|it| it.trim().to_string())
                    .collect(),
                "CYCLONE_CENSOR_SYSTEM_PATHS" => self.cyclone.censor_system_paths = value.parse::<bool>().unwrap_or(false),
                "CYCLONE_DISABLE_SYNTAX_CHECK" => self.cyclone.disable_syntax_check = value.parse::<bool>().unwrap_or(false),
                "CYCLONE_SILENCE_MODE" => self.cyclone.silence_mode = value.parse::<bool>().unwrap_or(false),

                "LOGGER_CONSOLE" => self.logger.console = value.parse::<bool>().unwrap_or(false),
                "LOGGER_LEVEL" => self.logger.level = value,
                "LOGGER_DIRECTORY" => self.logger.directory = value,
                "LOGGER_ROTATION" => self.logger.rotation = value,
                "LOGGER_FILE_PREFIX" => self.logger.file_prefix = value,
                "LOGGER_NO_COLOR" => self.logger.no_color = value.parse::<bool>().unwrap_or(false),

                // "LOGGER_EXECUTION_CONSOLE" => self.logger.execution.console = value.parse::<bool>().unwrap_or(false),
                // "LOGGER_EXECUTION_LEVEL" => self.logger.execution.level = value,

                _ => println!("WARN: Unknown or not supported config key: {}", key),
            }
        }
    }
    pub fn load_args(&mut self, args: &CliArgs) {
        if let Some(dir) = &args.working_directory {
            self.cyclone.source_path = dir.clone();
        }

        if let Some(level) = &args.log_level {
            self.logger.level = level.clone();
        }

        if let Some(host) = &args.host {
            self.server.host = host.clone();
        }

        if let Some(port) = &args.port {
            self.server.port = port.clone();
        }

        if let Some(timeout) = &args.timeout {
            self.cyclone.mandatory_timeout_ms = timeout.clone()
        }

        if args.no_syntax_check {
            self.cyclone.disable_syntax_check = true
        }

        if args.silence {
            self.cyclone.silence_mode = true
        }

        if args.no_clear_files {
            self.cyclone.delete_after_exec = false
        }

        if args.no_color {
            self.logger.no_color = true
        }

        if let Some(path) = args.cyclone_instance.first() {
            self.cyclone.path = path.clone()
        }
    }

    pub fn prepare_cyclone_env(&self) -> PathBuf {
        let executable = locate_cyclone_entry(
            if self.cyclone.path.is_empty() { None }
            else {Some(&self.cyclone.path)},
            !self.cyclone.silence_mode,
            if self.cyclone.executable.is_empty() { DEFAULT_CYCLONE_FILE } else { &self.cyclone.executable }
        ).expect("Failed to locate a valid Cyclone instance");

        let executable_absolute = dunce::canonicalize(&executable)
            .expect(&format!("Executable path does not exist: {:?}", &executable));

        if self.cyclone.append_env_path {
            prepare_cyclone_env(&executable_absolute)
                .expect(&format!("Failed to set env variable for Cyclone executable: {:?}", executable_absolute));
        }

        if !self.cyclone.source_path.is_empty() {
            env::set_current_dir(&self.cyclone.source_path)
                .expect(&format!("Failed to set working directory to: {}", &self.cyclone.source_path));
            // info!("Source code execution directory being set to: {}", &self.cyclone.source_path)
        }

        executable_absolute
    }
}