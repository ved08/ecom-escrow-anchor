use anchor_lang::prelude::*;

use crate::{ADMIN, GlobalState};
#[derive(Accounts)]
pub struct UpdateGlobalState<'info> {
    #[account(mut, address = ADMIN)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"global"],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,
    pub system_program: Program<'info, System>
}

impl <'info> UpdateGlobalState<'info> {
    pub fn update_fee(&mut self, protocol_fee: u16) -> Result<()>{
        self.global_state.protocol_fee = protocol_fee;

        Ok(())
    }
}