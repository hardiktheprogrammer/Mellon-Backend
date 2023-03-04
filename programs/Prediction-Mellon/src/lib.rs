mod constants;
mod state;
mod utils;
mod error;
// use crate::state::*;
use crate::{constants::*, state::*, utils::*,error::*};
use anchor_lang::{prelude::*, system_program};
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
        amount: u64,            // amount of the be4t
        price: f64,             // price of the bet
        duration: u32,          // seconds
        pyth_price_key: Pubkey, // Pubkey
    ) -> Result<()> {
        let master = &mut ctx.accounts.master;
        let bet = &mut ctx.accounts.bet;

        // Increase the Lea t id on Each bet Creation on the water

        master.last_bet_id += 1; // last bet id 

        bet.id = master.last_bet_id;
        bet.pyth_price_key = pyth_price_key;
        bet.amount = amount;
        bet.expiry_ts = get_unix_timestamp() + duration as i64; // duration of the bet
        bet.prediction_a = BettingPrediction {
            player: ctx.accounts.player.key(), // player account name for the prediction

            price,
        };
        // winner
        // Transfer the amount to the Bet PDA
        system_program::transfer(
            // transfer the amount to the bet PDA
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), // info putting the player solana on the bet PDA
                system_program::Transfer { 
                    from: ctx.accounts.player.to_account_info(),
                    to: bet.to_account_info(), //
                },  
            ),
            bet.amount,
        )?;

        Ok(())
    }

      pub fn enter_bet(ctx:Context<EnterBet>,price: f64) -> Result<()> { // enter the bet and the price


        let bet = &mut ctx.accounts.bet; // function for prediction B
        bet.prediciton_b = Some(BettingPrediction{
            player: ctx.accounts.player.key(),
            price,
        });
        bet.state = BetState::Started;

        // transfer the amount to the bet PDA
        system_program::transfer(



            CpiContext::new( 
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer{
                    from:ctx.accounts.player.to_account_info(),

                    to: bet.to_account_info()


                },

            ),
            bet.amount,
            
        )?;

        Ok(())
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
        space=8+8+32+8+8+32+8+1+32+8+1,
        seeds=[BET_SEED, &(master.last_bet_id + 1).to_le_bytes() ],
        bump
    )]
    pub bet: Account<'info, Bet>, // Bet Account

    #[account(mut, seeds=[MASTER_SEED],bump)]
    pub master: Account<'info, Master>,

    #[account(mut)]
    pub player: Signer<'info>, // Player

    pub system_program: Program<'info, System>, // System program
}

 #[derive(Accounts)]

 pub struct EnterBet<'info> {
     #[account(
        mut,
        SEEDS=[BET_SEED,&bet.id.to_le_bytes()], // bets seeds are the bet seed bytes 
        bump,
        constraint= validate_enter_bet(&*bet) @ BetError::CannotEnter       // constraints and if  someone enter another bet is not allowed 
    
    
    
    )]

    pub bet: Account<'info,Bet>,

    #[account(mut)]
    pub player: Signer<'info>, //  Player Signer for this account

    pub system_program: Program<'info,System> // system program  


 }

