use anchor_lang::prelude::*;

#[constant]
pub const ADMIN: Pubkey = pubkey!("CztRNUjoAWDHpB1oNkGnQvry6SjAtoUR7hMSejDXV71i");

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    PROCESSING,
    SHIPPED,
    DELIVERED,
    CANCELLED
}

impl Space for OrderStatus {
    const INIT_SPACE: usize = 1;
}