use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, Transfer};
use crate::error::ErrorCode;
use crate::program::BwbStake;

pub mod error;

declare_id!("EuwAXFYeeJMAQZWWTBnqjGdKrh1UWzsnAno7epsLTz8U");

#[program]
pub mod bwb_stake {
    use super::*;

    const ONE_DAY: u64 = 86400;

    pub fn initialize(
        ctx: Context<Initialize>, 
        cosigner: Pubkey,
        admin: Pubkey,
        receiver: Pubkey,
        operator: Pubkey,
        pool_admin: Pubkey
    ) -> Result<()> {
        msg!("Instruction: Initialize");
        let admin_info = &mut ctx.accounts.admin_info;
        admin_info.cosigner = cosigner;
        admin_info.admin = admin;
        admin_info.receiver = receiver;
        admin_info.operator = operator;
        admin_info.pool_admin = pool_admin;

        admin_info.stake_token_mint = ctx.accounts.stake_token_mint.key();

        msg!("cosigner is {:?}", admin_info.cosigner);
        msg!("admin is {:?}", admin_info.admin);
        msg!("receiver is {:?}", admin_info.receiver);
        msg!("operator is {:?}", admin_info.operator);
        msg!("pool_admin is {:?}", admin_info.pool_admin);
        msg!("stake_token_mint is {:?}", admin_info.stake_token_mint);

        Ok(())
    }

    pub fn create_new_pool(
        ctx: Context<CreateNewPool>, 
        stake_cap: u64,
        reward_cap: u64,
        stake_start_at: i64,
        stake_end_at: i64,
        duration: u64,
        duration_days: u64
    ) -> Result<()> {
        msg!("Instruction: create_new_pool");
        // check paused status
        let admin_info = &ctx.accounts.admin_info;
        require!(!admin_info.is_paused, ErrorCode::ProtocolPaused);

        let clock = Clock::get()?;
        require!(stake_start_at >= clock.unix_timestamp, ErrorCode::StartTimeNeedGTENow);
        require!(stake_end_at > stake_start_at, ErrorCode::StartTimeNeedLTEndTime);

        require!(duration > 0, ErrorCode::DurationNeedGT0);
        require!(duration == duration_days.checked_mul(ONE_DAY)
            .ok_or(ErrorCode::ArithmeticError)?, ErrorCode::DurationMustBeMultiDays
        );

        require!(stake_cap > 0 && reward_cap > 0, ErrorCode::TwoCapsNeedGT0);

        let admin_info = &mut ctx.accounts.admin_info;
        let new_pool = &mut ctx.accounts.new_pool;
        new_pool.pool_id = admin_info.next_pool_id;
        new_pool.stake_cap = stake_cap;
        new_pool.reward_cap = reward_cap;
        new_pool.stake_visible = stake_cap;
        new_pool.reward_visible = reward_cap;
        new_pool.stake_start_at = stake_start_at;
        new_pool.stake_end_at = stake_end_at;
        new_pool.duration = duration;

        admin_info.next_pool_id += 1;
        msg!("new pool_id is {:?}", new_pool.pool_id);
        msg!("stake_cap is {:?}", new_pool.stake_cap);
        msg!("reward_cap is {:?}", new_pool.reward_cap);
        msg!("stake_visible is {:?}", new_pool.stake_visible);
        msg!("reward_visible is {:?}", new_pool.reward_visible);
        msg!("stake_start_at is {:?}", new_pool.stake_start_at);
        msg!("stake_end_at is {:?}", new_pool.stake_end_at);
        msg!("duration is {:?}", new_pool.duration);

        Ok(())
    }

