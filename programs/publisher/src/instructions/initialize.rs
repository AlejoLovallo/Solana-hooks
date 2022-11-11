use anchor_lang::prelude::*;

use crate::{constants::*, states::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    //Payer account (owner of the program)
    #[account(mut)]
    pub owner: Signer<'info>,

    //The [StakeMaster] to be created.
    #[account(
        init,
        seeds = [
            PROGRAM_ADMIN_TAG.as_ref(),
        ],
        bump,
        payer = owner,
        space = ProgramAdmin::LEN
    )]
    pub program_admin: Account<'info, ProgramAdmin>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        let program_admin = &mut self.program_admin;
        program_admin.authority = self.owner.key();
        program_admin.state = 0;

        emit!(InitializeEvent {
            program_admin: program_admin.key(),
        });
        Ok(())
    }
}

#[event]
pub struct InitializeEvent {
    /// The [Stake Master] being created.
    #[index]
    pub program_admin: Pubkey,
}
