use anchor_lang::prelude::*;

use crate::{
    errors::PlayerErrors,
    states::{IPlayer, PlayerData},
};

impl IPlayer for PlayerData {
    fn check_is_not_banned(&mut self) -> Result<&mut Self> {
        if self.banned {
            return err!(PlayerErrors::RandomError);
        }

        Ok(self)
    }
}

use anchor_lang::prelude::*;

use crate::{constants::*, states::*};

#[derive(Accounts)]
pub struct CreateGlobalState<'info> {
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

impl<'info> CreateGlobalState<'info> {
    pub fn create_global_state(&mut self) -> Result<()> {
        let program_admin = &mut self.program_admin;
        program_admin.authority = self.owner.key();

        emit!(CreateGlobalStateEvent {
            program_admin: program_admin.key(),
        });
        Ok(())
    }
}

#[event]
pub struct CreateGlobalStateEvent {
    /// The [Stake Master] being created.
    #[index]
    pub program_admin: Pubkey,
}
