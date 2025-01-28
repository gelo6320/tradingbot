use crate::{pump_bot::PumpBot, constants::LAMPORTS_PER_SOL};
use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme};
use solana_sdk::pubkey::Pubkey;
use log::{info, warn, error};

pub async fn handle_buy(bot: &PumpBot, mint: Pubkey, slippage_bps: u64) -> Result<()> {
    let theme = ColorfulTheme::default();
    
    info!("Starting buy workflow for mint: {}", mint);
    let amount: u64 = match Input::with_theme(&theme)
        .with_prompt("Enter token amount to buy")
        .validate_with(|input: &u64| {
            if *input > 0 {
                Ok(())
            } else {
                Err("Amount must be greater than 0")
            }
        })
        .interact_text()
    {
        Ok(a) => a,
        Err(_) => return Ok(()),
    };

    info!("User input: {} tokens", amount);
    match bot.calculate_buy_price(&mint, amount).await {
        Ok(expected_sol) => {
            let max_sol = expected_sol + (expected_sol * slippage_bps / 10000);
            
            println!(
                "💎 Expected cost: {:.4} SOL\n🚀 Max with slippage: {:.4} SOL",
                expected_sol as f64 / LAMPORTS_PER_SOL,
                max_sol as f64 / LAMPORTS_PER_SOL
            );

            info!("Attempting buy transaction");
            if let Err(e) = bot.buy(mint, amount, slippage_bps).await {
                error!("Transaction failed: {}", e);
                println!("❌ Transaction failed: {}", e);
                println!("💡 Check: https://pump.fun/{}", mint);
            } else {
                info!("Buy transaction completed successfully");
            }
        }
        Err(e) => {
            error!("Price calculation error: {}", e);
            println!("❌ Price calculation failed: {}", e);
            println!("💡 The token might not be tradeable on Pump.fun");
        }
    }

    Ok(())
}