mod config;
mod monitor;
mod ntfy;
mod tankerkoenig;

use config::Config;
use dotenvy::dotenv;
use monitor::PriceMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = Config::from_env();

    let mut monitor = PriceMonitor::new(config)?;
    monitor.run().await;

    Ok(())
}
