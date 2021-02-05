use crate::schema::{JudgeConfig, JudgeResult};
use async_trait::async_trait;
use concurrent_queue::ConcurrentQueue;
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicNackOptions, BasicRejectOptions},
    types::LongLongUInt,
    Channel, ConsumerDelegate,
};
use log::{error, info};
use once_cell::sync::OnceCell;
use std_semaphore::Semaphore;

#[derive(Copy, Clone)]
pub struct Worker<T: PlatformWorker + Sync + Send + Copy + Clone> {
    pub id: i32,
    queue: &'static OnceCell<
        ConcurrentQueue<(
            /* channel */ Channel,
            /* delivery tag */ LongLongUInt,
            /* judge config */ JudgeConfig,
        )>,
    >,
    semaphore: &'static OnceCell<Semaphore>,
    platform_worker: T,
}

impl<T: PlatformWorker + Sync + Send + Copy + Clone> Worker<T> {
    pub fn new(
        id: i32,
        queue: &'static OnceCell<
            ConcurrentQueue<(
                /* channel */ Channel,
                /* delivery tag */ LongLongUInt,
                /* judge config */ JudgeConfig,
            )>,
        >,
        semaphore: &'static OnceCell<Semaphore>,
        platform_worker: T,
    ) -> Self {
        Self {
            id,
            queue,
            semaphore,
            platform_worker,
        }
    }

    pub async fn worker_thread(&self) {
        info!("worker {} started.", self.id);
        let queue = self.queue.get().unwrap();
        loop {
            self.semaphore.get().unwrap().acquire();
            while !queue.is_empty() {
                let item = queue.pop();
                if let Ok(result) = item {
                    let (channel, delivery_tag, config) = result;
                    match self.platform_worker.judge(&config).await {
                        Ok(result) => {
                            info!("{}", result);
                            channel
                                .basic_ack(delivery_tag, BasicAckOptions::default())
                                .await
                                .unwrap();
                        }
                        Err(err) => {
                            error!("worker {}: an error occurred while processing judge request #{}: {}", self.id, config.id, err);
                            channel
                                .basic_nack(delivery_tag, BasicNackOptions::default())
                                .await
                                .unwrap();
                        }
                    };
                }
            }
        }
    }
}

impl<T: PlatformWorker + Sync + Send + Copy + Clone> ConsumerDelegate for Worker<T> {
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        info!("worker {}: received judge request.", self.id);
        if let Ok(delivery) = delivery {
            if let Some((channel, delivery)) = delivery {
                if let Ok(s) = std::str::from_utf8(&delivery.data) {
                    if let Ok(config) = serde_json::from_str::<JudgeConfig>(s) {
                        info!("worker {}: accepted judge request #{}.", self.id, config.id);
                        let _ = self.queue.get().unwrap().push((
                            channel,
                            delivery.delivery_tag,
                            config,
                        ));
                        self.semaphore.get().unwrap().release();
                        return Box::pin(async {});
                    }
                }

                error!("worker {}: malformed judge request.", self.id);
                return Box::pin(async move {
                    channel
                        .basic_reject(delivery.delivery_tag, BasicRejectOptions::default())
                        .await
                        .unwrap();
                    ()
                });
            }
        }

        error!("worker {}: failed to consume message.", self.id);
        Box::pin(async {})
    }
}

#[async_trait]
pub trait PlatformWorker {
    async fn judge(&self, config: &JudgeConfig) -> Result<JudgeResult, String>;
}
