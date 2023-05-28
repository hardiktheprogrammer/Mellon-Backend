# Prediction-safroon

Prediction safroon is Crypto Prediction Market Place. The contract allows users to create bets and participate in them by making predictions. The outcome of the bets is determined based on the comparison of the predicted prices with the actual price obtained from a Pyth price feed.

## Overview

- **create_master**: Creates a master account used for managing bets.
- **create_bet:** Creates a new bet and transfers the specified amount from the player to the bet account.
- **enter_bet**: Allows a player to enter a bet by providing their prediction.
- **claim_bet**: Determines the outcome of a bet based on the actual price obtained from the Pyth price feed and distributes the prize to the winning player(s).
- **close_bet**: Closes a bet and performs any necessary cleanup.

## Tech Stack

<div align="center">

![pyth](https://www.gitbook.com/cdn-cgi/image/width=256,height=40,fit=contain,dpr=1.5,format=auto/https%3A%2F%2F828262926-files.gitbook.io%2F~%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F-Me_G5lHljRT4SyKcEA9%252Flogo%252Fh0vEEm11UGmb8yrr7H18%252Flogo.png%3Falt%3Dmedia%26token%3D8e5ce850-b4cd-4670-93c9-a8ebf0077a57)

</div>

## How It work

<h2 align="center">Create Master</h2>

```rust
pub fn create_master(_ctx: Context<CreateMaster>) -> Result<()> {
      Ok(())
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
```

<h2 align="center">Create a bet </h2>

```rust
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
            system_program::transfer {
                from: ctx.accounts.player.to_account_info(),
                to: bet.to_account_info(), //
            },
        ),
        bet.amount,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateBet<'info> {
    #[account(
        init,
        payer=player,
        space=8+8+32+8+8+32+8+1+32+8+1,
        seeds=[BET_SEED, &(master.last_bet_id + 1).to_le_bytes()],
        bump
    )]
    pub bet: Account<'info, Bet>, // Bet Account
    #[account(mut, seeds=[MASTER_SEED],bump)]
    pub master: Account<'info, Master>,
    #[account(mut)]
    pub player: Signer<'info>, // Player
    pub system_program: Program<'info, System>, // System program
}
```

<h2 align="center">Enter a bet </h2>

```rust
pub fn enter_bet(ctx: Context<EnterBet>, price: f64) -> Result<()> {
    // enter the bet and the price
    let bet = &mut ctx.accounts.bet; // function for prediction B
    bet.prediction_b = Some(BettingPrediction {
        player: ctx.accounts.player.key(),
        price,
    });
    bet.state = BetState::Started;
    // transfer the amount to the bet PDA
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.player.to_account_info(),
                to: bet.to_account_info(),
            },
        ),
        bet.amount,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct EnterBet<'info> {
    #[account(
     mut,
     seeds = [BET_SEED, &bet.id.to_le_bytes()], // bets seeds are the bet seed bytes
     bump,
     constraint = validate_enter_bet(&*bet) @ BetError::CannotEnter       // constraints and if  someone enter another bet is not allowed
     )]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub player: Signer<'info>, //  Player Signer for this account
    pub system_program: Program<'info, System> // system program
}

```

<h2 align="center">Claim the bet </h2>

```rust
pub fn claim_bet(ctx: Context<ClaimBet>) -> Result<()> { //
    let bet = &mut ctx.amounts.bet; //
    let price = bet.amount.checkout_mut(2).unwrap();//
    **bet.to_account_info().try_borrow_mut_lamports()?; -= prize; // lamports is equal to the prize
    let pyth_account_info = &ctx.accounts.pyth; // pyth price account info
    let feed = load_price_from_account_info(pyth_account_info)//
      .map_err(|_| error!(BetError::NOPythAccount))?; //
    let price_data = feed.get_price_unchecked(); // price check
    require!(price_data.price <= f64::Max as i64, BetError::PriceIsHigh);// checking the price is too Hight
    let pyth_price = price_data.price as f64; //
    msg!("pyth price is: {}",  pyth.price);
    let multiplier = 10f64.powi(-price_data.expo);  // the multiplier variable assisge a value that is equal to 10 raised to the power of the negated value of price_data.expo.  
    let adjusted_player_a = bet.prediction_a.price * multiplier;
    let adjusted_player_b = bet.prediction_b.as_ref().unwrap().price * multiplier;
    msg!("adjusted player A: {}", adjusted_player_a);
    msg!("adjusted player B: {}", adjusted_player_b);

    if (pyth_price - adjusted_player_a).abs() < (pyth_price - adjusted_player_b).abs() {
     let prize = calculate_prize();
     msg!("🤑 Winner is Player A, Sending {} lamports", prize);
     bet.state = BetState::PlayerAWon;
      **ctx // open connection
          .accounts
          .player_a
          .to_account_info()
          .try_borrow_mut_lamports()?; =+ prize;
}   else if abs_player_b < abs_player_a {
     msg!("🤑 Winner is Player B, Sending {} lamports", prize);
     bet.state = BetState::PlayerBWon;
     **ctx
      .accounts
      .player_b
      .to_account_info()
      .try_borrow_mut_lamports()?; =+ prize; // Try Borrow lamports

//   check if player A is the  Winner or Player B
// setup Draw Match  
} else {
  let draw_match = bet.amount; //real Price
  msg!("Draw! Sending bet:{} lamports", draw_amount);
  bet.state = BetState::Draw;
  // for player A
  **ctx
      .accounts
      .player_a
      .to_account_info()
      .try_borrow_mut_lamports()?; =+ draw_match; // Try Borrow Lamports
//  for Player B
  **ctx
      .accounts
      .player_b
      .to_account_info()
      .try_borrow_mut_lamports()?; =+ draw_match; // Try Borrow Lamports
  }
  Ok(()) // Ok
}

#[derive(Accounts)]
pub struct ClaimBet<'info> {
    #[account(
        mut,
        seeds=[BET_SEED, &bet.id.to_le_bytes()],
        bump,
        constraint = validate_claim_bet(&*bet) @ BetError::CannotClaim
     )]                             
    pub bet: Account<'info,Bet>, //  bet account
    #[account(address = bet.pyth_price_key @ BetError::InvalidKey)] // pyth oracel account
    pub pyth: AccountInfo<'info>, // pyth account info for the account
    #[account(mut, address = bet.prediction_b.as_ref().unwrap().player)] // both accounts
    pub player_b: AccountInfo<'info>, // Player B account info
    #[account(mut, address = bet.prediction_a.player)] // Player a
    pub player_a: AccountInfo<'info>, // player A account Info
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>
}
```

<h2 align="center">Close the bet </h2>

```rust
pub fn close_bet(_ctx: Context<CloseBet>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CloseBet<'info> {
    #[account( // mutable
    mut,
    seeds = [BET_SEED,&bet.id.to_le_bytes()],
    bump,
    close = player, //
    constraint = validate_close_bet(&*bet, player.key()) @ BetError::CannotClose

)] //
  pub bet: Account<'info,Bet>,
  #[account(mut)]
  pub player: Signer<'info>,
  pub system_program: Program<'info, System>,
}
```

## Demo

## Architecture

## Developer Contribution Guide

## Introduction for Contribution

### Get Started

### Collaboration

### Our partners

# Our Community

- Non-Technical discussion:

  - Telegram Group

- Technical discussion:
  - Discord Server
