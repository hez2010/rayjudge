mod queue;
mod schema;

use std::{time::Duration, str};
use lapin::{message::Delivery, options::BasicAckOptions, Channel, Error};
use queue::{Queue, QueuePublisher, QueueSubscriber};
use schema::{JudgeConfig, Program};

struct JudgeResult {
    status: String
}

async fn judge_submission(config: &JudgeConfig) -> Result<JudgeResult, &str> {
    Err(&config.r#type)
}

#[async_std::main]
async fn main() {
    env_logger::init();

    let config = JudgeConfig {
        version: "v5".to_string(),
        r#type: "programming".to_string(),
        stages: Vec::new(),
        program: Program {
            language: "csharp".to_string(),
            compile_args: Vec::new(),
            sources: Vec::new(),
            git_repo_name: None,
            entry_point: None,
        },
        random_generator: None,
        custom_comparator: None,
        testcases: Vec::new(),
    };
    let mut mq = Queue::new(
        "amqp://localhost:5672".to_string(),
        "mytest".to_string(),
        "mytest".to_string(),
        "".to_string(),
    );
    mq.connect().await.unwrap();
    mq.declare().await.unwrap();

    mq.subscribe(|d: Result<Option<(Channel, Delivery)>, Error>| async move {
        let (channel, delivery) = d.unwrap().expect("error in consumer");
        println!("{}", str::from_utf8(&delivery.data).unwrap());
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .expect("ack");
    })
    .await
    .unwrap();
    let json = serde_json::to_string_pretty(&config).unwrap();
    mq.publish(json.as_str()).await.unwrap();
    async_std::task::block_on(async {
        let result = judge_submission(&config).await;
        match result {
            Ok(r) => println!("{}", &r.status),
            Err(msg) => println!("Error with {}", msg)
        }
    });
    async_std::task::sleep(Duration::from_secs(1)).await
}
