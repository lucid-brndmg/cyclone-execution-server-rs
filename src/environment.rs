use std::env;
use std::path::{PathBuf};
use std::process::Stdio;
use std::string::ToString;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::cyclone::{fetch_cyclone_version};
use rand::distributions::Alphanumeric;
use rand::Rng;
use regex::Regex;
use serde::Serialize;
use tracing::{debug, error, info, trace, warn};
use crate::cli::CliArgs;
use crate::config::{CompiledExecutionLoggerPattern, Config};
use crate::execution::{ExecutionOutput, ExecutionResult, ExecutionStatus};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentSummary {
    pub version: String,
    pub disabled_options: Vec<String>,
    pub timeout: usize,
    pub is_queue_mode: bool,
    pub message: String
}

pub struct Environment {
    pub config: Config,
    pub cyclone_executable: PathBuf,
    pub summary: EnvironmentSummary,
    compiled_execution_logger_patterns: Vec<CompiledExecutionLoggerPattern>,

    pub source_path: PathBuf,
    pub censor_regex: Regex
}

const DEFAULT_MESSAGE: &str = "Cyclone execution server ready";
const DEFAULT_CONFIG_FILE: &str = "config.json";

impl Environment {
    pub fn initialize(args: &CliArgs) -> Self {
        let conf_location = match &args.config {
            Some(location) => location.as_str(),
            None => DEFAULT_CONFIG_FILE
        };
        let mut config = if let Ok(json) = std::fs::read_to_string(conf_location) {
            if let Some(config) = Config::from_json(&json) {
                println!("Configuration loaded from {}", conf_location);
                config
            } else {
                println!("WARN: Configuration file {} failed to parse, using default", conf_location);
                Config::default()
            }
        } else {
            println!("Configuration file not specified, using default");
            Config::default()
        };

        config.load_env();
        config.load_args(args);
        config.logger.initialize();

        let executable_absolute = config.prepare_cyclone_env();

        let (version, stderr) = fetch_cyclone_version(&executable_absolute).expect(&format!("Failed to fetch Cyclone version: {:?}", executable_absolute));
        if !stderr.is_empty() {
            warn!("Cyclone printed error at fetching Cyclone version {:?}: {:?}", version, stderr);
        }
        info!("Cyclone environment prepared with version: {:?}", version);
        let disabled_opts = config.cyclone.disabled_options.to_vec();
        let timeout = config.cyclone.mandatory_timeout_ms;

        let msg = if config.cyclone.censor_system_paths {DEFAULT_MESSAGE.to_string()} else { format!("{} using instance: {:?}", DEFAULT_MESSAGE, executable_absolute) };

        let ps = config.logger.compile_patterns();

        Self {
            config,
            // process_status: ProcessStatus::new(),
            cyclone_executable: executable_absolute,
            summary: EnvironmentSummary {
                version,
                disabled_options: disabled_opts,
                timeout,
                is_queue_mode: false,
                message: msg
            },
            compiled_execution_logger_patterns: ps,
            source_path: env::current_dir().expect("Unable to get current working directory"),
            censor_regex: Regex::new(r"([\/\\:][\w\-\\]+)+(.(cyclone|trace|jar)+?)").unwrap(),
        }
    }

    fn log_execution_result_keep_file(&self, out: &ExecutionOutput, filename: &str, program: &str) -> bool {
        if !self.compiled_execution_logger_patterns.is_empty() {
            let mut keep = false;
            let mut print_output = false;
            let mut print_input = false;
            let result = &out.result;
            let mut names = Vec::with_capacity(self.compiled_execution_logger_patterns.len());
            let mut level = tracing::Level::TRACE;
            for compiled_execution_logger_pattern in &self.compiled_execution_logger_patterns {
                if compiled_execution_logger_pattern.re.is_match(result) {
                    keep = keep
                        || compiled_execution_logger_pattern.keep_file;
                    print_input = print_input
                        || compiled_execution_logger_pattern.print_input;
                    print_output = print_output
                        || compiled_execution_logger_pattern.print_output;
                    let name = &compiled_execution_logger_pattern.name;
                    names.push(
                        if name.is_empty() { "#unnamed" }
                        else { name }
                    );
                    level = level.max(compiled_execution_logger_pattern.level)
                }
            }
            if !names.is_empty() {
                let message = format!(
                    "Spec {:?} matched {} patterns{}: {:?}{}{}",
                    filename,
                    names.len(),
                    if keep {" (file kept)"} else {""},
                    names.join(", "),
                    if print_input { format!("\ninput: {:?}", program) } else { "".to_string() },
                    if print_output { format!("\noutput: {:?}", &out.result) } else { "".to_string() }
                );
                // since event! only supports constant level...
                match level {
                    tracing::Level::TRACE => trace!(message),
                    tracing::Level::DEBUG => debug!(message),
                    tracing::Level::INFO => info!(message),
                    tracing::Level::WARN => warn!(message),
                    tracing::Level::ERROR => error!(message)
                }
                // event!(level, message)
            }

            keep
        } else { false }
    }

