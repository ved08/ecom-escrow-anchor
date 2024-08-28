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

    pub fn create_order(ctx: Context<CreateOrder>, order_id: String, amount: u64) -> Result<()> {
        ctx.accounts.create_order(order_id, amount, &ctx.bumps)?;
        Ok(())
    }
    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        ctx.accounts.cancel_order()?;
        Ok(())
    }
    pub fn finalize_order(ctx: Context<FinalizeOrder>) -> Result<()> {
        ctx.accounts.finalize_order()?;
        Ok(())
    }
}
