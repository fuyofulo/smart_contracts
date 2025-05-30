use anchor_lang::prelude::*;
use borsh::BorshSerialize;

declare_id!("3DtoP64uvKqmtUcLbsezanb1knR8kHboeQnPtULdGBgb");

#[derive(BorshSerialize)]
pub enum Operation {
    Initialize(u32),
    Double,
    Half
}

#[program]
pub mod anchor_cpi {
    use super::*;

    pub fn init(ctx: Context<Initialize>, init_value: u32) -> Result<()> {
        
        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, false)
        ];

        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts,
            data: Operation::Initialize(init_value).try_to_vec()?
        };

        anchor_lang::solana_program::program::invoke(
            &instruction, 
            &[ ctx.accounts.data_account.to_account_info() ]
        )?;

        Ok(());
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, false)
        ];

        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts,
            data: Operation::Double.try_to_vec()?
        };

        anchor_lang::solana_program::invoke(
            &instruction,
            & [ ctx.accounts.data_account.to_account_info() ]
        )?;

        Ok(());
    }

    pub fn half(ctx: Context<Half>) -> Result<()> {
        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, false)
        ];

        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts,
            data: Operation::Half.try_to_vec()?
        };

        anchor_lang::solana_program::invoke(
            &instruction,
            & [ ctx.accounts.data_account.to_account_info() ]
        )?;

        Ok(());
    }



}

#[account]
pub struct DataShape {
    value: u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub data_account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Account<'info>,
    pub native_program: Account<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataShape>,
    pub native_program: Account<'info>
}

#[derive(Accounts)]
pub struct Half<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataShape>,
    pub native_program: Account<'info>
}