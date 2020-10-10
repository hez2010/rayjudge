mod queue;
mod schema;
use queue::{Queue, QueuePublisher, QueueSubscriber};
use schema::{JudgeConfig, Program};

#[async_std::main]
async fn main() {
    env_logger::init();

    let x = JudgeConfig {
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
    // mq2.subscribe( |d|{async {()}}).await.unwrap();
    mq.subscribe(|d| async move { println!("{:?}", d) })
        .await
        .unwrap();
    mq.publish("test").await.unwrap();
    println!("{}", x);
}
