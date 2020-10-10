use async_amqp::*;
use lapin::{Connection, ConnectionProperties, Result};

struct Listener {
    url: String,
    queue: String
}

impl Listener {
    fn new(url: String, queue: String) -> Self { Self { url, queue } }
    pub async fn connect(&self) -> Result<()> {
        let conn = Connection::connect(self.url.as_str(),
            ConnectionProperties::default().with_async_std()).await?;

        let channel = conn.create_channel().await?;
        
        return Ok(());
    }
}

