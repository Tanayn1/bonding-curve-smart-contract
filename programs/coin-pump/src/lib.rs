use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};




declare_id!("DM5JQKpEcekwf25n8LSATQu76nqfgG31xihWGDy978GY");

#[program]
pub mod coin_pump {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {        
        Ok(())
    }

    pub fn launch_token(ctx: Context<LaunchToken>) -> Result<()> {
        
        let cpi_accounts = token::MintTo {
            mint:ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.reserve_account.to_account_info(),
            authority: ctx.accounts.platform_account.to_account_info(),
        };

        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_context, 100);
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct LaunchToken<'info> {
    #[account(init, payer = creator_account, space = 8 + 32 * 5 + 8 * 4)]
    
    pub reserve_account: AccountInfo<'info>,
    pub mint_account: Account<'info, Mint>,
    pub curve_account: AccountInfo<'info>,
    pub platform_account: AccountInfo<'info>,
    #[account(mut)]
    pub creator_account: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

}

#[account]
pub struct TokenLaunch {
   pub reserve_account: Pubkey,
   pub mint_account: Pubkey,
   pub curve_account: Pubkey,
   pub platform_account: Pubkey,
   pub creator_account: Pubkey,
   pub reserve_amount: u64,
   pub curve_amount: u64,
   pub tokens_sold: u64,
   pub sol_liquidty: u64,
}
