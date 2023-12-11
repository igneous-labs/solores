use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] = [233, 45, 62, 40, 35, 129, 48, 72];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoenixV1FulfillmentConfig {
    pub pubkey: Pubkey,
    pub phoenix_program_id: Pubkey,
    pub phoenix_log_authority: Pubkey,
    pub phoenix_market: Pubkey,
    pub phoenix_base_vault: Pubkey,
    pub phoenix_quote_vault: Pubkey,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
#[derive(Clone, Debug, PartialEq)]
pub struct PhoenixV1FulfillmentConfigAccount(pub PhoenixV1FulfillmentConfig);
impl PhoenixV1FulfillmentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PhoenixV1FulfillmentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PHOENIX_V1_FULFILLMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM: [u8; 8] =
    [65, 160, 197, 112, 239, 168, 103, 185];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SerumV3FulfillmentConfig {
    pub pubkey: Pubkey,
    pub serum_program_id: Pubkey,
    pub serum_market: Pubkey,
    pub serum_request_queue: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_base_vault: Pubkey,
    pub serum_quote_vault: Pubkey,
    pub serum_open_orders: Pubkey,
    pub serum_signer_nonce: u64,
    pub market_index: u16,
    pub fulfillment_type: SpotFulfillmentType,
    pub status: SpotFulfillmentConfigStatus,
    pub padding: [u8; 4],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SerumV3FulfillmentConfigAccount(pub SerumV3FulfillmentConfig);
impl SerumV3FulfillmentConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SerumV3FulfillmentConfig::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SERUM_V3_FULFILLMENT_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INSURANCE_FUND_STAKE_ACCOUNT_DISCM: [u8; 8] = [110, 202, 14, 42, 95, 73, 90, 95];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsuranceFundStake {
    pub authority: Pubkey,
    pub if_shares: u128,
    pub last_withdraw_request_shares: u128,
    pub if_base: u128,
    pub last_valid_ts: i64,
    pub last_withdraw_request_value: u64,
    pub last_withdraw_request_ts: i64,
    pub cost_basis: i64,
    pub market_index: u16,
    pub padding: [u8; 14],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundStakeAccount(pub InsuranceFundStake);
impl InsuranceFundStakeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INSURANCE_FUND_STAKE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INSURANCE_FUND_STAKE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InsuranceFundStake::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INSURANCE_FUND_STAKE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const PERP_MARKET_ACCOUNT_DISCM: [u8; 8] = [10, 223, 12, 44, 107, 245, 55, 247];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PerpMarket {
    pub pubkey: Pubkey,
    pub amm: AMM,
    pub pnl_pool: PoolBalance,
    pub name: [u8; 32],
    pub insurance_claim: InsuranceClaim,
    pub unrealized_pnl_max_imbalance: u64,
    pub expiry_ts: i64,
    pub expiry_price: i64,
    pub next_fill_record_id: u64,
    pub next_funding_rate_record_id: u64,
    pub next_curve_record_id: u64,
    pub imf_factor: u32,
    pub unrealized_pnl_imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
    pub unrealized_pnl_initial_asset_weight: u32,
    pub unrealized_pnl_maintenance_asset_weight: u32,
    pub number_of_users_with_base: u32,
    pub number_of_users: u32,
    pub market_index: u16,
    pub status: MarketStatus,
    pub contract_type: ContractType,
    pub contract_tier: ContractTier,
    pub padding1: bool,
    pub quote_spot_market_index: u16,
    pub padding: [u8; 48],
}
#[derive(Clone, Debug, PartialEq)]
pub struct PerpMarketAccount(pub PerpMarket);
impl PerpMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERP_MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERP_MARKET_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PerpMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERP_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SPOT_MARKET_ACCOUNT_DISCM: [u8; 8] = [100, 177, 8, 107, 168, 65, 65, 39];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpotMarket {
    pub pubkey: Pubkey,
    pub oracle: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub name: [u8; 32],
    pub historical_oracle_data: HistoricalOracleData,
    pub historical_index_data: HistoricalIndexData,
    pub revenue_pool: PoolBalance,
    pub spot_fee_pool: PoolBalance,
    pub insurance_fund: InsuranceFund,
    pub total_spot_fee: u128,
    pub deposit_balance: u128,
    pub borrow_balance: u128,
    pub cumulative_deposit_interest: u128,
    pub cumulative_borrow_interest: u128,
    pub total_social_loss: u128,
    pub total_quote_social_loss: u128,
    pub withdraw_guard_threshold: u64,
    pub max_token_deposits: u64,
    pub deposit_token_twap: u64,
    pub borrow_token_twap: u64,
    pub utilization_twap: u64,
    pub last_interest_ts: u64,
    pub last_twap_ts: u64,
    pub expiry_ts: i64,
    pub order_step_size: u64,
    pub order_tick_size: u64,
    pub min_order_size: u64,
    pub max_position_size: u64,
    pub next_fill_record_id: u64,
    pub next_deposit_record_id: u64,
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
    pub decimals: u32,
    pub market_index: u16,
    pub orders_enabled: bool,
    pub oracle_source: OracleSource,
    pub status: MarketStatus,
    pub asset_tier: AssetTier,
    pub padding1: [u8; 6],
    pub flash_loan_amount: u64,
    pub flash_loan_initial_token_amount: u64,
    pub total_swap_fee: u64,
    pub padding: [u8; 56],
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotMarketAccount(pub SpotMarket);
impl SpotMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SPOT_MARKET_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SPOT_MARKET_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SpotMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SPOT_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State {
    pub admin: Pubkey,
    pub whitelist_mint: Pubkey,
    pub discount_mint: Pubkey,
    pub signer: Pubkey,
    pub srm_vault: Pubkey,
    pub perp_fee_structure: FeeStructure,
    pub spot_fee_structure: FeeStructure,
    pub oracle_guard_rails: OracleGuardRails,
    pub number_of_authorities: u64,
    pub number_of_sub_accounts: u64,
    pub lp_cooldown_time: u64,
    pub liquidation_margin_buffer_ratio: u32,
    pub settlement_duration: u16,
    pub number_of_markets: u16,
    pub number_of_spot_markets: u16,
    pub signer_nonce: u8,
    pub min_perp_auction_duration: u8,
    pub default_market_order_time_in_force: u8,
    pub default_spot_auction_duration: u8,
    pub exchange_status: u8,
    pub liquidation_duration: u8,
    pub initial_pct_to_liquidate: u16,
    pub padding: [u8; 14],
}
#[derive(Clone, Debug, PartialEq)]
pub struct StateAccount(pub State);
impl StateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STATE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(State::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_ACCOUNT_DISCM: [u8; 8] = [159, 117, 95, 227, 239, 151, 58, 236];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    pub authority: Pubkey,
    pub delegate: Pubkey,
    pub name: [u8; 32],
    pub spot_positions: [SpotPosition; 8],
    pub perp_positions: [PerpPosition; 8],
    pub orders: [Order; 32],
    pub last_add_perp_lp_shares_ts: i64,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub total_social_loss: u64,
    pub settled_perp_pnl: i64,
    pub cumulative_spot_fees: i64,
    pub cumulative_perp_funding: i64,
    pub liquidation_margin_freed: u64,
    pub last_active_slot: u64,
    pub next_order_id: u32,
    pub max_margin_ratio: u32,
    pub next_liquidation_id: u16,
    pub sub_account_id: u16,
    pub status: UserStatus,
    pub is_margin_trading_enabled: bool,
    pub idle: bool,
    pub open_orders: u8,
    pub has_open_order: bool,
    pub open_auctions: u8,
    pub has_open_auction: bool,
    pub padding: [u8; 21],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserAccount(pub User);
impl UserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    USER_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(User::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_STATS_ACCOUNT_DISCM: [u8; 8] = [176, 223, 136, 27, 122, 79, 32, 227];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserStats {
    pub authority: Pubkey,
    pub referrer: Pubkey,
    pub fees: UserFees,
    pub next_epoch_ts: i64,
    pub maker_volume30d: u64,
    pub taker_volume30d: u64,
    pub filler_volume30d: u64,
    pub last_maker_volume30d_ts: i64,
    pub last_taker_volume30d_ts: i64,
    pub last_filler_volume30d_ts: i64,
    pub if_staked_quote_asset_amount: u64,
    pub number_of_sub_accounts: u16,
    pub number_of_sub_accounts_created: u16,
    pub is_referrer: bool,
    pub padding: [u8; 51],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserStatsAccount(pub UserStats);
impl UserStatsAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_STATS_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    USER_STATS_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UserStats::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_STATS_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_NAME_ACCOUNT_DISCM: [u8; 8] = [105, 133, 170, 110, 52, 42, 28, 182];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReferrerName {
    pub authority: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerNameAccount(pub ReferrerName);
impl ReferrerNameAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_NAME_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REFERRER_NAME_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReferrerName::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_NAME_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
