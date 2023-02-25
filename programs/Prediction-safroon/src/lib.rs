mod constants;
mod state;
mod utils;
// use crate::state::*;
use anchor_lang::{prelude::*, system_program};
use crate::{constants::*, state::*,utils::*};
use pyth_sdk_solana::load_price_feed_from_account_info;

declare_id!("FKF4YPNCtJjYqwUZwxLDCQjUuPJDGE1v8syc3Hj4nK1s");

#[program]
pub mod prediction_contract {

    use super::*;
    pub fn create_master(_ctx: Context<CreateMaster>) -> Result<()> {
        Ok(())
    }
pub fn create_bet(
    ctx: Context<CreateBet>,
    amount:u64, // amount of the be4t 
    price:f64, // price of the bet
    duration:u32,// seconds
pyth_price_key, Pubkey  // Pubkey 



)  -> Result<()> {





    let master = &mut ctx.accounts.master;
    let bet = &mut  ctx.accounts.bet;

    // Increase the Least id on Each bet Creation on the water

    master.last_bet_id += 1;

    bet.id = master.last_bet_id;
    bet.pyth_price_key = pyth_price_key;
    bet.amount = bet.amount;
    bet.expiry_ ts = get_unix_timestamp() + duration on i64;   // duration of the bet 
    bet.prediction_a = BetPrediction {
        Player:ctx.accounts.player.key(), // player account name for the prediction 
 
        price,


    };
// winner 
// Transfer the amount to the Bet PDA
    system_program::transfer(   // transfer the amount to the bet PDA
        CpiContext::new(
            ctx.accounts.systems_program.to_account_info(), // info putting the player solana on the bet PDA 
            system_program::Transfer{
                from:ctx.accounts.player.to_account_info(),
                to: btx.to_account_info() // 
            },
        ),

        bet.amount,
    )?;


    Ok(())
    }

    pub fn enter _bet(ctx,price) -> Result<()> {

    }



}       



    #[derive(Accounts)] // Account struct
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
        payer=player,
        space=8+8+32+8+8+32+8+1+32+8+1
    )]

    pub bet:Account<'info,Bet>,// Bet Account

    #[account(mut, seeds=[MASTER_SEED],bump)] 

    pub master:Account<'info,Master>

    #[account(mut)]
    pub player:Signer<'info>, // Player 

    pub system_program:Program<'info,System>, // System program 

}
