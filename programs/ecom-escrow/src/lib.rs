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

    pub fn create_order(ctx: Context<CreateOrder>) -> Result<()> {
    
        Ok(())
    }
    // pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn finalize_order(ctx: Context<FinalizeOrder>) -> Result<()> {
    //     Ok(())
    // }
    
}
