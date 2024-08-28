use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::error::ErrorCode::*;
use crate::Order;

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut, 
        close = user,
        seeds = [b"order", user.key().as_ref(), order.order_id.as_bytes()],
        bump = order.bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        mut,
        seeds = [b"orderVault", order.key().as_ref()],
        bump = order.vault_bump
    )]
    pub order_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CancelOrder<'info> {
    pub fn cancel_order(&mut self, order_id: String) -> Result<()> {
        // TODO: ADD SELLER SIDE CONDITION
        require_keys_eq!(
            self.user.key(),
            self.order.reciever.key(),
            RecieverNotAuthorized
        );
        require_eq!(self.order.order_id.clone(), order_id, OrderIdMismatch);

        let amount = self.order_vault.to_account_info().lamports();
        let binding = [self.order.vault_bump];
        let signer_seeds = &[&[
            b"orderVault",
            self.order.to_account_info().key.as_ref(),
            &binding,
        ][..]];
        let accounts = Transfer {
            from: self.order_vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}
