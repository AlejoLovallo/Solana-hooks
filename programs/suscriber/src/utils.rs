use crate::states::*;
use anchor_lang::prelude::*;

pub fn on_create_player(item: &mut impl IPlayer) -> Result<()> {
    item.on_creation();
    Ok(())
}