    pub async fn perform_execution(&self, program: &str) -> Option<ExecutionResult> {
        let id_length = self.config.cyclone.id_length;
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(id_length)
            .map(char::from)
            .collect();
        info!("Begin execution: {}", &id);
        let filename = format!("{}{}", id, &self.config.cyclone.extension);
        let mut file = tokio::fs::File::create(&filename).await.ok()?;
        file.write_all(program.as_ref()).await.ok()?;
        debug!("Execution file written: {}", &filename);
        let result = if let Ok(mut proc) = tokio::process::Command::new("java")
            .arg("-jar")
            .arg(&self.cyclone_executable)
            .arg(&filename)
            .arg("--no-color")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
            debug!("Process spawned. id = {}", &id);
            let trace_pattern = &self.config.cyclone.trace_keyword;
            let timeout_ms = self.config.cyclone.mandatory_timeout_ms;
            let result = match {
                if timeout_ms != 0 {
                    tokio::time::timeout(Duration::from_millis(timeout_ms as u64), proc.wait()).await
                } else {
                    // timeout not set, wait forever
                    Ok(proc.wait().await)
                }
            } {
                Err(_) => {
                    info!("Execution process killed due to timeout: {}", &id);
                    proc.kill().await.unwrap();
                    // self.process_status.decr_process();
                    Some(ExecutionResult {
                        output: None,
                        file_program: filename,
                        file_trace: None,
                        status: ExecutionStatus::SpawnTimeout,
                        id
                    })
                }
                Ok(_) => {
                    info!("Execution completed: {}", &id);
                    // self.process_status.decr_process();
                    let mut out_result = String::new();
                    // let mut stdout = String::new();
                    // let mut stderr = String::new();
                    if let Some(mut e) = proc.stderr {
                        e.read_to_string(&mut out_result).await.unwrap_or(0);
                    }

                    if let Some(mut o) = proc.stdout {
                        o.read_to_string(&mut out_result).await.unwrap_or(0);
                    }
                    debug!("Received output with length {}: {}", out_result.len(), &id);
                    let mut trace_file = None;
                    for line in out_result.lines() {
                        if line.starts_with(trace_pattern) {
                            trace_file = Some(line[trace_pattern.len()..].trim());
                            break
                        }
                    }
                    let trace = match trace_file {
                        Some(file) => tokio::fs::read_to_string(file).await.ok(),
                        None => None
                    };
                    Some(ExecutionResult {
                        id,
                        status: ExecutionStatus::Executed,
                        file_program: filename,
                        file_trace: match trace_file {
                            Some(s) => Some(String::from(s)),
                            None => None
                        },
                        output: Some(ExecutionOutput {
                            result: out_result,
                            trace
                        })
                    })
                }
            };
            result
        } else {
            info!("Process failed to spawn when executing: {}", &id);
            Some(ExecutionResult {
                id,
                status: ExecutionStatus::FailedToSpawn,
                file_program: filename,
                file_trace: None,
                output: None
            })
        };

        if let Some(ref out) = result {
            let force_keep_file = if let Some(output) = &out.output {
                self.log_execution_result_keep_file(output, &out.file_program, program)
            } else { false };

            let clear_files = !force_keep_file && self.config.cyclone.delete_after_exec;
            if clear_files { out.clear_files().await; }
        }

        result
    }
}