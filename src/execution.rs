use std::io::ErrorKind;
use serde::Serialize;
use tracing::{debug, error, info, warn};

#[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct ExecutionOutput {
    pub result: String,
    pub trace: Option<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum ExecutionStatus {
    Executed,
    SpawnTimeout,
    FailedToSpawn
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub id: String,
    pub file_program: String,
    pub file_trace: Option<String>,
    pub output: Option<ExecutionOutput>,
    pub status: ExecutionStatus,
}

impl ExecutionResult {
    pub async fn clear_files(&self) {
        debug!("Begin clearing files: {}", &self.id);
        let mut s = tokio::task::JoinSet::new();
        let file_program = self.file_program.clone();
        let file_trace = self.file_trace.clone();
        // Cyclone generates a huge SMT2 file temporarily, and remains on disk if process is killed
        let smt2_file = format!("{}.smt2", file_program);
        s.spawn(tokio::fs::remove_file(file_program));
        s.spawn(tokio::fs::remove_file(smt2_file));
        if let Some(file) = file_trace {
            s.spawn(tokio::fs::remove_file(file));
        }
        let results = s.join_all().await;
        for r in results {
            if let Err(e) = r {
                match e.kind() {
                    ErrorKind::NotFound => debug!("File not found while clearing: {}", e),
                    _ => warn!("Error clearing files: {}", e)
                }
            }
        }
    }
}