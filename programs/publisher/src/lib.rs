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

pub use suscriber::*;

use crate::{instructions::*, utils::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnZ");

#[program]
pub mod publisher {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()
    }

    #[access_control(is_admin(&ctx.accounts.program_admin, &ctx.accounts.authority))]
    pub fn suscribe(ctx: Context<Suscribe>, suscriber: Pubkey) -> Result<()> {
        ctx.accounts.suscribe(suscriber)
    }

    #[access_control(is_admin(&ctx.accounts.program_admin, &ctx.accounts.authority))]
    pub fn unsuscribe(ctx: Context<Unsuscribe>, suscriber: Pubkey ) -> Result<()> {
        ctx.accounts.unsuscribe(suscriber)
    }

    pub fn notify_suscribers(ctx: Context<NotifySuscribers>) -> Result<()> {}
}
