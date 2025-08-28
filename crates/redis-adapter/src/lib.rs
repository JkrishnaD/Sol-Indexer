use anyhow::Result;
use futures::StreamExt;
use redis::{Client, RedisError};

// A trait for publishing messages to a channel
#[async_trait::async_trait]
pub trait Publisher: Send + Sync {
    fn publisher(&self, channel: &str, payload: &[u8]) -> Result<(), anyhow::Error>;
}

#[async_trait::async_trait]
pub trait Consumer: Send + Sync {
    fn consumer(&self, channel: &str) -> Result<(), anyhow::Error>;
}

pub struct RedisPublisher {
    pub client: Client,
}

pub struct RedisConsumer {
    pub client: Client,
}

impl RedisPublisher {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let client = Client::open(rpc_url)?;
        Ok(Self { client })
    }
}

impl RedisConsumer {
    pub async fn consume<F>(&self, channel: &str, mut handler: F) -> Result<(), RedisError>
    where
        F: FnMut(String) -> Result<()> + Send + 'static,
    {
        let mut conn = self.client.get_async_pubsub().await.unwrap();

        conn.subscribe(channel).await.unwrap();
        let mut msg_stream = conn.on_message();

        while let Some(msg) = msg_stream.next().await {
            let payload: String = msg.get_payload()?;

            if let Err(e) = handler(payload) {
                eprintln!("Error handling message: {}", e);
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Publisher for RedisPublisher {
    fn publisher(&self, channel: &str, payload: &[u8]) -> Result<(), anyhow::Error> {
        // getting a client connection
        let mut connection = self.client.get_connection()?;
        // converting the bytes which we get from the geyser into string
        let payload_str = String::from_utf8(payload.to_vec());

        // publishing the message which is the data to the respective channel
        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(payload_str.unwrap())
            .exec(&mut connection)?;
        Ok(())
    }
}
