use anchor_lang::prelude::*;

declare_id!("2cESwGJN1TtkYENEYqQFJNAjDnkyhHjCUUeRmibP8RuP");

#[program]
pub mod sol_swap {
    use super::*;

    pub fn create_swap() {

    }
}

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub offered_asset: Asset,
    pub wanted_asset: Asset,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Asset {
    Sol {
        amount: u64,
    },
    Token {
        mint: Pubkey,
        amount: u64,
    },
}