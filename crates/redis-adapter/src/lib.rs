use anyhow::Result;
use redis::Client;

// A trait for publishing messages to a channel
#[async_trait::async_trait]
pub trait Publisher: Send + Sync {
    fn publisher(&self, channel: &str, payload: &[u8]) -> Result<(), anyhow::Error>;
}

pub struct RedisPublisher {
    pub client: Client,
}

impl RedisPublisher {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let client = Client::open(rpc_url)?;
        Ok(Self { client })
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
