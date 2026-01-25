use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("2cESwGJN1TtkYENEYqQFJNAjDnkyhHjCUUeRmibP8RuP");

#[program]
pub mod sol_swap {
    use super::*;

    // creates swap with SOL and wants SPL
    pub fn create_swap_from_sol(ctx: Context<CreateSwapFromSol>, offered_amount: u64, wanted_asset: Asset) -> Result<()> {
        // transfer SOL from maker to escrow
        // Populate escrow account with account fields
        Ok(())
    }

    // creates swap with SPL and wants SOL
}

#[derive(Accounts)]
pub struct CreateSwapFromSol<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub maker: Pubkey,
    pub offered_asset: Asset,
    pub wanted_asset: Asset,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
#[derive(InitSpace)]
pub enum Asset {
    Sol {
        amount: u64,
    },
    Token {
        mint: Pubkey,
        amount: u64,
    },
}