    pub fn update_pool(
        ctx: Context<UpdatePool>, 
        pool_id: u64,
        stake_cap: u64,
        reward_cap: u64,
        stake_start_at: i64,
        stake_end_at: i64,
        duration: u64,
        duration_days: u64
    )-> Result<()> {
        msg!("Instruction: update_pool");
        // check paused status
        let admin_info = &ctx.accounts.admin_info;
        require!(!admin_info.is_paused, ErrorCode::ProtocolPaused);

        let clock = Clock::get()?;
        require!(stake_start_at >= clock.unix_timestamp, ErrorCode::StartTimeNeedGTENow);
        require!(stake_end_at > stake_start_at, ErrorCode::StartTimeNeedLTEndTime);

        require!(duration > 0, ErrorCode::DurationNeedGT0);
        require!(duration == duration_days.checked_mul(ONE_DAY)
            .ok_or(ErrorCode::ArithmeticError)?, ErrorCode::DurationMustBeMultiDays
        );

        require!(stake_cap > 0 && reward_cap > 0, ErrorCode::TwoCapsNeedGT0);

        let pool = &mut ctx.accounts.pool;
        let clock = Clock::get()?;
        require!(clock.unix_timestamp < pool.stake_start_at, ErrorCode::PoolAlreadyStartStake);

        pool.stake_cap = stake_cap;
        pool.reward_cap = reward_cap;
        pool.stake_visible = stake_cap;
        pool.reward_visible = reward_cap;
        pool.stake_start_at = stake_start_at;
        pool.stake_end_at = stake_end_at;
        pool.duration = duration;

        msg!("pool_id is {:?}", pool_id);
        msg!("stake_cap is {:?}", pool.stake_cap);
        msg!("reward_cap is {:?}", pool.reward_cap);
        msg!("stake_visible is {:?}", pool.stake_visible);
        msg!("reward_visible is {:?}", pool.reward_visible);
        msg!("stake_start_at is {:?}", pool.stake_start_at);
        msg!("stake_end_at is {:?}", pool.stake_end_at);
        msg!("duration is {:?}", pool.duration);

        Ok(())
    }

    pub fn update_admin(ctx: Context<UpdateAdminRole>, new_admin: Pubkey) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        msg!("old admin is {:?}", admin_info.admin);
        admin_info.admin = new_admin;
        msg!("new admin is {:?}", admin_info.admin);

