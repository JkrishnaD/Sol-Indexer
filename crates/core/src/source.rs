use std::sync::mpsc::Receiver;

use anyhow::Result;
use async_trait::async_trait;

use crate::{AccountUpdate, BlockUpdate, TransactionUpdate};

#[async_trait]
pub trait Source: Send + Sync {
    fn stream_block(&mut self) -> Result<Receiver<BlockUpdate>>;
    fn stream_transactions(&mut self) -> Result<Receiver<TransactionUpdate>>;
    fn stream_accounts(&mut self) -> Result<Receiver<AccountUpdate>>;
}

#[async_trait]
pub trait Destination: Send + Sync {
    async fn write_block(&mut self, block: BlockUpdate) -> Result<()>;
    async fn write_transaction(&mut self, transaction: TransactionUpdate) -> Result<()>;
    async fn write_account(&mut self, account: AccountUpdate) -> Result<()>;
}
