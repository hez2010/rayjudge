mod queue;
mod schema;
mod worker;

use clap::Clap;
use concurrent_queue::ConcurrentQueue;
use lapin::{types::LongLongUInt, Channel};
use once_cell::sync::OnceCell;
use queue::{Queue, QueuePublisher, QueueSubscriber};
use schema::{JudgeConfig, JudgeResult, Program};
use std::{convert::TryInto, thread};
use std_semaphore::Semaphore;
use worker::Worker;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WORK_QUEUE: OnceCell<ConcurrentQueue<(
        /* channel */ Channel,
        /* delivery tag */ LongLongUInt,
        /* judge config */ JudgeConfig,
    )>> = OnceCell::new();

    static ref WORKER_SEMAPHORE: OnceCell<Semaphore> = OnceCell::new();
}

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "hez2010 <hez2010@outlook.com>, xfoxfu <i@xfox.me>, gw98 <i@yvettegwen.com>"
)]
struct Opts {
    /// The number of workers
    #[clap(short, long, default_value = "4")]
    worker: i32,
    /// The url of message queue
    #[clap(short, long, default_value = "amqp://localhost:5672")]
    url: String,
    /// The queue name of message queue
    #[clap(short, long, default_value = "Judge")]
    queue: String,
    /// The exchange name of message queue
    #[clap(short, long, default_value = "Judge")]
    exchange: String,
    /// The routing key of message queue
    #[clap(short, long, default_value = "")]
    routing_key: String,
}

const WORKER_COUNT: i32 = 4;

#[async_std::main]
async fn main() {
    let opts: Opts = Opts::parse();
    env_logger::init();

    let worker_queue: ConcurrentQueue<(
        /* channel */ Channel,
        /* delivery tag */ LongLongUInt,
        /* judge config */ JudgeConfig,
    )> = ConcurrentQueue::unbounded();

    if let Err(_) = WORK_QUEUE.set(worker_queue) {
        panic!("failed to set worker queue for once cell.");
    }

    let count: isize = opts.worker.try_into().unwrap();
    let worker_semaphore = Semaphore::new(count);

    if let Err(_) = WORKER_SEMAPHORE.set(worker_semaphore) {
        panic!("failed to set worker semaphore for once cell.");
    }

    println!("initing message queue.");

    let mut mq = Queue::new(
        opts.url,
        opts.queue,
        opts.exchange,
        opts.routing_key,
    );

    mq.connect().await.unwrap();
    mq.declare().await.unwrap();

    println!("starting judge workers.");

    let mut workers = Vec::new();

    for _ in 0..WORKER_COUNT {
        WORKER_SEMAPHORE.get().unwrap().acquire();
    }

    for i in 0..WORKER_COUNT {
        let worker = Worker::new(i, &WORK_QUEUE, &WORKER_SEMAPHORE);
        mq.subscribe(worker).await.unwrap();
        workers.push((
            i,
            thread::spawn(move || {
                async_std::task::block_on(worker.worker_thread());
            }),
        ));
    }

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

    let json = serde_json::to_string_pretty(&config).unwrap();
    mq.publish(json.as_str()).await.unwrap();

    for (id, handle) in workers {
        match handle.join() {
            Ok(_) => (),
            Err(_) => {
                println!("an error occurred in worker {}.", id);
            }
        }
    }
}
