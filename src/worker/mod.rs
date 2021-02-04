#[cfg(target_os = "linux")]
pub mod linux_worker;
#[cfg(target_os = "windows")]
pub mod windows_worker;

use concurrent_queue::ConcurrentQueue;
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicNackOptions, BasicRejectOptions},
    types::LongLongUInt,
    Channel, ConsumerDelegate,
};
use once_cell::sync::OnceCell;
use std_semaphore::Semaphore;

use crate::schema::JudgeConfig;

#[derive(Copy, Clone)]
pub struct Worker {
    pub id: i32,
    queue: &'static OnceCell<
        ConcurrentQueue<(
            /* channel */ Channel,
            /* delivery tag */ LongLongUInt,
            /* judge config */ JudgeConfig,
        )>,
    >,
    semaphore: &'static OnceCell<Semaphore>,
}

impl Worker {
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
    ) -> Self {
        Self { id, queue, semaphore }
    }

    pub async fn worker_thread(&self) {
        println!("worker {} started.", self.id);
        let queue = self.queue.get().unwrap();
        loop {
            self.semaphore.get().unwrap().acquire();
            while !queue.is_empty() {
                let item = queue.pop();
                if let Ok(result) = item {
                    let (channel, delivery_tag, config) = result;
                    #[cfg(target_os = "windows")]
                    match windows_worker::judge(self, &config).await {
                        Ok(result) => {
                            println!("{}", result);
                            channel
                                .basic_ack(delivery_tag, BasicAckOptions::default())
                                .await
                                .unwrap();
                        }
                        Err(err) => {
                            println!("{}", err);
                            channel
                                .basic_nack(delivery_tag, BasicNackOptions::default())
                                .await
                                .unwrap();
                        }
                    };
                    #[cfg(target_os = "linux")]
                    match linux_worker::judge(self, &config).await {
                        Ok(result) => {
                            println!("{}", result);
                            channel
                                .basic_ack(delivery_tag, BasicAckOptions::default())
                                .await
                                .unwrap();
                        }
                        Err(err) => {
                            println!("{}", err);
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

impl ConsumerDelegate for Worker {
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        println!("worker {} accepted judge request.", self.id);
        if let Ok(delivery) = delivery {
            if let Some((channel, delivery)) = delivery {
                if let Ok(s) = std::str::from_utf8(&delivery.data) {
                    if let Ok(config) = serde_json::from_str::<JudgeConfig>(s) {
                        let _ = self.queue.get().unwrap().push((
                            channel,
                            delivery.delivery_tag,
                            config,
                        ));
                        self.semaphore.get().unwrap().release();
                        return Box::pin(async {});
                    }
                }

                return Box::pin(async move {
                    channel
                        .basic_reject(delivery.delivery_tag, BasicRejectOptions::default())
                        .await
                        .unwrap();
                    ()
                });
            }
        }

        Box::pin(async {})
    }
}
