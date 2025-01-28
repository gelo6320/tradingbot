// [file name]: cli/mod.rs
pub mod handlers;
use crate::pump_bot::PumpBot;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select, Input, console::Style};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn run(bot: &PumpBot) -> Result<()> {
    let theme = ColorfulTheme::default();
    let banner_style = Style::new().cyan().bold();
    
    loop {
        println!("{}", banner_style.apply_to("=== Pump.fun Trading Bot ==="));
        
        let action = Select::with_theme(&theme)
            .with_prompt("Choose action")
            .items(&["Buy Tokens", "Exit"]) // Removed "Sell Tokens"
            .default(0)
            .interact()?;

        if action == 1 { // Adjusted index since "Exit" is now second item
            println!("Exiting...");
            break;
        }

        let mint_address: String = Input::with_theme(&theme)
            .with_prompt("Enter token mint address")
            .interact_text()?;

        let mint = match Pubkey::from_str(&mint_address) {
            Ok(pk) => pk,
            Err(e) => {
                println!("❌ Invalid mint address format: {}", e);
                println!("💡 Tip: Use a valid base58-encoded public key");
                continue;
            }
        };

        let slippage: f64 = match Input::with_theme(&theme)
            .with_prompt("Enter slippage percentage (0.1-100)")
            .validate_with(|input: &f64| {
                if (0.1..=100.0).contains(input) {
                    Ok(())
                } else {
                    Err("Slippage must be between 0.1% and 100%")
                }
            })
            .default(1.0)
            .interact_text()
        {
            Ok(s) => s,
            Err(_) => continue,
        };
        
        let slippage_bps = (slippage * 100.0) as u64;

        match action {
            0 => handlers::handle_buy(bot, mint, slippage_bps).await?,
            _ => unreachable!()
        }
    }
    Ok(())
}