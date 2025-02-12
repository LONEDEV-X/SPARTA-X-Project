use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use spl_token::instruction::initialize_account;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let token_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;

    // Initialize the token account
    initialize_account(
        program_id,
        token_account,
        mint_account,
        owner_account,
    )?;

    Ok(())
}
