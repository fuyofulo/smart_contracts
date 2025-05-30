use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Data {
    pub value: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Operation {
    Initialize(u32),
    Double,
    Half,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();

    let data_account = next_account_info(&mut iter)?;

    let instruction = Operation::try_from_slice(instruction_data)?;

    match instruction {
        Operation::Initialize(val) => {
            initialize(data_account, val)?;
        }
        Operation::Double => {
            double(data_account)?;
        }
        Operation::Half => {
            half(data_account)?;
        }
    }

    Ok(())
}

fn initialize(data_account: &AccountInfo, val: u32) -> ProgramResult {
    let data = Data { value: val };
    data.serialize(&mut *data_account.data.borrow_mut())?;
    Ok(())
}

fn double(data_account: &AccountInfo) -> ProgramResult {
    let mut data = Data::try_from_slice(&data_account.data.borrow_mut())?;
    data.value *= 2;
    data.serialize(&mut *data_account.data.borrow_mut())?;
    Ok(())
}

fn half(data_account: &AccountInfo) -> ProgramResult {
    let mut data = Data::try_from_slice(&data_account.data.borrow_mut())?;
    data.value /= 2;
    data.serialize(&mut *data_account.data.borrow_mut())?;
    Ok(())
}
