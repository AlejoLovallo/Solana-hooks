use anchor_lang::prelude::*;

#[error_code]
pub enum PlayerErrors {
    #[msg("Player error random error")]
    RandomError,
}
