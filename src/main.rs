use deepseekbot::{cli, pump_bot::PumpBot};
use solana_sdk::signature::Keypair;
use dotenv::dotenv;
use std::env;
use env_logger::Env;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Starting Pump.fun trading bot");
    
    dotenv().ok();
    let rpc_url = "https://frequent-morning-dream.solana-mainnet.quiknode.pro/c11685824f0fc5739c1076868f18f569f3dc83ce";
    let keypair = Keypair::from_base58_string(
        &env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env")
    );
    
    let bot = PumpBot::new(rpc_url, keypair);
    cli::run(&bot).await
}