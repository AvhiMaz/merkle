#![allow(unexpected_cfgs)]

use pinocchio::{
    AccountView, Address, ProgramResult, default_panic_handler, error::ProgramError, no_allocator,
};

use crate::ix::{initialize::process_initialize, insert::process_insert, verify::process_verify};

no_allocator!();

default_panic_handler!();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    unsafe { pinocchio::entrypoint::process_entrypoint::<3>(input, process_instruction) }
}

fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((&0, rest)) => process_initialize(program_id, accounts, rest),
        Some((&1, rest)) => process_insert(program_id, accounts, rest),
        Some((&2, rest)) => process_verify(accounts, rest),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
