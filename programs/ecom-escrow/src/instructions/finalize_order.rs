use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::Order;

#[derive(Accounts)]
pub struct FinalizeOrder<'info> {
    #[account(mut)]
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
    pub system_program: Program<'info, System>,
}

impl <'info> FinalizeOrder<'info> {
    pub fn finalize_order(&mut self) -> Result<()> {
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