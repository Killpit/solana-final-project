use anchor_lang::prelude::*;

declare_id!("CniABA8LEh4AKTL1fZNMPEyDqYnzAA2Y69ScsnmdXcxb");

#[program]
mod final_project {
    use super::*;

    pub fn create(ctx: Context<Create>) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub count: u64,
}