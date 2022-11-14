use anchor_lang::prelude::*;

use crate::{constants::*, states::*};

#[derive(Accounts)]
pub struct Unsuscribe<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            PROGRAM_ADMIN_TAG.as_ref(),
        ],
        bump
    )]
    pub program_admin: Box<Account<'info, ProgramAdmin>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Unsuscribe<'info> {
    pub fn unsuscribe(&mut self, suscriber: Pubkey) -> Result<()> {
        let program_admin = &mut self.program_admin;

        let index: Option<usize> = program_admin.suscribers.iter().position(&suscriber);

        emit!(UnsuscribeEvent {
            suscriber: suscriber.key(),
        });
        Ok(())
    }
}

#[event]
pub struct UnsuscribeEvent {
    /// The [Stake Master] being created.
    #[index]
    pub suscriber: Pubkey,
}
