use anchor_lang::{
    prelude::*,
    solana_program::clock::{Clock, UinxTimestamp},
};

use crate::{constants::*,state::*}


pub fn get_unix_timestamp() -> UinxTimestamp{
    Clock::get().unwrap().unix_timestamp

}
