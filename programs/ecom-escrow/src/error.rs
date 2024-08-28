use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Seller not authorized")]
    SellerNotAuthorized,
    #[msg("Reciever not authorized")]
    RecieverNotAuthorized,
    #[msg("Order id mismatch")]
    OrderIdMismatch
}
