use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
        entrypoint
}

enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

struct Counter {
    count: u32
}

entrypoint!(process_instruction)

pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
) -> ProgramResult {

	// getting the account from the account array
    let acc = next_account_info(&mut accounts.iter())?; 
    // '?' same as doing unwrap 
    // use unwrap or ? when you know for sure there is a value

	// desrializing the instruction data
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

	// borrowing the counter count
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

	// matching instruction and performing accordingly on the count
    match instruction_type {
        InstructionType::Increment(value) => {
            counter.data += value;
         },
        InstrctionType::Decrement(value) => {
            counter.data -= value;
        }
    }

	// serializing the counter data and storing it back to the account
    counter_data.serialize(&mut *acc.data.borrow_mut());

	// printing success message 
    msg!("Contract succeded");

	// confirming that the contract has done its job
    Ok(());
}
