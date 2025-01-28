use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use crate::constants::PUMP_PROGRAM;

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
}

pub struct PumpProgram;

impl anchor_lang::Id for PumpProgram {
    fn id() -> Pubkey {
        PUMP_PROGRAM.parse().unwrap()
    }
}