mod constants;
mod state;
// use crate::state::*;
use crate::{constants::*, state::*};
use anchor_lang::{prelude::*, system_program};
use pyth_sdk_solana::{load_price_feed_from_account_info};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod prediction_Contract {

    use super::*;
    pub fn create_master(_ctx:Context<CreateMaster>) -> Result<()> {
        Ok(())

}

// pub fn create_bat() -> Result<()> {

// }

// pub fn 
#[derive(Accounts)] // Account stuct
pub struct CreateMaster<'info> {
        #[account(
            init,
            seeds = [MASTER_SEED],
            payer = payer,
            space = 8 + 8,
            bump
            
            
        )]
    
        pub master: Account<'info, Master>, // Account itself

        #[account(mut)]
        pub payer: Signer<'info>,

        pub system_program: Program<'info, System> // system program
}
}
