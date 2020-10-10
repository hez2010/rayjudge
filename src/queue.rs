use async_amqp::*;
use async_trait::async_trait;
use lapin::{Channel, Connection, ConnectionProperties, Result, options::BasicConsumeOptions, options::BasicPublishOptions, options::QueueBindOptions, types::FieldTable};
use once_cell::sync::OnceCell;

type Receiver = fn(&str) -> Result<&str>;

pub struct Queue {
    url: String,
    queue: String,
    exchange: String,
    routing_key: String,
    channel: OnceCell<Channel>,
    callbacks: Vec<Receiver>,
    consumer_tag: String,
}

#[async_trait]
pub trait QueueSubscriber {
    fn subscribe(&mut self, callback: Receiver) -> Result<()>;
    async fn consume(&self);
}

#[async_trait]
pub trait QueuePublisher {
    async fn publish(&self, message: &str) -> Result<()>;
}

#[async_trait]
impl QueuePublisher for Queue {
    async fn publish(&self, message: &str) -> Result<()> {
        let payload = message.as_bytes().to_vec();
        self.channel.get().unwrap().basic_publish(&self.exchange, &self.routing_key, BasicPublishOptions::default(), payload, Default::default()).await?;

        Ok(())
    }
}

#[async_trait]
impl QueueSubscriber for Queue {
    fn subscribe(&mut self, callback: Receiver) -> Result<()> {
        self.callbacks.insert(0, callback);
        Ok(())
    }

    async fn consume(&self) {
        let channel = self.channel.get().unwrap();
        channel.basic_consume(&self.queue, self.consumer_tag, BasicConsumeOptions::default(), FieldTable::default()).await?;
        
    }
}

impl Queue {
    pub fn new(url: String, queue: String, exchange: String, routing_key: String) -> Self {
        Self {
            url,
            queue,
            exchange,
            routing_key,
            channel: OnceCell::new(),
            callbacks: Vec::new(),
            consumer_tag: "".to_string()
        }
    }


    pub async fn connect(&self) -> Result<()> {
        let conn = Connection::connect(
            self.url.as_str(),
            ConnectionProperties::default().with_async_std(),
        )
        .await?;

        let channel = conn.create_channel().await?;
        channel
            .queue_bind(
                self.queue.as_str(),
                self.exchange.as_str(),
                self.routing_key.as_str(),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel.set(channel).unwrap();

        Ok(())
    }
}
