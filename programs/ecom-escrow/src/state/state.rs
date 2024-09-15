use anchor_lang::prelude::*;

use crate::OrderStatus;

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub reciever: Pubkey,
    pub amount: u64,
    #[max_len(20)]
    /// TBD BASED ON db_id
    pub order_id: String,
    pub seller: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    // ARE WE SUPPOSED TO DO THIS?
    pub status: OrderStatus,
}

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub protocol_fee: u16,
    pub admin: Pubkey,
    pub bump: u8,
}