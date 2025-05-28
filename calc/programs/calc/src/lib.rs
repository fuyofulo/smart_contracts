use anchor_lang::prelude::*;

declare_id!("A6A2ZiorsHjBUQT2C5K4zEx9vZmVVHGs25hdmMaRmWbQ");

#[program]
pub mod calc {
    use super::*;

    pub fn init(ctx: Context<Initialize>, init_value: u32) {
        ctx.accounts.account.num = init_value;
        Ok(())
    }

    pub fn double(ctx: Context<Double>) {
        ctx.accounts.account.num = ctx.accounts.account.num * 2;
        Ok(())
    }
    
    pub fn add(ctx: Context<Add>, num: u32) {
        ctx.accounts.account.num = ctx.accounts.account.num + num;
        Ok(())
    }
}

struct DataShape {
    pub num: u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>,  
    pub system_program: Program<'info, System>,
    #[account(mut)]
    signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>,  
    #[account(mut)]
    signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>,  
    #[account(mut)]
    signer: Signer<'info>
}