use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, CloseAccount};

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

    pub fn create_swap_from_token(ctx: Context<CreateSwapFromToken>, offered_amount: u64, wanted_asset: Asset) -> Result<()> {
        ctx.accounts.escrow.maker = ctx.accounts.maker.key();
        ctx.accounts.escrow.wanted_asset = wanted_asset;
        ctx.accounts.escrow.offered_asset = Asset::Token {
            mint: ctx.accounts.mint.key(),
            amount: offered_amount
        };
        ctx.accounts.escrow.bump = ctx.bumps.escrow;

        let tkn_program = ctx.accounts.token_program.to_account_info();
        let from_pubkey = ctx.accounts.maker_token_account.to_account_info();
        let to_pubkey = ctx.accounts.escrow_token_account.to_account_info();

        let cpi_ctx = CpiContext::new(
            tkn_program,
            token::Transfer {
                from: from_pubkey,
                to: to_pubkey,
                authority: ctx.accounts.maker.to_account_info()
            }
        );

        token::transfer(cpi_ctx, offered_amount)?;
        Ok(())
    }

    // cancel swap
    pub fn cancel_swap(ctx: Context<CancelSwap>) -> Result<()> {
        match &ctx.accounts.escrow.offered_asset {
            Asset::Token {mint: _, amount} => {
                // transfer tokens from escrow to maker
                let escrow_token_acct = ctx.accounts.escrow_token_account
                    .as_ref()
                    .ok_or(ErrorCode::MissingTokenAccount)?;
                let maker_token_acct = ctx.accounts.maker_token_account
                    .as_ref()
                    .ok_or(ErrorCode::MissingTokenAccount)?;
                let token_program_acct = ctx.accounts.token_program
                    .as_ref()
                    .ok_or(ErrorCode::MissingTokenAccount)?;
                let cpi_program = token_program_acct.to_account_info();
                let bump = ctx.accounts.escrow.bump;
                let maker_key = ctx.accounts.maker.key();
                let seeds = &[b"escrow".as_ref(), maker_key.as_ref(), &[bump]];
                let signer_seeds = &[&seeds[..]];
                
                let token_ctx = CpiContext::new_with_signer(
                    cpi_program,
                    token::Transfer {
                        from: escrow_token_acct.to_account_info(),
                        to: maker_token_acct.to_account_info(),
                        authority: ctx.accounts.escrow.to_account_info(),
                    },
                    signer_seeds
                );
                token::transfer(token_ctx, *amount)?;

                let close_ctx = CpiContext::new_with_signer(
                    token_program_acct.to_account_info(),
                    token::CloseAccount {
                        account: escrow_token_acct.to_account_info(),
                        destination: ctx.accounts.maker.to_account_info(),
                        authority: ctx.accounts.escrow.to_account_info(),
                    },
                    signer_seeds
                );
                token::close_account(close_ctx)?;
            },
            Asset::Sol { .. } => {

            }
        }

        Ok(())
    }
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

#[derive(Accounts)]
pub struct CreateSwapFromToken<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        init,
        payer = maker,
        token::mint = mint,
        token::authority = escrow,
        seeds = [b"escrow_token", escrow.key().as_ref()],
        bump,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = maker,
    )]
    pub maker_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>
}

#[derive(Accounts)]
pub struct CancelSwap<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref()],
        has_one = maker,
        close = maker,
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    pub token_program: Option<Program<'info, Token>>,
    #[account(
        mut,
        token::authority = maker
    )]
    pub maker_token_account: Option<Account<'info, TokenAccount>>,
    #[account(
        mut,
        token::authority = escrow,
        seeds = [b"escrow_token", escrow.key().as_ref()],
        bump
    )]
    pub escrow_token_account: Option<Account<'info, TokenAccount>>,
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

#[error_code]
pub enum ErrorCode {
    #[msg("Account is missing")]
    MissingTokenAccount
}