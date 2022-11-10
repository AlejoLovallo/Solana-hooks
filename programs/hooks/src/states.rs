use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;

#[account]
#[derive(Default)]
pub struct ProgramAdmin {
    // Progra admin owner
    pub authority: Pubkey,
}

impl ProgramAdmin {
    pub const LEN: usize = (DISCRIMINATOR_LENGTH + 32) * 2;
}

pub trait IPlayer {
    fn on_creation(&mut self) -> Result<()>;
}

#[account]
#[derive(Default)]
pub struct PlayerData {
    /// Owner of this PlayerData account
    pub owner: Pubkey,
    /// Is banned?
    pub banned: bool,
    /// Activated
    pub active: bool,
}

impl PlayerData {
    pub const LEN: usize = (DISCRIMINATOR_LENGTH + 32 + 8 + 8);
}

#[account]
#[derive(Default)]
pub struct PlayerTenis {
    /// Owner of this PlayerData account
    pub owner: Pubkey,
    /// Is banned?
    pub banned: bool,
    /// Activated
    pub active: bool,
    /// Tenis rackets
    pub rackets: u8,
}

impl PlayerTenis {
    pub const LEN: usize = (DISCRIMINATOR_LENGTH + 32 + 8 + 8 + 8);
}