        Ok(())
    }

    pub fn update_receiver(ctx: Context<UpdateAdminRole>, new_receiver: Pubkey) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        msg!("old receiver is {:?}", admin_info.receiver);
        admin_info.receiver = new_receiver;

        msg!("new receiver is {:?}", admin_info.receiver);
        Ok(())
    }

    pub fn update_cosigner(ctx: Context<UpdateAdminRole>, new_cosigner: Pubkey) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        msg!("old cosigner is {:?}", admin_info.cosigner);
        admin_info.cosigner = new_cosigner;
        msg!("new cosigner is {:?}", admin_info.cosigner);
        Ok(())
    }

    pub fn update_operator(ctx: Context<UpdateAdminRole>, new_operator: Pubkey) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        msg!("old operator is {:?}", admin_info.operator);
        admin_info.operator = new_operator;
        msg!("new operator is {:?}", admin_info.operator);

        Ok(())
    }

    pub fn update_pool_admin(ctx: Context<UpdateAdminRole>, new_pool_admin: Pubkey) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        msg!("old pool_admin is {:?}", admin_info.pool_admin);
        admin_info.pool_admin = new_pool_admin;
        msg!("new pool_admin is {:?}", admin_info.pool_admin);
        Ok(())
    }

    pub fn set_admin_is_paused(ctx: Context<SetAdminIsPaused>, is_paused: bool) -> Result<()> {
        let admin_info = &mut ctx.accounts.admin_info;
        msg!("old paused is {:?}", admin_info.is_paused);
        admin_info.is_paused = is_paused;
        msg!("new paused is {:?}", admin_info.is_paused);

        Ok(())
    }

    pub fn set_pool_is_paused(ctx: Context<SetPoolIsPaused>, pool_id: u64, is_paused: bool) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        msg!("old paused is {:?}", pool.is_paused);
        pool.is_paused = is_paused;
        msg!("new paused is {:?}", pool.is_paused);

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, pool_id: u64, amount: u64) -> Result<()> {
        msg!("Instruction: Stake");
        require!(amount > 0, ErrorCode::StakeAmountNeedGT0);
        // check paused status
        let admin_info = &ctx.accounts.admin_info;
        require!(!admin_info.is_paused, ErrorCode::ProtocolPaused);
        let pool = &mut ctx.accounts.pool;
        require!(!pool.is_paused, ErrorCode::PoolPaused);
        // check user token balance
        require!(amount  <= ctx.accounts.user_token_wallet.amount, ErrorCode::AmountOverBalance);

        // check pool limit
        require!(pool_id < admin_info.next_pool_id, ErrorCode::InvalidPoolId);
        require!(amount <= pool.stake_visible, ErrorCode::NotVisibleStakeAmount);
        // check stake time limit
        let clock = Clock::get()?;
        require!(clock.unix_timestamp > pool.stake_start_at, ErrorCode::NotStakeTime);
        require!(clock.unix_timestamp < pool.stake_end_at, ErrorCode::NotStakeTime);
        
        // Transfer BWB
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_wallet.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // update order info
        let new_order = &mut ctx.accounts.new_order;
        new_order.order_id = ctx.accounts.user_info.next_order_id;
        new_order.pool_id = pool_id;
        new_order.staker = ctx.accounts.user.key();
        new_order.stake_amount = amount;

        let tmp: u128 = (pool.reward_cap as u128).checked_mul(amount as u128).ok_or(ErrorCode::ArithmeticError)?;
        new_order.reward_amount = (tmp.checked_div(pool.stake_cap as u128).ok_or(ErrorCode::ArithmeticError)?) as u64;

        new_order.start_time = clock.unix_timestamp;
        new_order.unstake_time = clock.unix_timestamp.checked_add(pool.duration as i64).ok_or(ErrorCode::ArithmeticError)?;
        

        // update user info
        let user_info = &mut ctx.accounts.user_info;
        user_info.next_order_id = user_info.next_order_id.checked_add(1).ok_or(ErrorCode::ArithmeticError)?;
        user_info.total_stake = user_info.total_stake.checked_add(amount).ok_or(ErrorCode::ArithmeticError)?;
        user_info.total_reward = user_info.total_reward.checked_add(new_order.reward_amount).ok_or(ErrorCode::ArithmeticError)?;

        // update pool info
        pool.stake_visible = pool.stake_visible.checked_sub(amount).ok_or(ErrorCode::ArithmeticError)?;
        pool.reward_visible = pool.reward_visible.checked_sub(new_order.reward_amount).ok_or(ErrorCode::ArithmeticError)?;

        // emit Stake log
        msg!("staker is {:?}", ctx.accounts.user.key());
        msg!("pool_id is {:?}", pool_id);
        msg!("order_id is {:?}", new_order.order_id);
        msg!("stake amount is {:?}",amount);
        msg!("reward amount is {:?}",new_order.reward_amount);
        msg!("unstake_time {:?}",new_order.unstake_time);

        Ok(())
    }

    pub fn unstake(
        ctx: Context<Unstake>, 
        order_id: u64,
        unstake_amount: u64,
        reward_amount: u64,
        restore_stake_amount: u64,
        restore_reward_amount: u64
    ) -> Result<()> {
        msg!("Instruction: Unstake");
        // check paused status
        let admin_info = &ctx.accounts.admin_info;
        require!(!admin_info.is_paused, ErrorCode::ProtocolPaused);
        let pool = &mut ctx.accounts.pool;
        require!(!pool.is_paused, ErrorCode::PoolPaused);
        
        // check order_id
        let user_info = &mut ctx.accounts.user_info;
        require!(order_id < user_info.next_order_id, ErrorCode::InvalidOrderId);
        
        // check order info
        let clock = Clock::get()?;
        let order = &mut ctx.accounts.order;
        
        require!(!order.is_unstake, ErrorCode::OrderAlreadyUnstake);
        // check user == order.staker
        require!(order.staker == ctx.accounts.user.key(), ErrorCode::InvalidUser);

        let mut real_time: i64 = clock.unix_timestamp;
        if clock.unix_timestamp >= order.unstake_time {
            real_time = order.unstake_time;
        }
        let passed_days = ((real_time.checked_sub(order.start_time).ok_or(ErrorCode::ArithmeticError)?) as u64).checked_div(ONE_DAY).ok_or(ErrorCode::ArithmeticError)?;
        let period_days = pool.duration.checked_div(ONE_DAY).ok_or(ErrorCode::ArithmeticError)?;
        
        // claim reward
        let current_reward = 
            ((order.reward_amount.checked_mul(passed_days).ok_or(ErrorCode::ArithmeticError)?)
            .checked_div(period_days).ok_or(ErrorCode::ArithmeticError)?)
            .checked_sub(order.claimed_reward).ok_or(ErrorCode::ArithmeticError)?;

        require!(current_reward.checked_add(order.claimed_reward).ok_or(ErrorCode::ArithmeticError)? <= order.reward_amount, ErrorCode::RewardExceed);
        msg!("order.rewardmount is {:?}",order.reward_amount);
        msg!("current_reward is {:?}", current_reward);
       
        // check input params
        require!(unstake_amount == order.stake_amount, ErrorCode::InputStakeAmountNotEqualOrderAmount);
        require!(reward_amount == current_reward, ErrorCode::InputRewardAmountNotEqualOrderReward);

        let withdraw_amount = order.stake_amount.checked_add(current_reward).ok_or(ErrorCode::ArithmeticError)?;
        msg!("withdraw_amount is {:?}", withdraw_amount);
        require!(withdraw_amount <= ctx.accounts.vault_token_account.amount, ErrorCode::AmountOverBalance);

        // update order
        order.is_unstake = true;
        order.last_claimed_time = clock.unix_timestamp;
        order.claimed_reward = order.claimed_reward.checked_add(current_reward).ok_or(ErrorCode::ArithmeticError)?;
        msg!("order.claimed_reward is {:?}", order.claimed_reward);

        // update user info
        user_info.total_claimed_reward = user_info.total_claimed_reward.checked_add(current_reward).ok_or(ErrorCode::ArithmeticError)?;
        user_info.total_stake = user_info.total_stake.checked_sub(unstake_amount).ok_or(ErrorCode::ArithmeticError)?;
        msg!("user_info.total_claimed_reward is {:?}", user_info.total_claimed_reward);

        // update pool info
        if real_time < order.unstake_time {
            let decimals: u128 = 1_000_000_000_0; // 1e10

            // remaining_days: period_days - passed_days
            let remaining_days = period_days.checked_sub(passed_days).ok_or(ErrorCode::ArithmeticError)?;
            
            // restore_ratio = (remaining_days * decimals) / period_days
            let restore_ratio = (remaining_days as u128)
                .checked_mul(decimals)
                .ok_or(ErrorCode::ArithmeticError)?
                .checked_div(period_days as u128)
                .ok_or(ErrorCode::ArithmeticError)?;
            
            // restore_stake = (unstake_amount * restore_ratio) / decimals
            let restore_stake = (unstake_amount as u128)
                .checked_mul(restore_ratio)
                .ok_or(ErrorCode::ArithmeticError)?
                .checked_div(decimals)
                .ok_or(ErrorCode::ArithmeticError)?;
            
            // restore_reward = (reward_amount * restore_ratio) / decimals
            let restore_reward = (order.reward_amount as u128)
                .checked_mul(restore_ratio)
                .ok_or(ErrorCode::ArithmeticError)?
                .checked_div(decimals)
                .ok_or(ErrorCode::ArithmeticError)?;

            let restore_stake: u64 = restore_stake.try_into().map_err(|_| ErrorCode::ArithmeticError)?;
            let restore_reward: u64 = restore_reward.try_into().map_err(|_| ErrorCode::ArithmeticError)?;

            msg!("remaining_days is {:?}", remaining_days);
            msg!("restore_ratio is {:?}", restore_ratio);
            msg!("restore_stake_amount is {:?}", restore_stake);
            msg!("restore_reward_amount is {:?}", restore_reward);

            require!(restore_stake_amount == restore_stake, ErrorCode::RestoreStakeCheckFail);
            require!(restore_reward_amount == restore_reward, ErrorCode::RestoreRewardCheckFail);

            // update pool
            msg!("before pool.stake_visible is {:?}", pool.stake_visible);
            msg!("before pool.reward_visible is {:?}", pool.reward_visible);

            let new_stake_visible = pool.stake_visible.checked_add(restore_stake).ok_or(ErrorCode::ArithmeticError)?;
            let new_reward_visible = pool.reward_visible.checked_add(restore_reward).ok_or(ErrorCode::ArithmeticError)?;
            require!(new_stake_visible <= pool.stake_cap, ErrorCode::RestoreStakeOverCap);
            require!(new_reward_visible <= pool.reward_cap, ErrorCode::RestoreRewardOverCap);

            pool.stake_visible = new_stake_visible;
            pool.reward_visible = new_reward_visible;

            msg!("after pool.stake_visible is {:?}", pool.stake_visible);
            msg!("after pool.reward_visible is {:?}", pool.reward_visible);
        } 

        let before_vault_bal = ctx.accounts.vault_token_account.amount;
        let before_user_bal = ctx.accounts.user_token_wallet.amount;
       
        // unstake and claim
        let bump = ctx.bumps.admin_info;
        let seeds = &[b"admin_info".as_ref(), &[bump]];
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.user_token_wallet.to_account_info(),
                    authority: ctx.accounts.admin_info.to_account_info(),
                },
            )
            .with_signer(&[&seeds[..]]),
            withdraw_amount,
        )?;

        ctx.accounts.vault_token_account.reload()?;
        ctx.accounts.user_token_wallet.reload()?;

        let after_vault_bal = ctx.accounts.vault_token_account.amount;
        let after_user_bal = ctx.accounts.user_token_wallet.amount;

        require!(after_user_bal.checked_sub(before_user_bal).ok_or(ErrorCode::ArithmeticError)? == withdraw_amount, ErrorCode::WithdrawAmountCheckFail);
        require!(before_vault_bal.checked_sub(after_vault_bal).ok_or(ErrorCode::ArithmeticError)? == withdraw_amount, ErrorCode::WithdrawAmountCheckFail);

        // emit Stake log
        msg!("order_id is {:?}", order.order_id);
        msg!("staker is {:?}", ctx.accounts.user.key());
        msg!("unstake amount {:?}",unstake_amount);
        msg!("order unstake status is {:?}",order.is_unstake);
        msg!("order claimed reward is {:?}",order.claimed_reward);
        Ok(())
    }
    
    pub fn claim_reward(
        ctx: Context<ClaimReward>, 
        order_id: u64,
        reward_amount: u64
    ) -> Result<()> {
        msg!("Instruction: Claim Reward");

        // check paused status
        let admin_info = &ctx.accounts.admin_info;
        require!(!admin_info.is_paused, ErrorCode::ProtocolPaused);
        let pool = &mut ctx.accounts.pool;
        require!(!pool.is_paused, ErrorCode::PoolPaused);
        
        // check order_id
        let user_info = &mut ctx.accounts.user_info;
        require!(order_id < user_info.next_order_id, ErrorCode::InvalidOrderId);
        
        // check order info
        let clock = Clock::get()?;
        let order = &mut ctx.accounts.order;
        require!(order.staker == ctx.accounts.user.key(), ErrorCode::InvalidUser);
        require!(clock.unix_timestamp >= order.start_time, ErrorCode::NotStartClaimReward);

        // claim reward
        let mut real_time: i64 = clock.unix_timestamp;
        if clock.unix_timestamp >= order.unstake_time {
            real_time = order.unstake_time;
        }
        let passed_days = ((real_time.checked_sub(order.start_time).ok_or(ErrorCode::ArithmeticError)?) as u64).checked_div(ONE_DAY).ok_or(ErrorCode::ArithmeticError)?;
        
        let period_days = pool.duration.checked_div(ONE_DAY).ok_or(ErrorCode::ArithmeticError)?;
        let reward = ((order.reward_amount.checked_mul(passed_days).ok_or(ErrorCode::ArithmeticError)?).checked_div(period_days).ok_or(ErrorCode::ArithmeticError)?).checked_sub(order.claimed_reward).ok_or(ErrorCode::ArithmeticError)?;

        require!(reward > 0, ErrorCode::RewardNeedGT0);
        //(passed_days - claimed_days)
        msg!("passed_days is {:?}", passed_days);
        msg!("period_days is {:?}", period_days);
        msg!("reward is {:?}", reward);

        require!(reward.checked_add(order.claimed_reward).ok_or(ErrorCode::ArithmeticError)? <= order.reward_amount, ErrorCode::RewardExceed);
        require!(reward <= ctx.accounts.vault_token_account.amount.into(), ErrorCode::AmountOverBalance);
        //check input params
        require!(reward_amount == reward, ErrorCode::InputRewardAmountNotEqualOrderReward);

        // update order
        order.last_claimed_time = clock.unix_timestamp;
        order.claimed_reward = order.claimed_reward.checked_add(reward).ok_or(ErrorCode::ArithmeticError)?;
        msg!("claimed_reward is {:?}", order.claimed_reward);
        // update user info
        user_info.total_claimed_reward = user_info.total_claimed_reward.checked_add(reward).ok_or(ErrorCode::ArithmeticError)?;
        msg!("total_claimed_reward is {:?}", user_info.total_claimed_reward);

        let before_vault_bal = ctx.accounts.vault_token_account.amount;
        let before_user_bal = ctx.accounts.user_token_wallet.amount;
        // claim reward
        let bump = ctx.bumps.admin_info;
        let seeds = &[b"admin_info".as_ref(), &[bump]];
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.user_token_wallet.to_account_info(),
                    authority: ctx.accounts.admin_info.to_account_info(),
                },
            )
            .with_signer(&[&seeds[..]]),
            reward ,
        )?;

        ctx.accounts.vault_token_account.reload()?;
        ctx.accounts.user_token_wallet.reload()?;
        
        let after_vault_bal = ctx.accounts.vault_token_account.amount;
        let after_user_bal = ctx.accounts.user_token_wallet.amount;

        require!(after_user_bal.checked_sub(before_user_bal).ok_or(ErrorCode::ArithmeticError)? == reward, ErrorCode::ClaimRewardCheckFail);
        require!(before_vault_bal.checked_sub(after_vault_bal).ok_or(ErrorCode::ArithmeticError)? == reward, ErrorCode::ClaimRewardCheckFail);

        msg!("order_id is {:?}", order_id);
        msg!("reward is {:?}", reward);
        msg!("claim time at is {:?}", clock.unix_timestamp);
        msg!("user total claimed reward is {:?}", user_info.total_claimed_reward);
        msg!("claimer is {:?}",ctx.accounts.user_token_wallet);

        Ok(())
    }

    pub fn withdraw_bwb_token(ctx: Context<WithdrawBWBToken>, amount: u64) -> Result<()> {

        // Transfer tokens from taker to initializer
        let bump = ctx.bumps.admin_info;
        let seeds = &[b"admin_info".as_ref(), &[bump]];

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from_token_account.to_account_info(),
                    to: ctx.accounts.to_token_account.to_account_info(),
                    authority: ctx.accounts.admin_info.to_account_info(),
                },
            )
            .with_signer(&[&seeds[..]]),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw_other_token(ctx: Context<WithdrawOtherToken>, amount: u64) -> Result<()> {

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from_token_account.to_account_info(),
                    to: ctx.accounts.to_token_account.to_account_info(),
                    authority: ctx.accounts.from_ata_owner.to_account_info(),
                }
            ),
            amount
        )?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, payer = authority, space = 8 + AdminInfo::LEN,
        seeds=[b"admin_info"],
        bump
    )]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(mut)]
    pub stake_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds=[b"vault_token_account"],
        token::mint=stake_token_mint,
        token::authority=admin_info,
        bump
    )]
    vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
    pub program: Program<'info, BwbStake>,
    #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
    pub program_data: Account<'info, ProgramData>,
    
    pub token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateNewPool<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(
        init, payer = pool_admin, space = 8 + PoolInfo::LEN,
        seeds=[b"new_pool", &admin_info.next_pool_id.to_le_bytes()],
        bump
    )]
    pub new_pool: Account<'info, PoolInfo>,

    #[account(mut, address = admin_info.pool_admin @ ErrorCode::InvalidOperator)]
    pub pool_admin: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct UpdatePool<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(
        mut,
        seeds=[b"new_pool", &pool_id.to_le_bytes()],
        bump
    )]
    pub pool: Account<'info, PoolInfo>,

    #[account(mut, address = admin_info.pool_admin @ ErrorCode::InvalidOperator)]
    pub pool_admin: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct SetPoolIsPaused<'info> {
    #[account(seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,
    #[account(mut, address = admin_info.operator @ ErrorCode::InvalidOperator)]
    pub operator: Signer<'info>,

    #[account(mut, seeds=[b"new_pool", &pool_id.to_le_bytes()], bump)]
    pub pool: Account<'info, PoolInfo>, 
}

