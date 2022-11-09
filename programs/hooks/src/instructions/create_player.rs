use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::PlayerErrors,
    states::{IPlayer, PlayerData, ProgramAdmin},
    utils::*,
};

impl IPlayer for PlayerData {
    fn on_creation(&mut self) -> Result<()> {
        if self.banned {
            return err!(PlayerErrors::RandomError);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      seeds = [
        PROGRAM_ADMIN_TAG.as_ref(),
      ],
      bump
    )]
    pub program_admin: Box<Account<'info, ProgramAdmin>>,

    #[account(
        init,
        seeds = [
            PLAYER_DATA_TAG.as_ref(),
            user.key().as_ref(),
        ],
        bump,
        payer = user,
        space = PlayerData::LEN
    )]
    pub player: Account<'info, PlayerData>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreatePlayer<'info> {
    pub fn create_player(&mut self) -> Result<()> {
        let player = &mut self.player;

        player.owner = self.user.key();
        player.banned = false;
        player.active = true;

        // Nahive approach where you check if is banned
        let mut player_data = PlayerData {
            owner: player.owner,
            banned: player.banned,
            active: player.active,
        };

        on_create_player(&mut player_data);

        emit!(CreatePlayerEvent {
            player: player.key(),
            owner: player.owner
        });

        Ok(())
    }
}

#[event]
pub struct CreatePlayerEvent {
    #[index]
    pub player: Pubkey,

    pub owner: Pubkey,
}
