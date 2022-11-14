use anchor_lang::prelude::*;

use crate::{constants::*, errors::*, states::*};

#[derive(Accounts)]
pub struct Suscribe<'info> {
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

impl<'info> Suscribe<'info> {
    pub fn suscribe(&mut self, suscriber: Pubkey) -> Result<()> {
        let program_admin = &mut self.program_admin;

        require!(
            program_admin.suscribers.len() <= 32,
            PublisherError::SuscribersLimit
        );
        program_admin.suscribers.push(suscriber);

        emit!(SuscribeEvent {
            suscriber: suscriber.key(),
        });
        Ok(())
    }
}

#[event]
pub struct SuscribeEvent {
    /// The [Stake Master] being created.
    #[index]
    pub suscriber: Pubkey,
}