#[derive(Accounts)]
pub struct SetAdminIsPaused<'info> {
    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,
    #[account(mut, address = admin_info.operator @ ErrorCode::InvalidOperator)]
    pub operator: Signer<'info>,
    
}

#[derive(Accounts)]
#[instruction(pool_id: u64, amount: u64)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = admin_info.cosigner)]
    pub cosigner: Signer<'info>,

    #[account(mut, seeds=[b"new_pool", &pool_id.to_le_bytes()], bump)]
    pub pool: Account<'info, PoolInfo>,

    #[account(seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(
        init_if_needed, payer = user, space = 8 + UserInfo::LEN,
        seeds=[b"user_info",user.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,
    #[account(
        init, payer = user, space = 8 + OrderInfo::LEN,
        seeds=[b"new_order", user.key().as_ref(), user_info.next_order_id.to_le_bytes().as_ref()], // order len = next_order_id, order id start from 0
        bump
    )]
    pub new_order: Account<'info, OrderInfo>,

    #[account(mut)]
    pub user_token_wallet: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"vault_token_account"],bump,
        token::mint=stake_token_mint,
        token::authority=admin_info
    )]
    vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, address = admin_info.stake_token_mint)]
    pub stake_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(order_id: u64)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = admin_info.cosigner)]
    pub cosigner: Signer<'info>,

    #[account(mut, seeds=[b"new_pool", &order.pool_id.to_le_bytes()], bump)]
    pub pool: Account<'info, PoolInfo>,

    #[account(seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(mut, seeds=[b"user_info",user.key().as_ref()],bump)]
    pub user_info: Account<'info, UserInfo>,
    #[account(
        mut, 
        seeds=[b"new_order", user.key().as_ref(), order_id.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, OrderInfo>,

    #[account(mut)]
    pub user_token_wallet: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"vault_token_account"],bump,
        token::mint=stake_token_mint,
        token::authority=admin_info
    )]
    vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, address = admin_info.stake_token_mint)]
    pub stake_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(order_id: u64)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = admin_info.cosigner)]
    pub cosigner: Signer<'info>,

    #[account(seeds=[b"new_pool", &order.pool_id.to_le_bytes()], bump)]
    pub pool: Account<'info, PoolInfo>,

    #[account(seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(mut, seeds=[b"user_info",user.key().as_ref()],bump)]
    pub user_info: Account<'info, UserInfo>,
    #[account(
        mut, 
        seeds=[b"new_order", user.key().as_ref(), order_id.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Account<'info, OrderInfo>,

    #[account(mut)]
    pub user_token_wallet: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"vault_token_account"],bump,
        token::mint=stake_token_mint,
        token::authority=admin_info,
    )]
    vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, address = admin_info.stake_token_mint)]
    pub stake_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdminRole<'info> {

    #[account(mut, seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(mut, address = admin_info.admin @ ErrorCode::InvalidAdmin)]
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminInfo {
    pub admin: Pubkey,
    pub cosigner:Pubkey,
    pub operator:Pubkey,
    pub receiver:Pubkey,
    pub pool_admin: Pubkey,
    pub stake_token_mint: Pubkey,
    pub next_pool_id: u64,
    pub is_paused: bool,
}
impl AdminInfo {
    pub const LEN: usize = 32*6 + 8 + 1;
}

