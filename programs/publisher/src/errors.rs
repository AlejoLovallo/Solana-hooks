use anchor_lang::prelude::*;

#[error_code]
pub enum PublisherError {
    #[msg("InvalidProgramAddress")]
    InvalidProgramAddress,

    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("Suscribers limit reached")]
    SuscribersLimit,
}
