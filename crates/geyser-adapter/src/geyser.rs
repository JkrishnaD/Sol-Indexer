use core::{
    AccountUpdate, BlockUpdate, EntryUpdate, SlotUpdate, TransactionStatusUpdate, TransactionUpdate,
};

use anyhow::{Context, Result};
use futures::StreamExt;
use serde_json::to_string;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;

use crate::filter::Filters;

// Channels we publish to the redis
pub const CH_ACCOUNTS: &str = "accounts";
pub const CH_BLOCKS: &str = "blocks";
pub const CH_TRANSACTIONS: &str = "transactions";
pub const CH_SLOTS: &str = "slots";
pub const CH_ENTRIES: &str = "entries";
pub const CH_TRANSACTION_STATUS: &str = "transaction_status";

// A trait for publishing messages to a channel
#[async_trait::async_trait]
pub trait Publisher: Send + Sync {
    fn publisher(channel: &str, payload: &[u8]) -> Result<(), anyhow::Error>;
}

pub async fn run_geyser<P: Publisher>(
    rpc_url: &str,
    x_token: Option<String>,
    filters: &Filters,
) -> Result<()> {
    let request = filters.to_subscribe_request();

    // create the connection
    let mut builder = GeyserGrpcClient::build_from_shared(rpc_url.to_string())?;
    if let Some(t) = x_token {
        builder = builder.x_token(Some(t))?;
    }

    // connect to the geyser
    let mut client = builder.connect().await?;

    // subscribe to the geyser
    let (_tx, mut stream) = client
        .subscribe_with_request(Some(request))
        .await
        .expect("Failed to subscribe to the geyser");

    // processing the stream
    while let Some(msg) = stream.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error receiving message from geyser: {:?}", e);
                break;
            }
        };

        match msg.update_oneof {
            Some(UpdateOneof::Account(a)) => {
                let update = AccountUpdate::try_from(a)
                    .map_err(|e| anyhow::anyhow!(e))
                    .context("Failed to convert AccountUpdate")?;
                let data = to_string(&update.info)?;
                P::publisher(CH_ACCOUNTS, data.as_bytes())?;
            }
            Some(UpdateOneof::Transaction(tx)) => {
                let update = TransactionUpdate::try_from(tx)
                    .map_err(|e| anyhow::anyhow!(e))
                    .context("Failed to convert TransactionUpdate")?;
                let data = to_string(&update)?;
                P::publisher(CH_TRANSACTIONS, data.as_bytes())?;
            }
            Some(UpdateOneof::Slot(s)) => {
                let update = SlotUpdate::try_from(s)
                    .map_err(|e| anyhow::anyhow!(e))
                    .context("Failed to convert SlotUpdate")?;
                let data = to_string(&update)?;
                P::publisher(CH_SLOTS, data.as_bytes())?;
            }
            Some(UpdateOneof::Block(b)) => {
                let update = BlockUpdate::try_from(b)
                    .map_err(|e| anyhow::anyhow!(e))
                    .context("Failed to convert BlockUpdate")?;
                let data = to_string(&update)?;
                P::publisher(CH_BLOCKS, data.as_bytes())?;
            }
            Some(UpdateOneof::Entry(e)) => {
                let update = EntryUpdate::try_from(e)
                    .map_err(|e| anyhow::anyhow!(e))
                    .context("Failed to convert EntryUpdate")?;
                let data = to_string(&update)?;
                P::publisher(CH_ENTRIES, data.as_bytes())?;
            }
            Some(UpdateOneof::TransactionStatus(ts)) => {
                let update = TransactionStatusUpdate::try_from(ts)
                    .map_err(|e| anyhow::anyhow!(e))
                    .context("Failed to convert TransactionStatusUpdate")?;
                let data = to_string(&update)?;
                P::publisher(CH_TRANSACTION_STATUS, data.as_bytes())?;
            }
            Some(UpdateOneof::Ping(p)) => {
                // Handle Ping update
                eprintln!("Received Ping update: {:?}", p);
            }
            Some(UpdateOneof::Pong(pong)) => {
                // Handle Pong update
                eprintln!("Received Pong update: {:?}", pong);
            }
            Some(UpdateOneof::BlockMeta(block_meta)) => {
                // Handle BlockMeta update
                eprintln!("Received BlockMeta update: {:?}", block_meta);
            }
            None => {
                eprintln!("Received empty update from geyser");
            }
        }
    }
    Ok(())
}
