use crate::{hash::ZERO_HASHES, state::MerkleTree};
use quasar_lang::prelude::*;

pub const DEPTH: u8 = 20;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: &'info mut Signer,

    #[account(
        init,
        payer = authority,
        space = MerkleTree::SPACE,
        seeds = [b"merkle", authority],
        bump,
    )]
    pub merkle_tree: &'info mut Account<MerkleTree>,

    pub system_program: &'info Program<System>,
}

impl<'info> Initialize<'info> {
    #[inline(always)]
    pub fn initialize(&mut self, _bumps: &InitializeBumps) -> Result<(), ProgramError> {
        self.merkle_tree.set_inner(
            *self.authority.address(),
            DEPTH,
            0u32,
            ZERO_HASHES[DEPTH as usize],
            core::array::from_fn(|i| ZERO_HASHES[i]),
        );
        Ok(())
    }
}
