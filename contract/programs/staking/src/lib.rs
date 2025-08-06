use anchor_lang::prelude::*;

use crate::instructions::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod helpers;

declare_id!("6ywTdo75yF3Ssur2W5BLyPrVSujcQyA5FXpZsCNU4Kzs");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, apy_rate: u64) -> Result<()> {
        instructions::initialize(ctx, apy_rate)
    }

    pub fn sol_stake(ctx: Context<SolStake>, amount: u64, duration: u64) -> Result<()> {
        instructions::sol_stake(ctx, amount, duration)
    }

    pub fn sol_unstake(ctx: Context<SolUnstake>) -> Result<()> {
        instructions::sol_unstake(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }

    pub fn config(ctx: Context<Config>, rate: u64) -> Result<()> {
        instructions::config(ctx, rate)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)
    }
}