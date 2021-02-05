use super::worker::PlatformWorker;
use crate::{schema::JudgeConfig, JudgeResult};
use async_trait::async_trait;
use log::info;

#[derive(Clone, Copy)]
pub struct LinuxWorker {}

impl LinuxWorker {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl PlatformWorker for LinuxWorker {
    async fn judge(&self, config: &JudgeConfig) -> Result<JudgeResult, String> {
        info!("{}", config);

        Err("not implemented.".to_string())
    }
}
