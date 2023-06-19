use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct OrderParams {
    pub order_type: OrderType,
    pub market_type: MarketType,
    pub direction: PositionDirection,
    pub user_order_id: u8,
    pub base_asset_amount: u64,
    pub price: u64,
    pub market_index: u16,
    pub reduce_only: bool,
    pub post_only: PostOnlyParam,
    pub immediate_or_cancel: bool,
    pub max_ts: Option<i64>,
    pub trigger_price: Option<u64>,
    pub trigger_condition: OrderTriggerCondition,
    pub oracle_price_offset: Option<i32>,
    pub auction_duration: Option<u8>,
    pub auction_start_price: Option<i64>,
    pub auction_end_price: Option<i64>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ModifyOrderParams {
    pub direction: Option<PositionDirection>,
    pub base_asset_amount: Option<u64>,
    pub price: Option<u64>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<PostOnlyParam>,
    pub immediate_or_cancel: Option<bool>,
    pub max_ts: Option<i64>,
    pub trigger_price: Option<u64>,
    pub trigger_condition: Option<OrderTriggerCondition>,
    pub oracle_price_offset: Option<i32>,
    pub auction_duration: Option<u8>,
    pub auction_start_price: Option<i64>,
    pub auction_end_price: Option<i64>,
    pub policy: Option<ModifyOrderPolicy>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiquidatePerpRecord {
    pub market_index: u16,
    pub oracle_price: i64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    pub lp_shares: u64,
    pub fill_record_id: u64,
    pub user_order_id: u32,
    pub liquidator_order_id: u32,
    pub liquidator_fee: u64,
    pub if_fee: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiquidateSpotRecord {
    pub asset_market_index: u16,
    pub asset_price: i64,
    pub asset_transfer: u128,
    pub liability_market_index: u16,
    pub liability_price: i64,
    pub liability_transfer: u128,
    pub if_fee: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiquidateBorrowForPerpPnlRecord {
    pub perp_market_index: u16,
    pub market_oracle_price: i64,
    pub pnl_transfer: u128,
    pub liability_market_index: u16,
    pub liability_price: i64,
    pub liability_transfer: u128,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiquidatePerpPnlForDepositRecord {
    pub perp_market_index: u16,
    pub market_oracle_price: i64,
    pub pnl_transfer: u128,
    pub asset_market_index: u16,
    pub asset_price: i64,
    pub asset_transfer: u128,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct PerpBankruptcyRecord {
    pub market_index: u16,
    pub pnl: i128,
    pub if_payment: u128,
    pub clawback_user: Option<Pubkey>,
    pub clawback_user_payment: Option<u128>,
    pub cumulative_funding_rate_delta: i128,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SpotBankruptcyRecord {
    pub market_index: u16,
    pub borrow_amount: u128,
    pub if_payment: u128,
    pub cumulative_deposit_interest_delta: u128,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct HistoricalOracleData {
    pub last_oracle_price: i64,
    pub last_oracle_conf: u64,
    pub last_oracle_delay: i64,
    pub last_oracle_price_twap: i64,
    pub last_oracle_price_twap5min: i64,
    pub last_oracle_price_twap_ts: i64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct HistoricalIndexData {
    pub last_index_bid_price: u64,
    pub last_index_ask_price: u64,
    pub last_index_price_twap: u64,
    pub last_index_price_twap5min: u64,
    pub last_index_price_twap_ts: i64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct InsuranceClaim {
    pub revenue_withdraw_since_last_settle: i64,
    pub max_revenue_withdraw_per_period: u64,
    pub quote_max_insurance: u64,
    pub quote_settled_insurance: u64,
    pub last_revenue_withdraw_ts: i64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct PoolBalance {
    pub scaled_balance: u128,
    pub market_index: u16,
    pub padding: [u8; 6],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct AMM {
    pub oracle: Pubkey,
    pub historical_oracle_data: HistoricalOracleData,
    pub base_asset_amount_per_lp: i128,
    pub quote_asset_amount_per_lp: i128,
    pub fee_pool: PoolBalance,
    pub base_asset_reserve: u128,
    pub quote_asset_reserve: u128,
    pub concentration_coef: u128,
    pub min_base_asset_reserve: u128,
    pub max_base_asset_reserve: u128,
    pub sqrt_k: u128,
    pub peg_multiplier: u128,
    pub terminal_quote_asset_reserve: u128,
    pub base_asset_amount_long: i128,
    pub base_asset_amount_short: i128,
    pub base_asset_amount_with_amm: i128,
    pub base_asset_amount_with_unsettled_lp: i128,
    pub max_open_interest: u128,
    pub quote_asset_amount: i128,
    pub quote_entry_amount_long: i128,
    pub quote_entry_amount_short: i128,
    pub quote_break_even_amount_long: i128,
    pub quote_break_even_amount_short: i128,
    pub user_lp_shares: u128,
    pub last_funding_rate: i64,
    pub last_funding_rate_long: i64,
    pub last_funding_rate_short: i64,
    pub last24h_avg_funding_rate: i64,
    pub total_fee: i128,
    pub total_mm_fee: i128,
    pub total_exchange_fee: u128,
    pub total_fee_minus_distributions: i128,
    pub total_fee_withdrawn: u128,
    pub total_liquidation_fee: u128,
    pub cumulative_funding_rate_long: i128,
    pub cumulative_funding_rate_short: i128,
    pub total_social_loss: u128,
    pub ask_base_asset_reserve: u128,
    pub ask_quote_asset_reserve: u128,
    pub bid_base_asset_reserve: u128,
    pub bid_quote_asset_reserve: u128,
    pub last_oracle_normalised_price: i64,
    pub last_oracle_reserve_price_spread_pct: i64,
    pub last_bid_price_twap: u64,
    pub last_ask_price_twap: u64,
    pub last_mark_price_twap: u64,
    pub last_mark_price_twap5min: u64,
    pub last_update_slot: u64,
    pub last_oracle_conf_pct: u64,
    pub net_revenue_since_last_funding: i64,
    pub last_funding_rate_ts: i64,
    pub funding_period: i64,
    pub order_step_size: u64,
    pub order_tick_size: u64,
    pub min_order_size: u64,
    pub max_position_size: u64,
    pub volume24h: u64,
    pub long_intensity_volume: u64,
    pub short_intensity_volume: u64,
    pub last_trade_ts: i64,
    pub mark_std: u64,
    pub oracle_std: u64,
    pub last_mark_price_twap_ts: i64,
    pub base_spread: u32,
    pub max_spread: u32,
    pub long_spread: u32,
    pub short_spread: u32,
    pub long_intensity_count: u32,
    pub short_intensity_count: u32,
    pub max_fill_reserve_fraction: u16,
    pub max_slippage_ratio: u16,
    pub curve_update_intensity: u8,
    pub amm_jit_intensity: u8,
    pub oracle_source: OracleSource,
    pub last_oracle_valid: bool,
    pub target_base_asset_amount_per_lp: i32,
    pub padding: [u8; 44],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct InsuranceFund {
    pub vault: Pubkey,
    pub total_shares: u128,
    pub user_shares: u128,
    pub shares_base: u128,
    pub unstaking_period: i64,
    pub last_revenue_settle_ts: i64,
    pub revenue_settle_period: i64,
    pub total_factor: u32,
    pub user_factor: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct OracleGuardRails {
    pub price_divergence: PriceDivergenceGuardRails,
    pub validity: ValidityGuardRails,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct PriceDivergenceGuardRails {
    pub mark_oracle_divergence_numerator: u64,
    pub mark_oracle_divergence_denominator: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ValidityGuardRails {
    pub slots_before_stale_for_amm: i64,
    pub slots_before_stale_for_margin: i64,
    pub confidence_interval_max_size: u64,
    pub too_volatile_ratio: i64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct FeeStructure {
    pub fee_tiers: [FeeTier; 10],
    pub filler_reward_structure: OrderFillerRewardStructure,
    pub referrer_reward_epoch_upper_bound: u64,
    pub flat_filler_fee: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct FeeTier {
    pub fee_numerator: u32,
    pub fee_denominator: u32,
    pub maker_rebate_numerator: u32,
    pub maker_rebate_denominator: u32,
    pub referrer_reward_numerator: u32,
    pub referrer_reward_denominator: u32,
    pub referee_fee_numerator: u32,
    pub referee_fee_denominator: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct OrderFillerRewardStructure {
    pub reward_numerator: u32,
    pub reward_denominator: u32,
    pub time_based_reward_lower_bound: u128,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct UserFees {
    pub total_fee_paid: u64,
    pub total_fee_rebate: u64,
    pub total_token_discount: u64,
    pub total_referee_discount: u64,
    pub total_referrer_reward: u64,
    pub current_epoch_referrer_reward: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SpotPosition {
    pub scaled_balance: u64,
    pub open_bids: i64,
    pub open_asks: i64,
    pub cumulative_deposits: i64,
    pub market_index: u16,
    pub balance_type: SpotBalanceType,
    pub open_orders: u8,
    pub padding: [u8; 4],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct PerpPosition {
    pub last_cumulative_funding_rate: i64,
    pub base_asset_amount: i64,
    pub quote_asset_amount: i64,
    pub quote_break_even_amount: i64,
    pub quote_entry_amount: i64,
    pub open_bids: i64,
    pub open_asks: i64,
    pub settled_pnl: i64,
    pub lp_shares: u64,
    pub last_base_asset_amount_per_lp: i64,
    pub last_quote_asset_amount_per_lp: i64,
    pub remainder_base_asset_amount: i32,
    pub market_index: u16,
    pub open_orders: u8,
    pub padding: [u8; 1],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Order {
    pub slot: u64,
    pub price: u64,
    pub base_asset_amount: u64,
    pub base_asset_amount_filled: u64,
    pub quote_asset_amount_filled: u64,
    pub trigger_price: u64,
    pub auction_start_price: i64,
    pub auction_end_price: i64,
    pub max_ts: i64,
    pub oracle_price_offset: i32,
    pub order_id: u32,
    pub market_index: u16,
    pub status: OrderStatus,
    pub order_type: OrderType,
    pub market_type: MarketType,
    pub user_order_id: u8,
    pub existing_position_direction: PositionDirection,
    pub direction: PositionDirection,
    pub reduce_only: bool,
    pub post_only: bool,
    pub immediate_or_cancel: bool,
    pub trigger_condition: OrderTriggerCondition,
    pub auction_duration: u8,
    pub padding: [u8; 3],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SwapDirection {
    Add,
    Remove,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ModifyOrderId {
    UserOrderId(u8),
    OrderId(u32),
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum PositionDirection {
    Long,
    Short,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SpotFulfillmentType {
    SerumV3,
    Match,
    PhoenixV1,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum PostOnlyParam {
    None,
    MustPostOnly,
    TryPostOnly,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ModifyOrderPolicy {
    TryModify,
    MustModify,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SwapReduceOnly {
    In,
    Out,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum TwapPeriod {
    FundingPeriod,
    FiveMin,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum LiquidationMultiplierType {
    Discount,
    Premium,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum MarginRequirementType {
    Initial,
    Maintenance,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OracleValidity {
    Invalid,
    TooVolatile,
    TooUncertain,
    StaleForMargin,
    InsufficientDataPoints,
    StaleForAmm,
    Valid,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DriftAction {
    UpdateFunding,
    SettlePnl,
    TriggerOrder,
    FillOrderMatch,
    FillOrderAmm,
    Liquidate,
    MarginCalc,
    UpdateTwap,
    UpdateAmmCurve,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum PositionUpdateType {
    Open,
    Increase,
    Reduce,
    Close,
    Flip,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DepositExplanation {
    None,
    Transfer,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum DepositDirection {
    Deposit,
    Withdraw,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OrderAction {
    Place,
    Cancel,
    Fill,
    Trigger,
    Expire,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OrderActionExplanation {
    None,
    InsufficientFreeCollateral,
    OraclePriceBreachedLimitPrice,
    MarketOrderFilledToLimitPrice,
    OrderExpired,
    Liquidation,
    OrderFilledWithAmm,
    OrderFilledWithAmmJit,
    OrderFilledWithMatch,
    OrderFilledWithMatchJit,
    MarketExpired,
    RiskingIncreasingOrder,
    ReduceOnlyOrderIncreasedPosition,
    OrderFillWithSerum,
    NoBorrowLiquidity,
    OrderFillWithPhoenix,
    OrderFilledWithAmmJitLpSplit,
    OrderFilledWithLpJit,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum LPAction {
    AddLiquidity,
    RemoveLiquidity,
    SettleLiquidity,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum LiquidationType {
    LiquidatePerp,
    LiquidateSpot,
    LiquidateBorrowForPerpPnl,
    LiquidatePerpPnlForDeposit,
    PerpBankruptcy,
    SpotBankruptcy,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SettlePnlExplanation {
    None,
    ExpiredPosition,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum StakeAction {
    Stake,
    UnstakeRequest,
    UnstakeCancelRequest,
    Unstake,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum PerpFulfillmentMethod {
    Amm(Option<u64>),
    Match(Pubkey, u16),
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SpotFulfillmentMethod {
    ExternalMarket,
    Match,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OracleSource {
    Pyth,
    Switchboard,
    QuoteAsset,
    Pyth1K,
    Pyth1M,
    PythStableCoin,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum MarketStatus {
    Initialized,
    Active,
    FundingPaused,
    AmmPaused,
    FillPaused,
    WithdrawPaused,
    ReduceOnly,
    Settlement,
    Delisted,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ContractType {
    Perpetual,
    Future,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ContractTier {
    A,
    B,
    C,
    Speculative,
    Isolated,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum AMMLiquiditySplit {
    ProtocolOwned,
    LpOwned,
    Shared,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SpotBalanceType {
    Deposit,
    Borrow,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum SpotFulfillmentConfigStatus {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum AssetTier {
    Collateral,
    Protected,
    Cross,
    Isolated,
    Unlisted,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ExchangeStatus {
    DepositPaused,
    WithdrawPaused,
    AmmPaused,
    FillPaused,
    LiqPaused,
    FundingPaused,
    SettlePnlPaused,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum UserStatus {
    Active,
    BeingLiquidated,
    Bankrupt,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum AssetType {
    Base,
    Quote,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OrderStatus {
    Init,
    Open,
    Filled,
    Canceled,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OrderType {
    Market,
    Limit,
    TriggerMarket,
    TriggerLimit,
    Oracle,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum OrderTriggerCondition {
    Above,
    Below,
    TriggeredAbove,
    TriggeredBelow,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum MarketType {
    Spot,
    Perp,
}
