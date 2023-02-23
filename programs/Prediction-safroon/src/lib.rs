mod constants;
mod state;
// use crate::state::*;
use anchor_lang::{prelude::*, system_program};
use crate::{constants::*, state::*};
use pyth_sdk_solana::load_price_feed_from_account_info;

declare_id!("FKF4YPNCtJjYqwUZwxLDCQjUuPJDGE1v8syc3Hj4nK1s");

#[program]
pub mod prediction_contract {

    use super::*;
    pub fn create_master(_ctx: Context<CreateMaster>) -> Result<()> {
        Ok(())
    }

    
    #[derive(Accounts)] // Account stuct
    pub struct CreateMaster<'info> {
        #[account(
            init,
            payer = payer,
            space = 8 + 8,
            seeds = [MASTER_SEED],
            bump
        )]
        pub master: Account<'info, Master>, // Account itself

        #[account(mut)]
        pub payer: Signer<'info>,

        pub system_program: Program<'info, System>, // system program


        
    }
#[derive(Accounts)]

pub struct CreateBet<'info> {

    #[account(

        init,
        payer=payer,
        space=8+8+32+8+8+32+8+1+32+8+1,
    )]

    pub bet:Account<'info,Bet> 

    #[account(mut)] 
}
