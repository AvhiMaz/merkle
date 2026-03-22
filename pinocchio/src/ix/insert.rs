use pinocchio::{AccountView, Address, ProgramResult, error::ProgramError};
use pinocchio_pubkey::derive_address;

use crate::{
    hash::{ZERO_HASHES, hash_pair},
    state::MerkleTree,
};

pub fn process_insert(
    program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    let [authority, merkle_tree, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if data.len() < 32 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let leaf: [u8; 32] = data[0..32]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    if leaf == [0u8; 32] {
        return Err(ProgramError::InvalidArgument);
    }

    let tree_data = unsafe { merkle_tree.borrow_unchecked_mut() };
    let tree = MerkleTree::load_mut(tree_data)?;

    if tree.authority != *authority.address().as_array() {
        return Err(ProgramError::InvalidArgument);
    }

    let expected_pda = derive_address(
        &[b"merkle", &tree.authority, &[tree.bump]],
        None,
        program_id.as_array(),
    );

    if merkle_tree.address().as_array() != &expected_pda {
        return Err(ProgramError::InvalidArgument);
    }

    let max_leaves = 1u32 << tree.depth;
    if tree.next_index >= max_leaves {
        return Err(ProgramError::Custom(1));
    }

    let mut current_index = tree.next_index;
    let mut current_hash = leaf;
    let depth = tree.depth as usize;

    let mut i = 0usize;
    while i < depth {
        if current_index % 2 == 0 {
            tree.filled_subtrees[i] = current_hash;
            current_hash = hash_pair(&current_hash, &ZERO_HASHES[i]);
        } else {
            current_hash = hash_pair(&tree.filled_subtrees[i], &current_hash);
        }
        current_index >>= 1;
        i += 1;
    }

    tree.current_root = current_hash;
    tree.next_index += 1;

    Ok(())
}
