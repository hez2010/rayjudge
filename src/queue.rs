use async_amqp::*;
use async_trait::async_trait;
use lapin::{
    options::BasicConsumeOptions, options::BasicPublishOptions, options::ExchangeDeclareOptions,
    options::QueueBindOptions, options::QueueDeclareOptions, types::FieldTable, Channel,
    Connection, ConnectionProperties, ConsumerDelegate, ExchangeKind, Result,
};
use once_cell::sync::OnceCell;

pub struct Queue {
    url: String,
    queue: String,
    exchange: String,
    routing_key: String,
    connection: OnceCell<Connection>,
    channel: OnceCell<Channel>,
    consumer_tag: String,
}

#[async_trait]
pub trait QueueSubscriber {
    async fn subscribe<D: ConsumerDelegate + 'static>(&mut self, callback: D) -> Result<()>;
}

#[async_trait]
pub trait QueuePublisher {
    async fn declare(&self) -> Result<()>;
    async fn publish(&self, message: &str) -> Result<()>;
}

#[async_trait]
impl QueuePublisher for Queue {
    async fn publish(&self, message: &str) -> Result<()> {
        let payload = message.as_bytes().to_vec();
        self.channel
            .get()
            .unwrap()
            .basic_publish(
                &self.exchange,
                &self.routing_key,
                BasicPublishOptions::default(),
                payload,
                Default::default(),
            )
            .await?;

        Ok(())
    }

    async fn declare(&self) -> Result<()> {
        let channel = self.connection.get().unwrap().create_channel().await?;
        self.channel.set(channel).unwrap();

        self.channel
            .get()
            .unwrap()
            .queue_declare(
                &self.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .get()
            .unwrap()
            .exchange_declare(
                &self.exchange,
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        self.channel
            .get()
            .unwrap()
            .queue_bind(
                &self.queue,
                &self.exchange,
                &self.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }
}

#[async_trait]
impl QueueSubscriber for Queue {
    async fn subscribe<D: ConsumerDelegate + 'static>(&mut self, callback: D) -> Result<()> {
        let consumer = self
            .channel
            .get()
            .unwrap()
            .basic_consume(
                &self.queue,
                self.consumer_tag.as_str(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        consumer.set_delegate(callback)?;

        Ok(())
    }
}

impl Queue {
    pub fn new(url: String, queue: String, exchange: String, routing_key: String) -> Self {
        Self {
            url,
            queue,
            exchange,
            routing_key,
            connection: OnceCell::new(),
            channel: OnceCell::new(),
            consumer_tag: "".to_string(),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        self.connection
            .set(
                Connection::connect(
                    self.url.as_str(),
                    ConnectionProperties::default().with_async_std(),
                )
                .await?,
            )
            .unwrap();

        Ok(())
    }
}
