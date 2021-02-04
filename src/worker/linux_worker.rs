use crate::{schema::JudgeConfig, JudgeResult};

#[cfg(target_os = "linux")]
pub async fn judge(worker: &Worker, config: &JudgeConfig) -> Result<JudgeResult, String> {
    println!("{}", config);

    Err(format!("error from worker {}: not implemented.", worker.id).to_string())
}
