mod schema;
mod queue;
use schema::{JudgeConfig,Program};
use queue::Queue;

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
    let mq = Queue::new("amqp://localhost:5672".to_string(), "test".to_string(), "test".to_string(), "".to_string());
    println!("conn");
    mq.connect().await.unwrap();
    println!("{}", x);
}
