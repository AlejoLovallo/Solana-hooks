use anchor_lang::prelude::*;
// Instructions.
pub mod instructions;
// State.
pub mod states;
// Errors.
pub mod errors;
// Constants.
pub mod constants;
// Utils.
pub mod utils;

use crate::instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod publisher {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
