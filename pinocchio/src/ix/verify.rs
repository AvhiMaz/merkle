use pinocchio::{AccountView, ProgramResult, error::ProgramError};

use crate::{hash::hash_pair, state::MerkleTree};

pub fn process_verify(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    let [merkle_tree, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if data.len() < 32 + 4 + 640 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let leaf: [u8; 32] = data[0..32]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let index = u32::from_le_bytes(
        data[32..36]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let proof_bytes: &[u8; 640] = data[36..676]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let tree_data = unsafe { merkle_tree.borrow_unchecked() };
    let tree = MerkleTree::load(tree_data)?;

    let depth = tree.depth as usize;
    let mut current_hash = leaf;
    let mut current_index = index;

    let mut i = 0usize;
    while i < depth {
        let sibling: &[u8; 32] = proof_bytes[i * 32..(i + 1) * 32]
            .try_into()
            .map_err(|_| ProgramError::InvalidArgument)?;

        current_hash = if current_index % 2 == 0 {
            hash_pair(&current_hash, sibling)
        } else {
            hash_pair(sibling, &current_hash)
        };
        current_index >>= 1;
        i += 1;
    }

    if current_hash == tree.current_root {
        Ok(())
    } else {
        Err(ProgramError::Custom(2))
    }
}
