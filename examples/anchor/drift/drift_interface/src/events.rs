use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const NEW_USER_RECORD_EVENT_DISCM: [u8; 8] = [236, 186, 113, 219, 42, 51, 149, 249];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct NewUserRecord {
    ts: i64,
    user_authority: Pubkey,
    user: Pubkey,
    sub_account_id: u16,
    name: [u8; 32],
    referrer: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NewUserRecordEvent(pub NewUserRecord);
impl BorshSerialize for NewUserRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        NEW_USER_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl NewUserRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != NEW_USER_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NEW_USER_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(NewUserRecord::deserialize(buf)?))
    }
}
pub const DEPOSIT_RECORD_EVENT_DISCM: [u8; 8] = [180, 241, 218, 207, 102, 135, 44, 134];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DepositRecord {
    ts: i64,
    user_authority: Pubkey,
    user: Pubkey,
    direction: DepositDirection,
    deposit_record_id: u64,
    amount: u64,
    market_index: u16,
    oracle_price: i64,
    market_deposit_balance: u128,
    market_withdraw_balance: u128,
    market_cumulative_deposit_interest: u128,
    market_cumulative_borrow_interest: u128,
    total_deposits_after: u64,
    total_withdraws_after: u64,
    explanation: DepositExplanation,
    transfer_user: Option<Pubkey>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositRecordEvent(pub DepositRecord);
impl BorshSerialize for DepositRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        DEPOSIT_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl DepositRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEPOSIT_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositRecord::deserialize(buf)?))
    }
}
pub const SPOT_INTEREST_RECORD_EVENT_DISCM: [u8; 8] = [183, 186, 203, 186, 225, 187, 95, 130];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SpotInterestRecord {
    ts: i64,
    market_index: u16,
    deposit_balance: u128,
    cumulative_deposit_interest: u128,
    borrow_balance: u128,
    cumulative_borrow_interest: u128,
    optimal_utilization: u32,
    optimal_borrow_rate: u32,
    max_borrow_rate: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SpotInterestRecordEvent(pub SpotInterestRecord);
impl BorshSerialize for SpotInterestRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SPOT_INTEREST_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SpotInterestRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SPOT_INTEREST_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SPOT_INTEREST_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SpotInterestRecord::deserialize(buf)?))
    }
}
pub const FUNDING_PAYMENT_RECORD_EVENT_DISCM: [u8; 8] = [8, 59, 96, 20, 137, 201, 56, 95];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FundingPaymentRecord {
    ts: i64,
    user_authority: Pubkey,
    user: Pubkey,
    market_index: u16,
    funding_payment: i64,
    base_asset_amount: i64,
    user_last_cumulative_funding: i64,
    amm_cumulative_funding_long: i128,
    amm_cumulative_funding_short: i128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundingPaymentRecordEvent(pub FundingPaymentRecord);
impl BorshSerialize for FundingPaymentRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FUNDING_PAYMENT_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FundingPaymentRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FUNDING_PAYMENT_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FUNDING_PAYMENT_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FundingPaymentRecord::deserialize(buf)?))
    }
}
pub const FUNDING_RATE_RECORD_EVENT_DISCM: [u8; 8] = [68, 3, 255, 26, 133, 91, 147, 254];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FundingRateRecord {
    ts: i64,
    record_id: u64,
    market_index: u16,
    funding_rate: i64,
    funding_rate_long: i128,
    funding_rate_short: i128,
    cumulative_funding_rate_long: i128,
    cumulative_funding_rate_short: i128,
    oracle_price_twap: i64,
    mark_price_twap: u64,
    period_revenue: i64,
    base_asset_amount_with_amm: i128,
    base_asset_amount_with_unsettled_lp: i128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FundingRateRecordEvent(pub FundingRateRecord);
impl BorshSerialize for FundingRateRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FUNDING_RATE_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FundingRateRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FUNDING_RATE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FUNDING_RATE_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FundingRateRecord::deserialize(buf)?))
    }
}
pub const CURVE_RECORD_EVENT_DISCM: [u8; 8] = [101, 238, 40, 228, 70, 46, 61, 117];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CurveRecord {
    ts: i64,
    record_id: u64,
    peg_multiplier_before: u128,
    base_asset_reserve_before: u128,
    quote_asset_reserve_before: u128,
    sqrt_k_before: u128,
    peg_multiplier_after: u128,
    base_asset_reserve_after: u128,
    quote_asset_reserve_after: u128,
    sqrt_k_after: u128,
    base_asset_amount_long: u128,
    base_asset_amount_short: u128,
    base_asset_amount_with_amm: i128,
    total_fee: i128,
    total_fee_minus_distributions: i128,
    adjustment_cost: i128,
    oracle_price: i64,
    fill_record: u128,
    number_of_users: u32,
    market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CurveRecordEvent(pub CurveRecord);
impl BorshSerialize for CurveRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CURVE_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CurveRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CURVE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CURVE_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CurveRecord::deserialize(buf)?))
    }
}
pub const ORDER_RECORD_EVENT_DISCM: [u8; 8] = [104, 19, 64, 56, 89, 21, 2, 90];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct OrderRecord {
    ts: i64,
    user: Pubkey,
    order: Order,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderRecordEvent(pub OrderRecord);
impl BorshSerialize for OrderRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ORDER_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl OrderRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ORDER_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ORDER_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OrderRecord::deserialize(buf)?))
    }
}
pub const ORDER_ACTION_RECORD_EVENT_DISCM: [u8; 8] = [224, 52, 67, 71, 194, 237, 109, 1];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct OrderActionRecord {
    ts: i64,
    action: OrderAction,
    action_explanation: OrderActionExplanation,
    market_index: u16,
    market_type: MarketType,
    filler: Option<Pubkey>,
    filler_reward: Option<u64>,
    fill_record_id: Option<u64>,
    base_asset_amount_filled: Option<u64>,
    quote_asset_amount_filled: Option<u64>,
    taker_fee: Option<u64>,
    maker_fee: Option<i64>,
    referrer_reward: Option<u32>,
    quote_asset_amount_surplus: Option<i64>,
    spot_fulfillment_method_fee: Option<u64>,
    taker: Option<Pubkey>,
    taker_order_id: Option<u32>,
    taker_order_direction: Option<PositionDirection>,
    taker_order_base_asset_amount: Option<u64>,
    taker_order_cumulative_base_asset_amount_filled: Option<u64>,
    taker_order_cumulative_quote_asset_amount_filled: Option<u64>,
    maker: Option<Pubkey>,
    maker_order_id: Option<u32>,
    maker_order_direction: Option<PositionDirection>,
    maker_order_base_asset_amount: Option<u64>,
    maker_order_cumulative_base_asset_amount_filled: Option<u64>,
    maker_order_cumulative_quote_asset_amount_filled: Option<u64>,
    oracle_price: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderActionRecordEvent(pub OrderActionRecord);
impl BorshSerialize for OrderActionRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ORDER_ACTION_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl OrderActionRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ORDER_ACTION_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ORDER_ACTION_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OrderActionRecord::deserialize(buf)?))
    }
}
pub const LP_RECORD_EVENT_DISCM: [u8; 8] = [101, 22, 54, 38, 178, 13, 142, 111];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LpRecord {
    ts: i64,
    user: Pubkey,
    action: LPAction,
    n_shares: u64,
    market_index: u16,
    delta_base_asset_amount: i64,
    delta_quote_asset_amount: i64,
    pnl: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpRecordEvent(pub LpRecord);
impl BorshSerialize for LpRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LP_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LpRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LP_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LpRecord::deserialize(buf)?))
    }
}
pub const LIQUIDATION_RECORD_EVENT_DISCM: [u8; 8] = [127, 17, 0, 108, 182, 13, 231, 53];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LiquidationRecord {
    ts: i64,
    liquidation_type: LiquidationType,
    user: Pubkey,
    liquidator: Pubkey,
    margin_requirement: u128,
    total_collateral: i128,
    margin_freed: u64,
    liquidation_id: u16,
    bankrupt: bool,
    canceled_order_ids: Vec<u32>,
    liquidate_perp: LiquidatePerpRecord,
    liquidate_spot: LiquidateSpotRecord,
    liquidate_borrow_for_perp_pnl: LiquidateBorrowForPerpPnlRecord,
    liquidate_perp_pnl_for_deposit: LiquidatePerpPnlForDepositRecord,
    perp_bankruptcy: PerpBankruptcyRecord,
    spot_bankruptcy: SpotBankruptcyRecord,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidationRecordEvent(pub LiquidationRecord);
impl BorshSerialize for LiquidationRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LIQUIDATION_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LiquidationRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUIDATION_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDATION_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidationRecord::deserialize(buf)?))
    }
}
pub const SETTLE_PNL_RECORD_EVENT_DISCM: [u8; 8] = [57, 68, 105, 26, 119, 198, 213, 89];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SettlePnlRecord {
    ts: i64,
    user: Pubkey,
    market_index: u16,
    pnl: i128,
    base_asset_amount: i64,
    quote_asset_amount_after: i64,
    quote_entry_amount: i64,
    settle_price: i64,
    explanation: SettlePnlExplanation,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettlePnlRecordEvent(pub SettlePnlRecord);
impl BorshSerialize for SettlePnlRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SETTLE_PNL_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SettlePnlRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SETTLE_PNL_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_PNL_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SettlePnlRecord::deserialize(buf)?))
    }
}
pub const INSURANCE_FUND_RECORD_EVENT_DISCM: [u8; 8] = [56, 222, 215, 235, 78, 197, 99, 146];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct InsuranceFundRecord {
    ts: i64,
    spot_market_index: u16,
    perp_market_index: u16,
    user_if_factor: u32,
    total_if_factor: u32,
    vault_amount_before: u64,
    insurance_vault_amount_before: u64,
    total_if_shares_before: u128,
    total_if_shares_after: u128,
    amount: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundRecordEvent(pub InsuranceFundRecord);
impl BorshSerialize for InsuranceFundRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INSURANCE_FUND_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl InsuranceFundRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INSURANCE_FUND_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INSURANCE_FUND_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InsuranceFundRecord::deserialize(buf)?))
    }
}
pub const INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM: [u8; 8] = [68, 66, 156, 7, 216, 148, 250, 114];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct InsuranceFundStakeRecord {
    ts: i64,
    user_authority: Pubkey,
    action: StakeAction,
    amount: u64,
    market_index: u16,
    insurance_vault_amount_before: u64,
    if_shares_before: u128,
    user_if_shares_before: u128,
    total_if_shares_before: u128,
    if_shares_after: u128,
    user_if_shares_after: u128,
    total_if_shares_after: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InsuranceFundStakeRecordEvent(pub InsuranceFundStakeRecord);
impl BorshSerialize for InsuranceFundStakeRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl InsuranceFundStakeRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INSURANCE_FUND_STAKE_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InsuranceFundStakeRecord::deserialize(buf)?))
    }
}
pub const SWAP_RECORD_EVENT_DISCM: [u8; 8] = [162, 187, 123, 194, 138, 56, 250, 241];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SwapRecord {
    ts: i64,
    user: Pubkey,
    amount_out: u64,
    amount_in: u64,
    out_market_index: u16,
    in_market_index: u16,
    out_oracle_price: i64,
    in_oracle_price: i64,
    fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapRecordEvent(pub SwapRecord);
impl BorshSerialize for SwapRecordEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SWAP_RECORD_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SwapRecordEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SWAP_RECORD_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_RECORD_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapRecord::deserialize(buf)?))
    }
}
