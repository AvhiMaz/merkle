use crate::{
    hash::{hash_pair, ZERO_HASHES},
    state::MerkleTree,
};
use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Insert<'info> {
    pub authority: &'info Signer,

    #[account(
        mut,
        seeds = [b"merkle", authority],
        bump,
        has_one = authority,
    )]
    pub merkle_tree: &'info mut Account<MerkleTree>,
}

impl<'info> Insert<'info> {
    #[inline(always)]
    pub fn insert(&mut self, leaf: [u8; 32]) -> Result<(), ProgramError> {
        let tree = &mut *self.merkle_tree;

        if leaf == [0u8; 32] {
            return Err(ProgramError::InvalidArgument);
        }

        let max_leaves = 1u32 << tree.depth;
        if tree.next_index >= max_leaves {
            return Err(ProgramError::Custom(1));
        }

        let mut current_index = u32::from(tree.next_index);
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
}
