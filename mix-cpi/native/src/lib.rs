use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
	msg,
	pubkey::Pubkey,
	entrypoint
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Data {
    value: u32
}

#[derive(BorshSerialize, BorshDeserialize)]
enum Operation {
	Initialize(u32),
	Double,
	Half
}


entrypoint!(process_instruction)

pub fn process_instruction(
	_program_id: &Pubkey,  
	accounts: &[AccountInfo],
	instruction_data: &[u8],  
) -> ProgramResult {

	let iter = &mut accounts.iter();
	let user_account = next_account_info(iter)?;
	let data_account = next_account_info(iter)?;
	let system_program = next_account_info(iter)?;

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
	let mut data = Data { value: val };
	data.serialize(&mut *data_account.data.borrow_mut())?;
	Ok(())
}

fn double(data_account: &AccountInfo) -> ProgramResult {
	let mut data = Data::try_from_slice(&data_account.data.borrow_mut())?;
	data.value = data.value * 2;
	data.serialize(&mut *data_account.data.borrow_mut())?;
	Ok(())
}

fn half(data_account: &AccountInfo) -> ProgramResult {
	let mut data = Data::try_from_slice(&data_account.data.borrow_mut())?;
	data.value = data.value / 2;
	data.serialize(&mut *data_account.data.borrow_mut())?;
	Ok(())
}
