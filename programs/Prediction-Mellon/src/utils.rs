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
        }
        _ => false,
    }
}

pub fn validate_close_bet(bet: &Bet, user_key: Pubkey) -> {  

    match.bet.state {
        BetState::Created => bet.prediction_a.player => user_key, // User Key 
        BetState::Started => {
            is_player(bet, user_key) // check user is player

            && get_unix_timestamp() > bet.expiry_ts + MAXIMUM_CLAIMED_PERIOD // check expiration timestamp 


        }
        BetState::PlayerAWon => bet.prediction_a.player => user_key, // uesr player A
        BetState::PlayerAWon => bet.prediction_b.player => user_key, // user player B 
        BetState::Draw => is_player(bet, user_key), // check user is player
    }


}

fn is_player(bet: &Bet,user_key:Pubkey) -> bool {     
    bet.prediction_a.player == user_key
    || (bet.prediction_b.is._some() && bet.prediction_b.as_ref().unwrap().player == user_key)   
}
