use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("cannot enter")]
    CannotEnter,
    #[msg("cannot claim ")]
    ConnotClaim,
    #[msg("cannot close")]
    CannotClose,
    #[msg("Given Key for account Does not match")]
    InvalidKey,
    #[msg("Bet price is too High parse to u32")]
    PriceIsHigh,
}
