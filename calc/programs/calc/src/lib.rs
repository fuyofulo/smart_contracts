use anchor_lang::prelude::*;

declare_id!("A6A2ZiorsHjBUQT2C5K4zEx9vZmVVHGs25hdmMaRmWbQ");

#[program]
pub mod calc {
    use super::*;

    pub fn init(ctx: Context<Initialize>, init_value: u32) -> Result<()> {
        ctx.accounts.account.num = init_value;
        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * 2;
        Ok(())
    }
    
    pub fn add(ctx: Context<Add>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num + num;
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num - num;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiply>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * num;
        Ok(())
    }

    pub fn half(ctx: Context<Half>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num / 2;
        Ok(())
    }


}

#[account]
pub struct DataShape {
    pub num: u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>,  
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,  
    #[account(mut)]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,  
    #[account(mut)]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)] 
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Half<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct Multiply<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>
}