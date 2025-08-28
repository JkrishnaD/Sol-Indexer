use anyhow::Result;
use config::CONFIG;
use filter::Filters;
use geyser::run_geyser;
use redis_adapter::RedisPublisher;
use rustls::crypto::{CryptoProvider, ring::default_provider};
mod filter;
mod geyser;

#[tokio::main]
async fn main() -> Result<()> {
    CryptoProvider::install_default(default_provider()).unwrap();
    println!("Starting geyser adapter...");
    let rpc_url = &CONFIG.rpc_url;
    let redis_url = &CONFIG.redis_url;
    let x_token = &CONFIG.x_token;

    let filters_path = "filters.json";

    let filters = Filters::from_file(filters_path)?;

    let publisher = RedisPublisher::new(&redis_url)?;
    run_geyser::<RedisPublisher>(rpc_url, x_token.clone(), &filters, publisher).await
    
}
