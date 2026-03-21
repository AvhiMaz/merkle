use quasar_lang::prelude::*;

#[account(discriminator = [0x4D, 0x52, 0x4B, 0x4C])]
pub struct MerkleTree {
    pub authority: Address,
    pub depth: u8,
    pub next_index: u32,
    pub current_root: [u8; 32],
    pub filled_subtrees: [[u8; 32]; 20],
}
