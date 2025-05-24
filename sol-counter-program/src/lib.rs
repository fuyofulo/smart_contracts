use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Getting the account from the account array
    let acc = next_account_info(&mut accounts.iter())?;
    
    // Deserializing the instruction data
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    
    // Borrowing the counter data
    let mut counter = Counter::try_from_slice(&acc.data.borrow())?;
    
    // Matching instruction and performing accordingly on the count
    match instruction_type {
        InstructionType::Increment(value) => {
            counter.count += value;
        },
        InstructionType::Decrement(value) => {
            counter.count -= value;
        }
    }
    
    // Serializing the counter data and storing it back to the account
    counter.serialize(&mut *acc.data.borrow_mut())?;
    
    // Printing success message 
    msg!("Contract succeeded");
    
    // Confirming that the contract has done its job
    Ok(())
}
