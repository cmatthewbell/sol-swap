use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("2cESwGJN1TtkYENEYqQFJNAjDnkyhHjCUUeRmibP8RuP");

#[program]
pub mod sol_swap {
    use super::*;

    // creates swap with SOL and wants SPL
    pub fn create_swap_from_sol(ctx: Context<CreateSwapFromSol>, offered_amount: u64, wanted_asset: Asset) -> Result<()> {
        // Populate escrow account with account fields
        ctx.accounts.escrow.maker = ctx.accounts.maker.key();
        ctx.accounts.escrow.offered_asset = Asset::Sol {amount: offered_amount};
        ctx.accounts.escrow.wanted_asset = wanted_asset;
        ctx.accounts.escrow.bump = ctx.bumps.escrow;

        // transfer SOL from maker to escrow
        let from_pubkey = ctx.accounts.maker.to_account_info();
        let to_pubkey = ctx.accounts.escrow.to_account_info();
        let sys = ctx.accounts.system_program.to_account_info();

        let cpi_ctx = CpiContext::new(
            sys,
            system_program::Transfer {
                from: from_pubkey,
                to: to_pubkey,
            }
        );

        system_program::transfer(cpi_ctx, offered_amount)?;
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