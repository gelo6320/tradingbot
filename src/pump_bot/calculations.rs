use super::PumpBot;
use crate::{constants::*, models::BondingCurve};
use anyhow::{Result, Context};
use solana_sdk::pubkey::Pubkey;
use anchor_lang::AnchorDeserialize;
use log::{info, debug, error};
use solana_sdk::signature::Signer;

impl PumpBot {
    pub async fn calculate_buy_price(&self, mint: &Pubkey, amount: u64) -> Result<u64> {
        info!("Calculating buy price for {} tokens of mint: {}", amount, mint);
        let state = self.get_bonding_curve_state(mint).await?;
        
        debug!("Bonding curve state: {:?}", state);
        let k = state.virtual_sol_reserves
            .checked_mul(state.virtual_token_reserves)
            .context("Arithmetic overflow in constant product calculation")?;
        
        let new_token_reserves = state.virtual_token_reserves
            .checked_sub(amount)
            .context("Insufficient token reserves in bonding curve")?;
        
        let expected_sol = state.virtual_sol_reserves
            .checked_sub(k.checked_div(new_token_reserves)
                .context("Division error in price calculation")?)
            .context("Subtraction error in price calculation")?;

        info!("Calculated expected SOL cost: {} lamports", expected_sol);
        Ok(expected_sol)
    }

    pub async fn check_sol_balance(&self, required_lamports: u64) -> Result<()> {
        let balance = self.rpc_client.get_balance(&self.payer.pubkey())
            .await
            .context("Failed to fetch SOL balance")?;
        
        debug!("Current SOL balance: {} lamports", balance);
        if balance < required_lamports {
            error!(
                "Insufficient SOL. Required: {:.4}, Available: {:.4}",
                required_lamports as f64 / LAMPORTS_PER_SOL,
                balance as f64 / LAMPORTS_PER_SOL
            );
            Err(anyhow::anyhow!("Insufficient SOL balance"))
        } else {
            info!("SOL balance sufficient for transaction");
            Ok(())
        }
    }

    pub async fn get_bonding_curve_state(&self, mint: &Pubkey) -> Result<BondingCurve> {
        info!("Fetching bonding curve state for mint: {}", mint);
        let bonding_curve = self.get_bonding_curve_address(mint);
        debug!("Bonding curve address: {}", bonding_curve);
        
        let account_data = self.rpc_client.get_account_data(&bonding_curve)
            .await
            .with_context(|| format!(
                "Bonding curve account not found\nMint: {}\nBonding Curve: {}",
                mint,
                bonding_curve
            ))?;

        BondingCurve::deserialize(&mut &account_data[8..])
            .context("Failed to deserialize bonding curve account")
    }
}