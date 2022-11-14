use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;

#[account]
#[derive(Default)]
pub struct ProgramAdmin {
    // Progra admin owner
    pub authority: Pubkey,
    // State
    pub state: u64,
    // Suscribers
    pub suscribers: Vec<Pubkey>,
}

impl ProgramAdmin {
    pub const LEN: usize = (DISCRIMINATOR_LENGTH + 32 + 64 + 32 * 32) * 2;
}

pub trait IPlayer {
    fn update(&mut self) -> Result<u64>;
}

/*
    public abstract class Observer {
        protected Subject subject;
        public abstract void update();
    }
*/
