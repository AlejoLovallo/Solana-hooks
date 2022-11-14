use anchor_lang::prelude::*;

use crate::{errors::*, states::*};

// access control modifier
pub fn is_admin(admin: &Account<ProgramAdmin>, authority: &AccountInfo) -> Result<()> {
    require!(
        admin.authority.eq(&authority.key()),
        PublisherError::Unauthorized
    );
    Ok(())
}
