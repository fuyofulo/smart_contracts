use anchor_lang::prelude::*;

declare_id!("HtrscvEkCiZYQMy2VKSSEQAjuCAQ3GjxCYaog1KkjM8H");

#[program]
pub mod rect {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
