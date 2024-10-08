pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HjJQRt9aZ21yzVqoVx9qaGsTukk3CnodmmE1yqkwqRYA");

#[program]
pub mod ecom_escrow {
    use super::*;
    pub fn init_global_state(ctx: Context<InitGlobalState>) -> Result<()> {
        ctx.accounts.init_global_state(ctx.bumps.global_state)?;
        Ok(())
    }
    pub fn update_global_state(ctx: Context<UpdateGlobalState>, fee: u16) -> Result<()> {
        ctx.accounts.update_fee(fee)?;
        Ok(())
    }
    pub fn create_order(ctx: Context<CreateOrder>, order_id: String, amount: u64) -> Result<()> {
        ctx.accounts.create_order(order_id, amount, &ctx.bumps)?;
        Ok(())
    }
    pub fn cancel_order(ctx: Context<CancelOrder>, order_id: String) -> Result<()> {
        ctx.accounts.cancel_order(order_id)?;
        Ok(())
    }
    pub fn finalize_order(ctx: Context<FinalizeOrder>, order_id: String) -> Result<()> {
        ctx.accounts.finalize_order(order_id)?;
        Ok(())
    }
}
