use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol paused")]
    ProtocolPaused,
    #[msg["Pool is paused"]]
    PoolPaused,
    #[msg("Amount over balance")]
    AmountOverBalance,
    #[msg["Invalid order id"]]
    InvalidOrderId,
    #[msg["Not reach unstake time"]]
    NotMatchStakeDays,
    #[msg["Invalid admin"]]
    InvalidAdmin,
    #[msg["Invalid operator"]]
    InvalidOperator,
    #[msg["Not visible stake amount"]]
    NotVisibleStakeAmount,
    #[msg["Not stake time"]]
    NotStakeTime,
    #[msg["Reward exceed"]]
    RewardExceed,
    #[msg["Not start claim reward"]]
    NotStartClaimReward,
    #[msg("Arithmetic Error (overflow/underflow)")]
    ArithmeticError,
    #[msg("Not reach unstake time")]
    NotReachUnstakeTime,
    #[msg("Start time need LT end time")]
    StartTimeNeedLTEndTime,
    #[msg("Input stake amount not equal order amount")]
    InputStakeAmountNotEqualOrderAmount,
    #[msg("Input reward amount not equal order reward")]
    InputRewardAmountNotEqualOrderReward,
    #[msg("Invalid user")]
    InvalidUser,
    #[msg("Duration need gt 0")]
    DurationNeedGT0,
    #[msg("Duration must be multi days")]
    DurationMustBeMultiDays,
    #[msg("Start time need GT 0")]
    StartTimeNeedGT0,
    #[msg("Pool already start stake")]
    PoolAlreadyStartStake,
    #[msg("Order already unstake")]
    OrderAlreadyUnstake,
    #[msg("Invalid pool id")]
    InvalidPoolId,
    #[msg("Claim reward check fail")]
    ClaimRewardCheckFail,
    #[msg("Withdraw amount check fail")]
    WithdrawAmountCheckFail,
    #[msg("Start time need gte now")]
    StartTimeNeedGTENow,
    #[msg("Two caps need gt 0")]
    TwoCapsNeedGT0,
    #[msg("Passed days need gt 0")]
    PassedDaysNeedGT0,
    #[msg("Stake amount need GT 0")]
    StakeAmountNeedGT0,
    #[msg("Reward need GT 0")]
    RewardNeedGT0,
    #[msg("Restore stake amount check fail")]
    RestoreStakeCheckFail,
    #[msg("Restore reward amount check fail")]
    RestoreRewardCheckFail,
    #[msg("Restore stake amount over cap")]
    RestoreStakeOverCap,   
     #[msg("Restore reward amount over cap")]
    RestoreRewardOverCap,
    
}
