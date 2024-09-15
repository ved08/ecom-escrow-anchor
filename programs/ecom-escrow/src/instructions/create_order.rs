use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{Order, OrderStatus};

#[derive(Accounts)]
#[instruction(order_id: String)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Order::INIT_SPACE,
        seeds = [b"order", user.key().as_ref(), order_id.as_bytes()],
        bump
    )]
    pub order: Account<'info, Order>,
    #[account(
        mut,
        seeds = [b"orderVault", order.key().as_ref()],
        bump
    )]
    pub order_vault: SystemAccount<'info>,
    /// CHECK: We just need this to store seller details 
    pub seller: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateOrder<'info> {
    pub fn create_order(
        &mut self,
        order_id: String,
        amount: u64,
        bumps: &CreateOrderBumps,
    ) -> Result<()> {
        let rent = Rent::get()?;
        let min_rent = rent.minimum_balance(0);
        self.order.set_inner(Order {
            reciever: self.user.key(),
            amount,
            order_id,
            seller: self.seller.key(),
            bump: bumps.order,
            vault_bump: bumps.order_vault,
            status: OrderStatus::PROCESSING,
        });
        let accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.order_vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(cpi_ctx, amount + min_rent)?;
        Ok(())
    }
}
