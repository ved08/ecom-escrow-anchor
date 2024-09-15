use anchor_lang::prelude::*;

use crate::{GlobalState, ADMIN};
#[derive(Accounts)]
pub struct InitGlobalState<'info> {
    #[account(mut, address = ADMIN)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"global"],
        space = 8 + GlobalState::INIT_SPACE,
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    pub system_program: Program<'info, System>
}

impl <'info> InitGlobalState<'info> {
    pub fn init_global_state(&mut self) -> Result<()>{
        self.global_state.set_inner(GlobalState {
            protocol_fee: 0,
            admin: self.admin.key(),
            bump: self.global_state.bump,
        });
        Ok(())
    }
}