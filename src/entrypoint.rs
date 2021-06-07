use solana_program::{
    entrypoint,
    pubkey::Pubkey,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
};
use crate::processor::Processor;

//**************************************************************************************************
//  entrypoint!
//--------------------------------------------------------------------------------------------------
entrypoint!(process_instruction);

//**************************************************************************************************
//  process_instruction
//--------------------------------------------------------------------------------------------------
fn process_instruction(
    _program_id:        &Pubkey,
    _accounts:          &[AccountInfo],
    _instruction_data:  &[u8]
) -> ProgramResult {
    Processor::process_instruction(_program_id, _accounts, _instruction_data)
}