use anchor_lang::prelude::*;

declare_id!("2cESwGJN1TtkYENEYqQFJNAjDnkyhHjCUUeRmibP8RuP");

#[program]
pub mod sol_swap {
    use super::*;
}

#[account]
pub struct Escrow {
    maker: Pubkey,
    offered_asset: Asset,
    wanted_asset: Asset,
    bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
enum Asset {
    Sol {
        amount: u64,
    },
    Token {
        mint: Pubkey,
        amount: u64,
    },
}