#[account]
pub struct PoolInfo {// PDA
    pub is_paused: bool,
    pub pool_id: u64,
    pub stake_cap: u64,
    pub reward_cap: u64,
    pub stake_visible: u64,// stake_cap - sum(order.stake_amount)
    pub reward_visible: u64,// reward_cap - sum(order.reward_amount)
    pub stake_start_at: i64,
    pub stake_end_at: i64,
    pub duration: u64//seconds for 1,3,6 month

}
impl PoolInfo {
    pub const LEN: usize = 1 + 8 + 16*4 + 8*3;// PDA data size
}

#[account]
pub struct UserInfo {
    pub next_order_id: u64,//order_id=0,1,2,... // stake next_order_id =>orderInfo(order_id) 
    pub total_stake:u64,
    pub total_reward: u64,
    pub total_claimed_reward: u64
}
impl UserInfo {
    pub const LEN: usize = 8 + 16*3;
}

#[account]
pub struct OrderInfo {// key(user,order_id,"new_order") => PDA
    pub order_id: u64,
    pub pool_id: u64,
    pub staker: Pubkey,
    pub stake_amount: u64,
    pub reward_amount: u64,
    pub start_time: i64,
    pub unstake_time: i64,// what time to unstake 
    pub claimed_reward: u64,
    pub is_unstake: bool,
    pub last_claimed_time: i64,
}
impl OrderInfo {
    pub const LEN: usize = 8 + 8 + 32 + 16*2 + 8*3 + 1 + 8;
}

#[derive(Accounts)]
pub struct WithdrawBWBToken<'info> {
    #[account(seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,
    #[account(mut,token::mint=admin_info.stake_token_mint)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint=admin_info.stake_token_mint,
        token::authority=admin_info.receiver
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawOtherToken<'info> {
    
    #[account(seeds=[b"admin_info"],bump)]
    pub admin_info: Account<'info, AdminInfo>,

    #[account(address = admin_info.operator)]
    pub operator: Signer<'info>,
    /// The mint to distribute.
    pub mint: Account<'info, Mint>,// BWB token address
    #[account(mut, token::mint=mint)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint=mint,
        token::authority=admin_info.receiver
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(address = from_token_account.owner)]
    pub from_ata_owner: Signer<'info>,// this program Id
    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}


