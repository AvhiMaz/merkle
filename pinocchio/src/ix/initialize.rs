use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::rent::{ACCOUNT_STORAGE_OVERHEAD, DEFAULT_LAMPORTS_PER_BYTE},
};
use pinocchio_pubkey::derive_address;
use pinocchio_system::instructions::CreateAccount;

use crate::{
    hash::ZERO_HASHES,
    state::{DEPTH, MerkleTree},
};

const RENT_EXEMPT_LAMPORTS: u64 =
    2 * (ACCOUNT_STORAGE_OVERHEAD + MerkleTree::LEN as u64) * DEFAULT_LAMPORTS_PER_BYTE;

pub fn process_initialize(
    program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    let [authority, merkle_tree, _system_program, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !merkle_tree.is_data_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let bump = data[0];

    let expected_pda = derive_address(
        &[b"merkle", authority.address().as_ref(), &[bump]],
        None,
        program_id.as_array(),
    );

    if merkle_tree.address().as_array() != &expected_pda {
        return Err(ProgramError::InvalidArgument);
    }

    let binding = [bump];
    let seeds = [
        Seed::from(b"merkle".as_ref()),
        Seed::from(authority.address().as_ref()),
        Seed::from(binding.as_ref()),
    ];

    CreateAccount {
        from: authority,
        to: merkle_tree,
        lamports: RENT_EXEMPT_LAMPORTS,
        space: MerkleTree::LEN as u64,
        owner: program_id,
    }
    .invoke_signed(&[Signer::from(&seeds[..])])?;

    let tree_data = unsafe { merkle_tree.borrow_unchecked_mut() };
    let tree = MerkleTree::load_mut(tree_data)?;

    tree.authority = *authority.address().as_array();
    tree.depth = DEPTH as u8;
    tree.bump = bump;
    tree._pad = [0; 2];
    tree.next_index = 0;
    tree.current_root = ZERO_HASHES[DEPTH];
    tree.filled_subtrees = core::array::from_fn(|i| ZERO_HASHES[i]);

    Ok(())
}
