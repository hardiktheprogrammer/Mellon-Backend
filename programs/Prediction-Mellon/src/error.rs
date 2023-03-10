use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("cannot enter")]
    CannotEnter,
    #[msg("cannot claim ")]
    ConnotClaim,
    #[msg("cannot close")]
    CannotClose,
}
