use anchor_lang::prelude::*;
// Instructions.
pub mod instructions;
// State.
pub mod states;
// Errors.
pub mod errors;
// Constants.
pub mod constants;

use crate::{instructions::*, states::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hooks {
    use super::*;

    pub fn create_global_state(ctx: Context<CreateGlobalState>) -> Result<()> {
        ctx.accounts.create_global_state()
    }

    pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
        ctx.accounts.create_player()
    }

    pub fn create_tennis_player(ctx: Context<CreateTenisPlayer>) -> Result<()> {
        ctx.accounts.create_tenis_player()
    }
}
