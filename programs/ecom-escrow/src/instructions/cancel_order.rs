use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::Order;
use crate::error::ErrorCode::RecieverNotAuthorized;

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
        bump
    )]
    pub order_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> CancelOrder<'info> {
    pub fn cancel_order(&mut self) -> Result<()> {

        // TODO: ADD SELLER SIDE CONDITION
        require_keys_eq!(self.user.key(), self.order.reciever.key(), RecieverNotAuthorized);

        let amount = self.order_vault.to_account_info().lamports();
        let binding = [self.order.bump];
        let signer_seeds = &[&[
         b"order",
         self.user.to_account_info().key.as_ref(),
         self.order.order_id.as_bytes(),
         &binding,
        ][..]];
        let accounts = Transfer {
            from: self.order_vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}