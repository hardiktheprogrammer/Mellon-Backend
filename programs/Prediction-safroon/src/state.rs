use anchor_lang::prelude::*;

#[account]
pub struct Master {
    // Master Account hold the programe and the master account itself
    pub last_bet_id: u64,
}

#[account]
pub struct Bet {
    pub id: u64,                                 // new ID
    pub amount: u64,                             // Beting amount wroth
    pub prediction_a: BettingPrediction,         // take prediction from the bet creator
    pub prediction_b: Option<BettingPrediction>, // an Option
    pub expire_ts: i64,                          // expiration time of the bet that has beee created
    pub state: BetState,
    pub pyth_price_key: Pubkey, // Bet becomes the price of the bet
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]

pub struct BettingPrediction {
    pub player: Pubkey, // Player

    pub price: f64, // Price of the bet that has been created
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize, AnchorDeserialize)]

pub enum BetState {
    Created,
    Expired,
    Draw,
    PlayerAWon,
    PlayerBWon,
}
