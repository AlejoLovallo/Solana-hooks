use anchor_lang::prelude::*;

use crate::{
    constants::*,
    states::{IPlayer, PlayerTenis, ProgramAdmin},
    utils::*,
};

impl IPlayer for PlayerTenis {
    fn on_creation(&mut self) -> Result<()> {
        if self.rackets < 8 {
            self.rackets = self.rackets + 10;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTennisPlayer<'info> {
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
            PLAYER_TENIS_TAG.as_ref(),
            user.key().as_ref(),
        ],
        bump,
        payer = user,
        space = PlayerTenis::LEN
    )]
    pub player: Account<'info, PlayerTenis>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateTennisPlayer<'info> {
    pub fn create_tenis_player(&mut self, rackets: u8) -> Result<()> {
        let player = &mut self.player;

        player.owner = self.user.key();
        player.banned = false;
        player.active = true;
        player.rackets = rackets;

        let mut player_tenis = PlayerTenis {
            owner: player.owner,
            banned: player.banned,
            active: player.active,
            rackets: player.rackets,
        };

        on_create_player(&mut player_tenis);

        emit!(CreateTenisPlayerEvent {
            player: player.key(),
            owner: player.owner
        });

        Ok(())
    }
}

#[event]
pub struct CreateTenisPlayerEvent {
    #[index]
    pub player: Pubkey,

    pub owner: Pubkey,
}
