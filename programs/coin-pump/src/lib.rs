use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
    token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount, Transfer, transfer},
};
use mpl_token_metadata::{pda::find_metadata_account, state::DataV2};
declare_id!("3913THeK4R5cU8SSpKgEhzeSfxuLta9yWTqWU758vuii");

#[program]
pub mod smart_contract {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn launch_token(ctx: Context<LaunchToken>, tokenName : String, tokenSymbol : String, uri : String) -> Result<()> {
        let cpi_account: MintTo = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.reserve_account.to_account_info(),
            authority: ctx.accounts.platform_account.to_account_info()
        };

        let seeds: &[u8; 6] = b"reward";
        let bump: u8 = *ctx.bumps.get("reward_token_mint").unwrap();
        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

        let data_v2 = DataV2 {
            name: tokenName,
            symbol: tokenName,
            uri: uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let cpi_context: CpiContext<MintTo> = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            cpi_account,
            signer
        );

        mint_to(cpi_context, 1000000000);

        let cpi_transfer_account: Transfer = Transfer {
            from: ctx.accounts.curve_account.to_account_info(),
            to: ctx.accounts.curve_account.to_account_info(),
            authority: ctx.accounts.platform_account.to_account_info()
        };

        let cpi_transfer_context: CpiContext<Transfer> = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_transfer_account);
        //transfer to curve account 80 million
        transfer(cpi_transfer_context, 800000000);
        
        // Update TokenLaunch account
        let token_launch = &mut ctx.accounts.token_launch;
        token_launch.reserve_account = ctx.accounts.reserve_account.key();
        token_launch.mint_account = ctx.accounts.mint_account.key();
        token_launch.curve_account = ctx.accounts.curve_account.key();
        token_launch.platform_account = ctx.accounts.platform_account.key();
        token_launch.creator_account = ctx.accounts.creator_account.key();
        token_launch.reserve_amount = 1000000000; // example value
        token_launch.curve_amount = 800000000; // example value
        token_launch.tokens_sold = 0; // initial value
        token_launch.sol_liquidty = 0; // initial value

        Ok(())
    }


    pub fn buy(ctx: Context<Initialize>) -> Result<()> {
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


    pub token_launch: Account<'info, TokenLaunch>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(init, payer = creator_account, space = 8 + 32 * 5 + 8 * 4)]
    pub token_launch: Account<'info, TokenLaunch>,

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

#[account]
pub struct Buys {
    pub buyer: Pubkey,
    pub curve_account: Pubkey,
    pub reserve_account: Pubkey,

}
