use crate::{hash::hash_pair, state::MerkleTree};
use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Verify<'info> {
    pub merkle_tree: &'info Account<MerkleTree>,
}

impl<'info> Verify<'info> {
    #[inline(always)]
    pub fn verify(
        &self,
        leaf: [u8; 32],
        index: u32,
        proof_bytes: [u8; 640],
    ) -> Result<(), ProgramError> {
        let tree = &self.merkle_tree;

        let mut current_hash = leaf;
        let mut current_index = index;

        let depth = tree.depth as usize;
        let mut i = 0usize;
        while i < depth {
            let sibling: &[u8; 32] = proof_bytes[i * 32..(i + 1) * 32]
                .try_into()
                .map_err(|_| ProgramError::InvalidArgument)?;

            current_hash = if current_index.is_multiple_of(2) {
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
}
