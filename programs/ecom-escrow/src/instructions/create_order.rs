use anchor_lang::prelude::*;

use crate::Order;


#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = 8 + Order::INIT_SPACE,
    )]
    pub order: Account<'info, Order>,
    pub system_program: Program<'info, System>,
}