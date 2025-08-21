use std::sync::mpsc::Receiver;

use anyhow::Result;
use async_trait::async_trait;

use crate::BlockUpdate;

#[async_trait]
pub trait Source: Send + Sync {
    fn stream_block(&mut self) -> Result<Receiver<BlockUpdate>>;
}
