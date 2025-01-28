use super::PumpBot;
use crate::{constants::*, models::PumpProgram};
use anyhow::{Result, Context};
use anchor_client::solana_sdk::{
    system_program,
    instruction::{AccountMeta, Instruction},
    transaction::Transaction,
};
use solana_sdk::{pubkey::Pubkey, signature::Signer};
use anchor_lang::Id;
use log::{info, debug};

impl PumpBot {
    pub async fn buy(&self, mint: Pubkey, amount: u64, slippage_bps: u64) -> Result<()> {
        info!("Initiating buy order for {} tokens of mint: {}", amount, mint);
        let expected_sol = self.calculate_buy_price(&mint, amount).await?;
        let max_sol_cost = expected_sol + (expected_sol * slippage_bps / 10000);
        info!("Max SOL cost with slippage: {} lamports", max_sol_cost);
        
        self.check_sol_balance(max_sol_cost).await?;

        let bonding_curve = self.get_bonding_curve_address(&mint);
        debug!("Derived bonding curve address: {}", bonding_curve);

        let associated_bonding_curve = Pubkey::find_program_address(
            &[b"associated_bonding_curve", &mint.to_bytes()],
            &PumpProgram::id(),
        ).0;

        let associated_user = Pubkey::find_program_address(
            &[b"associated_user", &self.payer.pubkey().to_bytes(), &mint.to_bytes()],
            &PumpProgram::id(),
        ).0;

        info!("Building transaction instruction");
        let accounts = vec![
            // ... (keep existing account setup)
        ];

        let discriminator = &anchor_lang::solana_program::hash::hash("global:buy".as_bytes()).to_bytes()[..8];
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&amount.to_le_bytes());
        data.extend_from_slice(&max_sol_cost.to_le_bytes());

        let instruction = Instruction {
            program_id: PumpProgram::id(),
            accounts,
            data,
        };

        self.execute_transaction(instruction).await
    }

    pub async fn execute_transaction(&self, instruction: Instruction) -> Result<()> {
        info!("Executing transaction");
        let payer = self.payer.clone();
        let rpc_client = self.rpc_client.clone();

        info!("Fetching recent blockhash");
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .await
            .context("Failed to get recent blockhash")?;

        let mut tx = Transaction::new_with_payer(
            &[instruction], 
            Some(&payer.pubkey())
        );
        
        info!("Signing transaction");
        tx.try_sign(&[payer.as_ref()], recent_blockhash)
            .context("Failed to sign transaction")?;

        info!("Sending transaction");
        let signature = rpc_client
            .send_and_confirm_transaction_with_spinner(&tx)
            .await
            .context("Failed to send and confirm transaction")?;

        info!("Transaction confirmed: https://solscan.io/tx/{}", signature);
        Ok(())
    }
}