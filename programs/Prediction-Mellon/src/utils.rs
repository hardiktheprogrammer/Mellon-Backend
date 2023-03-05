use anchor_lang::{
    prelude::*,
    solana_program::clock::{Clock, UnixTimestamp},
};

use crate::{constants::*, state::*};

pub fn get_unix_timestamp() -> UnixTimestamp {
    Clock::get().unwrap().unix_timestamp // timestamp
}

pub fn validate_enter_bet(bet: &Bet) -> bool {
    bet.prediction_b.is_none()
        && (bet.expiry_ts - MINIMUM_REMAINING_TIME_UNTIL_EXPIRY > get_unix_timestamp())
}
