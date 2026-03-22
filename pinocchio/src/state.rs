use crate::utils::{impl_len, impl_load};

pub const DEPTH: usize = 20;

#[repr(C)]
pub struct MerkleTree {
    pub authority: [u8; 32],
    pub depth: u8,
    pub bump: u8,
    pub _pad: [u8; 2],
    pub next_index: u32,
    pub current_root: [u8; 32],
    pub filled_subtrees: [[u8; 32]; DEPTH],
}

impl_len!(MerkleTree);
impl_load!(MerkleTree);
