#![cfg_attr(not(test), no_std)]

mod hash;
mod ix;
mod state;

use ix::*;
use quasar_lang::prelude::*;

declare_id!("Dd1RAREe2DcVY7vKyGiGcx7xVMeRL3z1H1dFAecLYypK");

#[program]
mod merkle {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>) -> Result<(), ProgramError> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    #[instruction(discriminator = 1)]
    pub fn insert(ctx: Ctx<Insert>, leaf: [u8; 32]) -> Result<(), ProgramError> {
        ctx.accounts.insert(leaf)
    }

    #[instruction(discriminator = 2)]
    pub fn verify(
        ctx: Ctx<Verify>,
        leaf: [u8; 32],
        index: u32,
        proof_bytes: [u8; 640],
    ) -> Result<(), ProgramError> {
        ctx.accounts.verify(leaf, index, proof_bytes)
    }
}

#[cfg(test)]
mod tests;
