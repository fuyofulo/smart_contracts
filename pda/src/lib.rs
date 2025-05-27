use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
	pubkey::Pubkey,
	entrypoint,
    program_error::ProgramError,
    system_instruction::create_account,
    program::invoke_signed
};

entrypoint!(process_instruction);

pub fn process_instruction(
	program_id: &Pubkey,  
	accounts: &[AccountInfo],
	_instruction_data: &[u8],  
) -> ProgramResult {

    let iter = &mut accounts.iter();
    let pda = next_account_info(iter)?;
    let user = next_account_info(iter)?;
    let _system_program = next_account_info(iter)?;

    let seeds = &[user.key.as_ref(), b"user"];

    let (pda_public_key, bump) = Pubkey::find_program_address(seeds, program_id);

    if *pda.key != pda_public_key {
        return Err(ProgramError::InvalidArgument);
    }
    
    let ix = create_account(user.key, pda.key, 1000000000, 8, program_id);

    let signer_seeds: &[&[u8]] = &[seeds[0], seeds[1], &[bump]];

    let _ = invoke_signed(&ix, accounts, &[signer_seeds]);

	Ok(())
}




