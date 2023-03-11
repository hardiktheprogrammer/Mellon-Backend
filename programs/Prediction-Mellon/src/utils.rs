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

pub fn validate_claim_bet(bet: &Bet) -> bool {
    match bet.state {
        BetState::Started => {
            let current_ts = get_unix_timestamp(); // current timestamp
            let time_passed_since_expiry = current_ts - bet.expiry_ts; // time passed since expiation
            0 < time_passed_since_expiry && time_passed_since_expiry <= bet.expiry_ts
        },
        _ => false,
    }
}