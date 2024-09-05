use std::sync::Arc;
use serde::Deserialize;
use tracing::info;
use crate::cyclone::{is_valid_program, ValidationResult};
use crate::environment::{Environment, EnvironmentSummary};
use crate::execution::{ExecutionOutput, ExecutionStatus};
use crate::response::{RESP_CODE_EXECUTION_TIMEOUT, RESP_CODE_INVALID_OPTIONS, RESP_CODE_SYNTAX_ERROR, RESP_CODE_UNSUCCESSFUL_EXECUTION, ResponseOf};

const CENSOR_REPLACEMENT: &str = "<censored-path>";

#[derive(Debug, Deserialize)]
pub struct ExecutionRequest {
    pub program: String
}

pub fn handle_server_info(env: &Arc<Environment>) -> ResponseOf<EnvironmentSummary> {
    ResponseOf::success(env.summary.clone())
}

pub async fn handle_exec(req: &ExecutionRequest, env: &Arc<Environment>) -> ResponseOf<ExecutionOutput> {
    info!("Request at /exec");
    let disabled_options = &env.config.cyclone.disabled_options;
    let program = &req.program;
    match {
        if env.config.cyclone.disable_syntax_check { ValidationResult::Valid }
        else { is_valid_program(program, disabled_options) }
    } {
        ValidationResult::InvalidOption => ResponseOf::non_success(RESP_CODE_INVALID_OPTIONS),
        ValidationResult::SyntaxError => ResponseOf::non_success(RESP_CODE_SYNTAX_ERROR),
        ValidationResult::Valid => {
            if let Some(result) = env.perform_execution(program).await {
                if let Some(mut out) = result.output {
                    if env.config.cyclone.censor_system_paths {
                        out.result = env.censor_regex.replace_all(&out.result, CENSOR_REPLACEMENT).to_string();
                        if let Some(trace) = out.trace {
                            out.trace = Some(env.censor_regex.replace_all(&trace, CENSOR_REPLACEMENT).to_string())
                        }

                    }
                    ResponseOf::success(out)
                } else {
                    match result.status {
                        ExecutionStatus::SpawnTimeout => ResponseOf::non_success(RESP_CODE_EXECUTION_TIMEOUT),
                        _ => ResponseOf::non_success(RESP_CODE_UNSUCCESSFUL_EXECUTION)
                    }
                }
            } else {
                // failed to even write the program file
                ResponseOf::non_success(RESP_CODE_UNSUCCESSFUL_EXECUTION)
            }
        },
    }
}