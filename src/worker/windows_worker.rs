use crate::{schema::JudgeConfig, JudgeResult};

use super::Worker;

#[cfg(target_os = "windows")]
pub async fn judge(worker: &Worker, config: &JudgeConfig) -> Result<JudgeResult, String> {
    println!("{}", config);

    Err(format!("error from worker {}: not implemented.", worker.id).to_string())
}
