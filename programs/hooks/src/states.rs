use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;

#[account]
#[derive(Default)]
pub struct ProgramAdmin {
    // Progra admin owner
    pub authority: Pubkey,
}

impl ProgramAdmin {
    pub const LEN: usize = (DISCRIMINATOR_LENGTH + 32 ) * 2;
}

pub trait IPlayer {
    fn check_is_not_banned(&mut self) -> Result<&mut Self>;
}

#[account]
#[derive(Default)]
pub struct PlayerData {
    /// Owner of this PlayerData account
    pub player: Pubkey,
    /// Is banned?
    pub banned: bool,
    /// Activated
    pub active: bool,
}

impl PlayerData {
    pub const LEN: usize = (DISCRIMINATOR_LENGTH + 32 + 8 + 8)
}

