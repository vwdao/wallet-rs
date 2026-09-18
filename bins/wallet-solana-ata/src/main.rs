mod ata;
mod config;
mod rpc;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/wallet-solana-ata.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: config::Config = wallet_config::load_yaml(&args.config)?;
    ata::run(&cfg).await
}