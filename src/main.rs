mod config;
mod response;
mod cyclone;
mod environment;
mod execution;
mod parse;
mod cli;

use std::sync::Arc;
use std::time::Duration;
use poem::{get, handler, listener::TcpListener, web::{Json, Data}, Route, Server, middleware::AddData, EndpointExt, post};
use poem::http::Method;
use poem::listener::Listener;
use poem::middleware::Cors;
use serde::{Deserialize};
use tracing::{error, info, warn};
use crate::cli::CliArgs;
use crate::cyclone::{is_valid_program, ValidationResult};
use crate::environment::{Environment, EnvironmentSummary};
use crate::execution::{ExecutionOutput, ExecutionStatus};
use crate::response::{RESP_CODE_EXECUTION_TIMEOUT, RESP_CODE_INVALID_OPTIONS, RESP_CODE_SYNTAX_ERROR, RESP_CODE_UNSUCCESSFUL_EXECUTION, ResponseOf};

const CENSOR_REPLACEMENT: &str = "<censored-path>";
const EDITOR_URL: &str = "https://cyclone.cs.nuim.ie/editor";
const REPO_URL: &str = "https://github.com/lucid-brndmg/cyclone-execution-server-rs";

#[derive(Debug, Deserialize)]
struct ExecutionRequest {
    program: String
}

#[handler]
fn server_info(env: Data<&Arc<Environment>>) -> Json<ResponseOf<EnvironmentSummary>> {
    // format!("hello: {}", name)
    info!("Request at /");
    Json(ResponseOf::success(env.summary.clone()))
}

#[handler]
async fn exec(req: Json<ExecutionRequest>, env: Data<&Arc<Environment>>) -> Json<ResponseOf<ExecutionOutput>> {
    info!("Request at /exec");
    let disabled_options = &env.config.cyclone.disabled_options;
    let program = &req.program;
    match {
        if env.config.cyclone.disable_syntax_check { ValidationResult::Valid }
        else { is_valid_program(program, disabled_options) }
    } {
        ValidationResult::InvalidOption => Json(ResponseOf::non_success(RESP_CODE_INVALID_OPTIONS)),
        ValidationResult::SyntaxError => Json(ResponseOf::non_success(RESP_CODE_SYNTAX_ERROR)),
        ValidationResult::Valid => {
            if let Some(result) = env.perform_execution(program).await {
                if let Some(mut out) = result.output {
                    if env.config.cyclone.censor_system_paths {
                        out.result = env.censor_regex.replace_all(&out.result, CENSOR_REPLACEMENT).to_string();
                        if let Some(trace) = out.trace {
                            out.trace = Some(env.censor_regex.replace_all(&trace, CENSOR_REPLACEMENT).to_string())
                        }

                    }
                    Json(ResponseOf::success(out))
                } else {
                    match result.status {
                        ExecutionStatus::SpawnTimeout => Json(ResponseOf::non_success(RESP_CODE_EXECUTION_TIMEOUT)),
                        _ => Json(ResponseOf::non_success(RESP_CODE_UNSUCCESSFUL_EXECUTION))
                    }
                }
            } else {
                // failed to even write the program file
                Json(ResponseOf::non_success(RESP_CODE_UNSUCCESSFUL_EXECUTION))
            }
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("* Cyclone Online Editor's Execution Server *");
    println!("See {} for configurations", REPO_URL);

    let args = CliArgs::init();
    let env = Environment::initialize(&args);

    let host = env.config.server.host.clone();
    let mut port = env.config.server.port;
    let mut error = None;
    let mut begin_range = false;
    let max_port = 9999;
    let min_port = 2000;
    let acceptor = loop {
        if begin_range && port > max_port {
            error!("Failed to find a port to start a server.");
            return Err(error.unwrap());
        }
        let listener = TcpListener::bind(format!("{host}:{port}"));
        match listener.into_acceptor().await {
            Ok(a) => break a,
            Err(err) => error = Some(err),
        };
        // Most likely, another application is bound to this port.

        if begin_range {
            port += 1;
        } else {
            warn!("Cannot start server at {}:{} Retrying at a random port.", &host, port);
            begin_range = true;
            port = min_port;
        }
    };

    let timeout_fmt = if env.config.cyclone.mandatory_timeout_ms == 0 {
        "0 (NO TIMEOUT)".to_string()
    } else {
        let ms = env.config.cyclone.mandatory_timeout_ms;
        format!("{} ms ({} secs)", ms, Duration::from_millis(ms as u64).as_secs())
    };

    println!(
        "\nStarting Cyclone execution server at: http://{host}:{port}\nCyclone instance: {:?}\nWorking Directory: {:?}\nExecution Timeout: {}\nDisabled Options: {}\nVersion: {:?}\n\n{}{}{}",
        &env.cyclone_executable,
        &env.source_path,
        timeout_fmt,
        &env.config.cyclone.disabled_options.join(", "),
        &env.summary.version,
        if env.config.cyclone.disable_syntax_check { "! Syntax check is OFF\n" } else { "" },
        if env.config.cyclone.delete_after_exec { "" } else { "! Automatic file clearing is OFF\n" },
        if env.config.cyclone.censor_system_paths { "" } else { "! Censoring system file paths is OFF: make sure server is only accessible by TRUSTED devices\n" }
    );

    // since the url format is simple, there is no need to use a library to encode it
    let url = format!("{}?set_exec_server=http%3A%2F%2F{host}%3A{port}", EDITOR_URL);
    if !env.config.cyclone.silence_mode {
        match open::that_detached(&url) {
            Ok(_) => info!("Started a browser at {}", &url),
            Err(_) => error!("Cannot start a browser at {} , visit it manually to enable this server", &url)
        }
    } else {
        info!("Visit {} to automatically enable this server", &url)
    }

    let state = Arc::new(env);
    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false)
        .allow_origins_fn(|hd| true);
    let app = Route::new()
        .at("/", get(server_info))
        .at("/exec", post(exec))
        .with(cors)
        .with(AddData::new(state));

    Server::new_with_acceptor(acceptor).run(app).await?;
    Ok(())
}