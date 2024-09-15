use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{error::ErrorCode::*, GlobalState};
use crate::Order;

#[derive(Accounts)]
pub struct FinalizeOrder<'info> {
    #[account(mut)]
    // This is the seller
    pub user: Signer<'info>,
    /// CHECK: We just need pubkey of reciever
    pub reciever: UncheckedAccount<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"order", reciever.key().as_ref(), order.order_id.as_bytes()],
        bump = order.bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        mut,
        seeds = [b"orderVault", order.key().as_ref()],
        bump
    )]
    pub order_vault: SystemAccount<'info>,
    #[account(
        seeds = [b"global"],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    /// CHECK: ADMIN VERIFICATION IS DONE IN INSTRUCTION
    pub admin: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeOrder<'info> {
    pub fn finalize_order(&mut self, order_id: String) -> Result<()> {
        // TODO: ADD RECIEVER CHECK
        require_keys_eq!(
            self.user.key(),
            self.order.seller.key(),
            SellerNotAuthorized
        );
        require_keys_eq!(
            self.reciever.key(),
            self.order.reciever.key(),
            RecieverNotAuthorized
        );
        require_eq!(
            self.order.order_id.clone(), 
            order_id, OrderIdMismatch
        );
        require_keys_eq!(self.global_state.admin, self.admin.key(), AdminMismatch);

        let amount = self.order_vault.to_account_info().lamports();
        let protocol_fee = 
        amount.checked_mul(self.global_state.protocol_fee as u64)
        .unwrap()
        .checked_div(10000)
        .unwrap();
        let resolved_amount = amount.checked_sub(protocol_fee).unwrap();

        let binding = [self.order.vault_bump];
        let signer_seeds = &[&[
            b"orderVault",
            self.order.to_account_info().key.as_ref(),
            &binding,
        ][..]];

        // Transfer amount to reciever
        let accounts = Transfer {
            from: self.order_vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, resolved_amount)?;
        
        // Transfer protocol fee to admin
        let accounts = Transfer {
            from: self.order_vault.to_account_info(),
            to: self.admin.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, protocol_fee)?;
        
        Ok(())
    }
}
