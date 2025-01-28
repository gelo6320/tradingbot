pub mod transactions;
pub mod calculations;

use std::sync::Arc;
use anchor_lang::Id;
use std::time::Duration;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::Keypair;
use solana_client::nonblocking::rpc_client::RpcClient;
use reqwest::Client;
use rand::Rng;
use anyhow::Result;
use crate::models::PumpProgram;

pub struct PumpBot {
    pub payer: Arc<Keypair>,
    pub rpc_client: Arc<RpcClient>,
    http_client: Client,
    user_agents: Vec<&'static str>,
}

impl PumpBot {
    pub fn new(rpc_url: &str, keypair: Keypair) -> Self {
        let payer = Arc::new(keypair);
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        ));
        
        let user_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Mobile/15E148 Safari/604.1"
        ];

        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(user_agents[rand::thread_rng().gen_range(0..user_agents.len())])
            .build()
            .unwrap();

        Self { payer, rpc_client, http_client, user_agents }
    }

    pub fn get_bonding_curve_address(&self, mint: &Pubkey) -> Pubkey {
        let seeds = &[
            b"bonding-curve".as_ref(),
            mint.as_ref(),
        ];
        let (address, _bump) = Pubkey::find_program_address(seeds, &PumpProgram::id());
        address
    }
}