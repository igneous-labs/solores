use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum DriftProgramIx {
    InitializeUser(InitializeUserIxArgs),
    InitializeUserStats(InitializeUserStatsIxArgs),
    InitializeReferrerName(InitializeReferrerNameIxArgs),
    Deposit(DepositIxArgs),
    Withdraw(WithdrawIxArgs),
    TransferDeposit(TransferDepositIxArgs),
    PlacePerpOrder(PlacePerpOrderIxArgs),
    CancelOrder(CancelOrderIxArgs),
    CancelOrderByUserId(CancelOrderByUserIdIxArgs),
    CancelOrders(CancelOrdersIxArgs),
    ModifyOrder(ModifyOrderIxArgs),
    ModifyOrderByUserId(ModifyOrderByUserIdIxArgs),
    PlaceAndTakePerpOrder(PlaceAndTakePerpOrderIxArgs),
    PlaceAndMakePerpOrder(PlaceAndMakePerpOrderIxArgs),
    PlaceSpotOrder(PlaceSpotOrderIxArgs),
    PlaceAndTakeSpotOrder(PlaceAndTakeSpotOrderIxArgs),
    PlaceAndMakeSpotOrder(PlaceAndMakeSpotOrderIxArgs),
    BeginSwap(BeginSwapIxArgs),
    EndSwap(EndSwapIxArgs),
    AddPerpLpShares(AddPerpLpSharesIxArgs),
    RemovePerpLpShares(RemovePerpLpSharesIxArgs),
    RemovePerpLpSharesInExpiringMarket(RemovePerpLpSharesInExpiringMarketIxArgs),
    UpdateUserName(UpdateUserNameIxArgs),
    UpdateUserCustomMarginRatio(UpdateUserCustomMarginRatioIxArgs),
    UpdateUserMarginTradingEnabled(UpdateUserMarginTradingEnabledIxArgs),
    UpdateUserDelegate(UpdateUserDelegateIxArgs),
    DeleteUser(DeleteUserIxArgs),
    FillPerpOrder(FillPerpOrderIxArgs),
    RevertFill(RevertFillIxArgs),
    FillSpotOrder(FillSpotOrderIxArgs),
    TriggerOrder(TriggerOrderIxArgs),
    ForceCancelOrders(ForceCancelOrdersIxArgs),
    UpdateUserIdle(UpdateUserIdleIxArgs),
    UpdateUserOpenOrdersCount(UpdateUserOpenOrdersCountIxArgs),
    SettlePnl(SettlePnlIxArgs),
    SettleFundingPayment(SettleFundingPaymentIxArgs),
    SettleLp(SettleLpIxArgs),
    SettleExpiredMarket(SettleExpiredMarketIxArgs),
    LiquidatePerp(LiquidatePerpIxArgs),
    LiquidateSpot(LiquidateSpotIxArgs),
    LiquidateBorrowForPerpPnl(LiquidateBorrowForPerpPnlIxArgs),
    LiquidatePerpPnlForDeposit(LiquidatePerpPnlForDepositIxArgs),
    ResolvePerpPnlDeficit(ResolvePerpPnlDeficitIxArgs),
    ResolvePerpBankruptcy(ResolvePerpBankruptcyIxArgs),
    ResolveSpotBankruptcy(ResolveSpotBankruptcyIxArgs),
    SettleRevenueToInsuranceFund(SettleRevenueToInsuranceFundIxArgs),
    UpdateFundingRate(UpdateFundingRateIxArgs),
    UpdateSpotMarketCumulativeInterest(UpdateSpotMarketCumulativeInterestIxArgs),
    UpdateAmms(UpdateAmmsIxArgs),
    UpdateSpotMarketExpiry(UpdateSpotMarketExpiryIxArgs),
    UpdateUserQuoteAssetInsuranceStake(UpdateUserQuoteAssetInsuranceStakeIxArgs),
    InitializeInsuranceFundStake(InitializeInsuranceFundStakeIxArgs),
    AddInsuranceFundStake(AddInsuranceFundStakeIxArgs),
    RequestRemoveInsuranceFundStake(RequestRemoveInsuranceFundStakeIxArgs),
    CancelRequestRemoveInsuranceFundStake(CancelRequestRemoveInsuranceFundStakeIxArgs),
    RemoveInsuranceFundStake(RemoveInsuranceFundStakeIxArgs),
    Initialize(InitializeIxArgs),
    InitializeSpotMarket(InitializeSpotMarketIxArgs),
    InitializeSerumFulfillmentConfig(InitializeSerumFulfillmentConfigIxArgs),
    UpdateSerumFulfillmentConfigStatus(UpdateSerumFulfillmentConfigStatusIxArgs),
    InitializePhoenixFulfillmentConfig(InitializePhoenixFulfillmentConfigIxArgs),
    PhoenixFulfillmentConfigStatus(PhoenixFulfillmentConfigStatusIxArgs),
    UpdateSerumVault(UpdateSerumVaultIxArgs),
    InitializePerpMarket(InitializePerpMarketIxArgs),
    DeleteInitializedPerpMarket(DeleteInitializedPerpMarketIxArgs),
    MoveAmmPrice(MoveAmmPriceIxArgs),
    UpdatePerpMarketExpiry(UpdatePerpMarketExpiryIxArgs),
    SettleExpiredMarketPoolsToRevenuePool(SettleExpiredMarketPoolsToRevenuePoolIxArgs),
    DepositIntoPerpMarketFeePool(DepositIntoPerpMarketFeePoolIxArgs),
    RepegAmmCurve(RepegAmmCurveIxArgs),
    UpdatePerpMarketAmmOracleTwap(UpdatePerpMarketAmmOracleTwapIxArgs),
    ResetPerpMarketAmmOracleTwap(ResetPerpMarketAmmOracleTwapIxArgs),
    UpdateK(UpdateKIxArgs),
    UpdatePerpMarketMarginRatio(UpdatePerpMarketMarginRatioIxArgs),
    UpdatePerpMarketMaxImbalances(UpdatePerpMarketMaxImbalancesIxArgs),
    UpdatePerpMarketLiquidationFee(UpdatePerpMarketLiquidationFeeIxArgs),
    UpdateInsuranceFundUnstakingPeriod(UpdateInsuranceFundUnstakingPeriodIxArgs),
    UpdateSpotMarketLiquidationFee(UpdateSpotMarketLiquidationFeeIxArgs),
    UpdateWithdrawGuardThreshold(UpdateWithdrawGuardThresholdIxArgs),
    UpdateSpotMarketIfFactor(UpdateSpotMarketIfFactorIxArgs),
    UpdateSpotMarketRevenueSettlePeriod(UpdateSpotMarketRevenueSettlePeriodIxArgs),
    UpdateSpotMarketStatus(UpdateSpotMarketStatusIxArgs),
    UpdateSpotMarketAssetTier(UpdateSpotMarketAssetTierIxArgs),
    UpdateSpotMarketMarginWeights(UpdateSpotMarketMarginWeightsIxArgs),
    UpdateSpotMarketBorrowRate(UpdateSpotMarketBorrowRateIxArgs),
    UpdateSpotMarketMaxTokenDeposits(UpdateSpotMarketMaxTokenDepositsIxArgs),
    UpdateSpotMarketOracle(UpdateSpotMarketOracleIxArgs),
    UpdateSpotMarketStepSizeAndTickSize(UpdateSpotMarketStepSizeAndTickSizeIxArgs),
    UpdateSpotMarketMinOrderSize(UpdateSpotMarketMinOrderSizeIxArgs),
    UpdateSpotMarketOrdersEnabled(UpdateSpotMarketOrdersEnabledIxArgs),
    UpdateSpotMarketName(UpdateSpotMarketNameIxArgs),
    UpdatePerpMarketStatus(UpdatePerpMarketStatusIxArgs),
    UpdatePerpMarketContractTier(UpdatePerpMarketContractTierIxArgs),
    UpdatePerpMarketImfFactor(UpdatePerpMarketImfFactorIxArgs),
    UpdatePerpMarketUnrealizedAssetWeight(UpdatePerpMarketUnrealizedAssetWeightIxArgs),
    UpdatePerpMarketConcentrationCoef(UpdatePerpMarketConcentrationCoefIxArgs),
    UpdatePerpMarketCurveUpdateIntensity(UpdatePerpMarketCurveUpdateIntensityIxArgs),
    UpdatePerpMarketTargetBaseAssetAmountPerLp(UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs),
    UpdateLpCooldownTime(UpdateLpCooldownTimeIxArgs),
    UpdatePerpFeeStructure(UpdatePerpFeeStructureIxArgs),
    UpdateSpotFeeStructure(UpdateSpotFeeStructureIxArgs),
    UpdateInitialPctToLiquidate(UpdateInitialPctToLiquidateIxArgs),
    UpdateLiquidationDuration(UpdateLiquidationDurationIxArgs),
    UpdateOracleGuardRails(UpdateOracleGuardRailsIxArgs),
    UpdateStateSettlementDuration(UpdateStateSettlementDurationIxArgs),
    UpdatePerpMarketOracle(UpdatePerpMarketOracleIxArgs),
    UpdatePerpMarketBaseSpread(UpdatePerpMarketBaseSpreadIxArgs),
    UpdateAmmJitIntensity(UpdateAmmJitIntensityIxArgs),
    UpdatePerpMarketMaxSpread(UpdatePerpMarketMaxSpreadIxArgs),
    UpdatePerpMarketStepSizeAndTickSize(UpdatePerpMarketStepSizeAndTickSizeIxArgs),
    UpdatePerpMarketName(UpdatePerpMarketNameIxArgs),
    UpdatePerpMarketMinOrderSize(UpdatePerpMarketMinOrderSizeIxArgs),
    UpdatePerpMarketMaxSlippageRatio(UpdatePerpMarketMaxSlippageRatioIxArgs),
    UpdatePerpMarketMaxFillReserveFraction(UpdatePerpMarketMaxFillReserveFractionIxArgs),
    UpdatePerpMarketMaxOpenInterest(UpdatePerpMarketMaxOpenInterestIxArgs),
    UpdateAdmin(UpdateAdminIxArgs),
    UpdateWhitelistMint(UpdateWhitelistMintIxArgs),
    UpdateDiscountMint(UpdateDiscountMintIxArgs),
    UpdateExchangeStatus(UpdateExchangeStatusIxArgs),
    UpdatePerpAuctionDuration(UpdatePerpAuctionDurationIxArgs),
    UpdateSpotAuctionDuration(UpdateSpotAuctionDurationIxArgs),
    AdminRemoveInsuranceFundStake(AdminRemoveInsuranceFundStakeIxArgs),
}
impl DriftProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INITIALIZE_USER_IX_DISCM => Ok(Self::InitializeUser(
                InitializeUserIxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_USER_STATS_IX_DISCM => Ok(Self::InitializeUserStats(
                InitializeUserStatsIxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_REFERRER_NAME_IX_DISCM => Ok(Self::InitializeReferrerName(
                InitializeReferrerNameIxArgs::deserialize(&mut reader)?,
            )),
            DEPOSIT_IX_DISCM => Ok(Self::Deposit(DepositIxArgs::deserialize(&mut reader)?)),
            WITHDRAW_IX_DISCM => Ok(Self::Withdraw(WithdrawIxArgs::deserialize(&mut reader)?)),
            TRANSFER_DEPOSIT_IX_DISCM => Ok(Self::TransferDeposit(
                TransferDepositIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_PERP_ORDER_IX_DISCM => Ok(Self::PlacePerpOrder(
                PlacePerpOrderIxArgs::deserialize(&mut reader)?,
            )),
            CANCEL_ORDER_IX_DISCM => Ok(Self::CancelOrder(CancelOrderIxArgs::deserialize(
                &mut reader,
            )?)),
            CANCEL_ORDER_BY_USER_ID_IX_DISCM => Ok(Self::CancelOrderByUserId(
                CancelOrderByUserIdIxArgs::deserialize(&mut reader)?,
            )),
            CANCEL_ORDERS_IX_DISCM => Ok(Self::CancelOrders(CancelOrdersIxArgs::deserialize(
                &mut reader,
            )?)),
            MODIFY_ORDER_IX_DISCM => Ok(Self::ModifyOrder(ModifyOrderIxArgs::deserialize(
                &mut reader,
            )?)),
            MODIFY_ORDER_BY_USER_ID_IX_DISCM => Ok(Self::ModifyOrderByUserId(
                ModifyOrderByUserIdIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_AND_TAKE_PERP_ORDER_IX_DISCM => Ok(Self::PlaceAndTakePerpOrder(
                PlaceAndTakePerpOrderIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_AND_MAKE_PERP_ORDER_IX_DISCM => Ok(Self::PlaceAndMakePerpOrder(
                PlaceAndMakePerpOrderIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_SPOT_ORDER_IX_DISCM => Ok(Self::PlaceSpotOrder(
                PlaceSpotOrderIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM => Ok(Self::PlaceAndTakeSpotOrder(
                PlaceAndTakeSpotOrderIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM => Ok(Self::PlaceAndMakeSpotOrder(
                PlaceAndMakeSpotOrderIxArgs::deserialize(&mut reader)?,
            )),
            BEGIN_SWAP_IX_DISCM => Ok(Self::BeginSwap(BeginSwapIxArgs::deserialize(&mut reader)?)),
            END_SWAP_IX_DISCM => Ok(Self::EndSwap(EndSwapIxArgs::deserialize(&mut reader)?)),
            ADD_PERP_LP_SHARES_IX_DISCM => Ok(Self::AddPerpLpShares(
                AddPerpLpSharesIxArgs::deserialize(&mut reader)?,
            )),
            REMOVE_PERP_LP_SHARES_IX_DISCM => Ok(Self::RemovePerpLpShares(
                RemovePerpLpSharesIxArgs::deserialize(&mut reader)?,
            )),
            REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM => {
                Ok(Self::RemovePerpLpSharesInExpiringMarket(
                    RemovePerpLpSharesInExpiringMarketIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_USER_NAME_IX_DISCM => Ok(Self::UpdateUserName(
                UpdateUserNameIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM => Ok(Self::UpdateUserCustomMarginRatio(
                UpdateUserCustomMarginRatioIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM => {
                Ok(Self::UpdateUserMarginTradingEnabled(
                    UpdateUserMarginTradingEnabledIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_USER_DELEGATE_IX_DISCM => Ok(Self::UpdateUserDelegate(
                UpdateUserDelegateIxArgs::deserialize(&mut reader)?,
            )),
            DELETE_USER_IX_DISCM => Ok(Self::DeleteUser(DeleteUserIxArgs::deserialize(
                &mut reader,
            )?)),
            FILL_PERP_ORDER_IX_DISCM => Ok(Self::FillPerpOrder(FillPerpOrderIxArgs::deserialize(
                &mut reader,
            )?)),
            REVERT_FILL_IX_DISCM => Ok(Self::RevertFill(RevertFillIxArgs::deserialize(
                &mut reader,
            )?)),
            FILL_SPOT_ORDER_IX_DISCM => Ok(Self::FillSpotOrder(FillSpotOrderIxArgs::deserialize(
                &mut reader,
            )?)),
            TRIGGER_ORDER_IX_DISCM => Ok(Self::TriggerOrder(TriggerOrderIxArgs::deserialize(
                &mut reader,
            )?)),
            FORCE_CANCEL_ORDERS_IX_DISCM => Ok(Self::ForceCancelOrders(
                ForceCancelOrdersIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_USER_IDLE_IX_DISCM => Ok(Self::UpdateUserIdle(
                UpdateUserIdleIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM => Ok(Self::UpdateUserOpenOrdersCount(
                UpdateUserOpenOrdersCountIxArgs::deserialize(&mut reader)?,
            )),
            SETTLE_PNL_IX_DISCM => Ok(Self::SettlePnl(SettlePnlIxArgs::deserialize(&mut reader)?)),
            SETTLE_FUNDING_PAYMENT_IX_DISCM => Ok(Self::SettleFundingPayment(
                SettleFundingPaymentIxArgs::deserialize(&mut reader)?,
            )),
            SETTLE_LP_IX_DISCM => Ok(Self::SettleLp(SettleLpIxArgs::deserialize(&mut reader)?)),
            SETTLE_EXPIRED_MARKET_IX_DISCM => Ok(Self::SettleExpiredMarket(
                SettleExpiredMarketIxArgs::deserialize(&mut reader)?,
            )),
            LIQUIDATE_PERP_IX_DISCM => Ok(Self::LiquidatePerp(LiquidatePerpIxArgs::deserialize(
                &mut reader,
            )?)),
            LIQUIDATE_SPOT_IX_DISCM => Ok(Self::LiquidateSpot(LiquidateSpotIxArgs::deserialize(
                &mut reader,
            )?)),
            LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM => Ok(Self::LiquidateBorrowForPerpPnl(
                LiquidateBorrowForPerpPnlIxArgs::deserialize(&mut reader)?,
            )),
            LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM => Ok(Self::LiquidatePerpPnlForDeposit(
                LiquidatePerpPnlForDepositIxArgs::deserialize(&mut reader)?,
            )),
            RESOLVE_PERP_PNL_DEFICIT_IX_DISCM => Ok(Self::ResolvePerpPnlDeficit(
                ResolvePerpPnlDeficitIxArgs::deserialize(&mut reader)?,
            )),
            RESOLVE_PERP_BANKRUPTCY_IX_DISCM => Ok(Self::ResolvePerpBankruptcy(
                ResolvePerpBankruptcyIxArgs::deserialize(&mut reader)?,
            )),
            RESOLVE_SPOT_BANKRUPTCY_IX_DISCM => Ok(Self::ResolveSpotBankruptcy(
                ResolveSpotBankruptcyIxArgs::deserialize(&mut reader)?,
            )),
            SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM => Ok(Self::SettleRevenueToInsuranceFund(
                SettleRevenueToInsuranceFundIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_FUNDING_RATE_IX_DISCM => Ok(Self::UpdateFundingRate(
                UpdateFundingRateIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM => {
                Ok(Self::UpdateSpotMarketCumulativeInterest(
                    UpdateSpotMarketCumulativeInterestIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_AMMS_IX_DISCM => Ok(Self::UpdateAmms(UpdateAmmsIxArgs::deserialize(
                &mut reader,
            )?)),
            UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM => Ok(Self::UpdateSpotMarketExpiry(
                UpdateSpotMarketExpiryIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM => {
                Ok(Self::UpdateUserQuoteAssetInsuranceStake(
                    UpdateUserQuoteAssetInsuranceStakeIxArgs::deserialize(&mut reader)?,
                ))
            }
            INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM => Ok(Self::InitializeInsuranceFundStake(
                InitializeInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
            )),
            ADD_INSURANCE_FUND_STAKE_IX_DISCM => Ok(Self::AddInsuranceFundStake(
                AddInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
            )),
            REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM => {
                Ok(Self::RequestRemoveInsuranceFundStake(
                    RequestRemoveInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
                ))
            }
            CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM => {
                Ok(Self::CancelRequestRemoveInsuranceFundStake(
                    CancelRequestRemoveInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
                ))
            }
            REMOVE_INSURANCE_FUND_STAKE_IX_DISCM => Ok(Self::RemoveInsuranceFundStake(
                RemoveInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(
                &mut reader,
            )?)),
            INITIALIZE_SPOT_MARKET_IX_DISCM => Ok(Self::InitializeSpotMarket(
                InitializeSpotMarketIxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM => {
                Ok(Self::InitializeSerumFulfillmentConfig(
                    InitializeSerumFulfillmentConfigIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM => {
                Ok(Self::UpdateSerumFulfillmentConfigStatus(
                    UpdateSerumFulfillmentConfigStatusIxArgs::deserialize(&mut reader)?,
                ))
            }
            INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM => {
                Ok(Self::InitializePhoenixFulfillmentConfig(
                    InitializePhoenixFulfillmentConfigIxArgs::deserialize(&mut reader)?,
                ))
            }
            PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM => Ok(Self::PhoenixFulfillmentConfigStatus(
                PhoenixFulfillmentConfigStatusIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SERUM_VAULT_IX_DISCM => Ok(Self::UpdateSerumVault(
                UpdateSerumVaultIxArgs::deserialize(&mut reader)?,
            )),
            INITIALIZE_PERP_MARKET_IX_DISCM => Ok(Self::InitializePerpMarket(
                InitializePerpMarketIxArgs::deserialize(&mut reader)?,
            )),
            DELETE_INITIALIZED_PERP_MARKET_IX_DISCM => Ok(Self::DeleteInitializedPerpMarket(
                DeleteInitializedPerpMarketIxArgs::deserialize(&mut reader)?,
            )),
            MOVE_AMM_PRICE_IX_DISCM => Ok(Self::MoveAmmPrice(MoveAmmPriceIxArgs::deserialize(
                &mut reader,
            )?)),
            UPDATE_PERP_MARKET_EXPIRY_IX_DISCM => Ok(Self::UpdatePerpMarketExpiry(
                UpdatePerpMarketExpiryIxArgs::deserialize(&mut reader)?,
            )),
            SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM => {
                Ok(Self::SettleExpiredMarketPoolsToRevenuePool(
                    SettleExpiredMarketPoolsToRevenuePoolIxArgs::deserialize(&mut reader)?,
                ))
            }
            DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM => Ok(Self::DepositIntoPerpMarketFeePool(
                DepositIntoPerpMarketFeePoolIxArgs::deserialize(&mut reader)?,
            )),
            REPEG_AMM_CURVE_IX_DISCM => Ok(Self::RepegAmmCurve(RepegAmmCurveIxArgs::deserialize(
                &mut reader,
            )?)),
            UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM => Ok(Self::UpdatePerpMarketAmmOracleTwap(
                UpdatePerpMarketAmmOracleTwapIxArgs::deserialize(&mut reader)?,
            )),
            RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM => Ok(Self::ResetPerpMarketAmmOracleTwap(
                ResetPerpMarketAmmOracleTwapIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_K_IX_DISCM => Ok(Self::UpdateK(UpdateKIxArgs::deserialize(&mut reader)?)),
            UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM => Ok(Self::UpdatePerpMarketMarginRatio(
                UpdatePerpMarketMarginRatioIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM => Ok(Self::UpdatePerpMarketMaxImbalances(
                UpdatePerpMarketMaxImbalancesIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM => {
                Ok(Self::UpdatePerpMarketLiquidationFee(
                    UpdatePerpMarketLiquidationFeeIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM => {
                Ok(Self::UpdateInsuranceFundUnstakingPeriod(
                    UpdateInsuranceFundUnstakingPeriodIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM => {
                Ok(Self::UpdateSpotMarketLiquidationFee(
                    UpdateSpotMarketLiquidationFeeIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM => Ok(Self::UpdateWithdrawGuardThreshold(
                UpdateWithdrawGuardThresholdIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM => Ok(Self::UpdateSpotMarketIfFactor(
                UpdateSpotMarketIfFactorIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM => {
                Ok(Self::UpdateSpotMarketRevenueSettlePeriod(
                    UpdateSpotMarketRevenueSettlePeriodIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_SPOT_MARKET_STATUS_IX_DISCM => Ok(Self::UpdateSpotMarketStatus(
                UpdateSpotMarketStatusIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM => Ok(Self::UpdateSpotMarketAssetTier(
                UpdateSpotMarketAssetTierIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM => Ok(Self::UpdateSpotMarketMarginWeights(
                UpdateSpotMarketMarginWeightsIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM => Ok(Self::UpdateSpotMarketBorrowRate(
                UpdateSpotMarketBorrowRateIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM => {
                Ok(Self::UpdateSpotMarketMaxTokenDeposits(
                    UpdateSpotMarketMaxTokenDepositsIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_SPOT_MARKET_ORACLE_IX_DISCM => Ok(Self::UpdateSpotMarketOracle(
                UpdateSpotMarketOracleIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM => {
                Ok(Self::UpdateSpotMarketStepSizeAndTickSize(
                    UpdateSpotMarketStepSizeAndTickSizeIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM => Ok(Self::UpdateSpotMarketMinOrderSize(
                UpdateSpotMarketMinOrderSizeIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM => Ok(Self::UpdateSpotMarketOrdersEnabled(
                UpdateSpotMarketOrdersEnabledIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_MARKET_NAME_IX_DISCM => Ok(Self::UpdateSpotMarketName(
                UpdateSpotMarketNameIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_STATUS_IX_DISCM => Ok(Self::UpdatePerpMarketStatus(
                UpdatePerpMarketStatusIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM => Ok(Self::UpdatePerpMarketContractTier(
                UpdatePerpMarketContractTierIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM => Ok(Self::UpdatePerpMarketImfFactor(
                UpdatePerpMarketImfFactorIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM => {
                Ok(Self::UpdatePerpMarketUnrealizedAssetWeight(
                    UpdatePerpMarketUnrealizedAssetWeightIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM => {
                Ok(Self::UpdatePerpMarketConcentrationCoef(
                    UpdatePerpMarketConcentrationCoefIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM => {
                Ok(Self::UpdatePerpMarketCurveUpdateIntensity(
                    UpdatePerpMarketCurveUpdateIntensityIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM => {
                Ok(Self::UpdatePerpMarketTargetBaseAssetAmountPerLp(
                    UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_LP_COOLDOWN_TIME_IX_DISCM => Ok(Self::UpdateLpCooldownTime(
                UpdateLpCooldownTimeIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_FEE_STRUCTURE_IX_DISCM => Ok(Self::UpdatePerpFeeStructure(
                UpdatePerpFeeStructureIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM => Ok(Self::UpdateSpotFeeStructure(
                UpdateSpotFeeStructureIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM => Ok(Self::UpdateInitialPctToLiquidate(
                UpdateInitialPctToLiquidateIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_LIQUIDATION_DURATION_IX_DISCM => Ok(Self::UpdateLiquidationDuration(
                UpdateLiquidationDurationIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_ORACLE_GUARD_RAILS_IX_DISCM => Ok(Self::UpdateOracleGuardRails(
                UpdateOracleGuardRailsIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM => Ok(Self::UpdateStateSettlementDuration(
                UpdateStateSettlementDurationIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_ORACLE_IX_DISCM => Ok(Self::UpdatePerpMarketOracle(
                UpdatePerpMarketOracleIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM => Ok(Self::UpdatePerpMarketBaseSpread(
                UpdatePerpMarketBaseSpreadIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_AMM_JIT_INTENSITY_IX_DISCM => Ok(Self::UpdateAmmJitIntensity(
                UpdateAmmJitIntensityIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM => Ok(Self::UpdatePerpMarketMaxSpread(
                UpdatePerpMarketMaxSpreadIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM => {
                Ok(Self::UpdatePerpMarketStepSizeAndTickSize(
                    UpdatePerpMarketStepSizeAndTickSizeIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_PERP_MARKET_NAME_IX_DISCM => Ok(Self::UpdatePerpMarketName(
                UpdatePerpMarketNameIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM => Ok(Self::UpdatePerpMarketMinOrderSize(
                UpdatePerpMarketMinOrderSizeIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM => {
                Ok(Self::UpdatePerpMarketMaxSlippageRatio(
                    UpdatePerpMarketMaxSlippageRatioIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM => {
                Ok(Self::UpdatePerpMarketMaxFillReserveFraction(
                    UpdatePerpMarketMaxFillReserveFractionIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM => {
                Ok(Self::UpdatePerpMarketMaxOpenInterest(
                    UpdatePerpMarketMaxOpenInterestIxArgs::deserialize(&mut reader)?,
                ))
            }
            UPDATE_ADMIN_IX_DISCM => Ok(Self::UpdateAdmin(UpdateAdminIxArgs::deserialize(
                &mut reader,
            )?)),
            UPDATE_WHITELIST_MINT_IX_DISCM => Ok(Self::UpdateWhitelistMint(
                UpdateWhitelistMintIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_DISCOUNT_MINT_IX_DISCM => Ok(Self::UpdateDiscountMint(
                UpdateDiscountMintIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_EXCHANGE_STATUS_IX_DISCM => Ok(Self::UpdateExchangeStatus(
                UpdateExchangeStatusIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_PERP_AUCTION_DURATION_IX_DISCM => Ok(Self::UpdatePerpAuctionDuration(
                UpdatePerpAuctionDurationIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_SPOT_AUCTION_DURATION_IX_DISCM => Ok(Self::UpdateSpotAuctionDuration(
                UpdateSpotAuctionDurationIxArgs::deserialize(&mut reader)?,
            )),
            ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM => Ok(Self::AdminRemoveInsuranceFundStake(
                AdminRemoveInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeUser(args) => {
                INITIALIZE_USER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializeUserStats(args) => {
                INITIALIZE_USER_STATS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializeReferrerName(args) => {
                INITIALIZE_REFERRER_NAME_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::Deposit(args) => {
                DEPOSIT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::Withdraw(args) => {
                WITHDRAW_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::TransferDeposit(args) => {
                TRANSFER_DEPOSIT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PlacePerpOrder(args) => {
                PLACE_PERP_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CancelOrder(args) => {
                CANCEL_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CancelOrderByUserId(args) => {
                CANCEL_ORDER_BY_USER_ID_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CancelOrders(args) => {
                CANCEL_ORDERS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ModifyOrder(args) => {
                MODIFY_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ModifyOrderByUserId(args) => {
                MODIFY_ORDER_BY_USER_ID_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PlaceAndTakePerpOrder(args) => {
                PLACE_AND_TAKE_PERP_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PlaceAndMakePerpOrder(args) => {
                PLACE_AND_MAKE_PERP_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PlaceSpotOrder(args) => {
                PLACE_SPOT_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PlaceAndTakeSpotOrder(args) => {
                PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PlaceAndMakeSpotOrder(args) => {
                PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::BeginSwap(args) => {
                BEGIN_SWAP_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::EndSwap(args) => {
                END_SWAP_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::AddPerpLpShares(args) => {
                ADD_PERP_LP_SHARES_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RemovePerpLpShares(args) => {
                REMOVE_PERP_LP_SHARES_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RemovePerpLpSharesInExpiringMarket(args) => {
                REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserName(args) => {
                UPDATE_USER_NAME_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserCustomMarginRatio(args) => {
                UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserMarginTradingEnabled(args) => {
                UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserDelegate(args) => {
                UPDATE_USER_DELEGATE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeleteUser(args) => {
                DELETE_USER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::FillPerpOrder(args) => {
                FILL_PERP_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RevertFill(args) => {
                REVERT_FILL_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::FillSpotOrder(args) => {
                FILL_SPOT_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::TriggerOrder(args) => {
                TRIGGER_ORDER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ForceCancelOrders(args) => {
                FORCE_CANCEL_ORDERS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserIdle(args) => {
                UPDATE_USER_IDLE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserOpenOrdersCount(args) => {
                UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SettlePnl(args) => {
                SETTLE_PNL_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SettleFundingPayment(args) => {
                SETTLE_FUNDING_PAYMENT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SettleLp(args) => {
                SETTLE_LP_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SettleExpiredMarket(args) => {
                SETTLE_EXPIRED_MARKET_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::LiquidatePerp(args) => {
                LIQUIDATE_PERP_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::LiquidateSpot(args) => {
                LIQUIDATE_SPOT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::LiquidateBorrowForPerpPnl(args) => {
                LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::LiquidatePerpPnlForDeposit(args) => {
                LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ResolvePerpPnlDeficit(args) => {
                RESOLVE_PERP_PNL_DEFICIT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ResolvePerpBankruptcy(args) => {
                RESOLVE_PERP_BANKRUPTCY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ResolveSpotBankruptcy(args) => {
                RESOLVE_SPOT_BANKRUPTCY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SettleRevenueToInsuranceFund(args) => {
                SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateFundingRate(args) => {
                UPDATE_FUNDING_RATE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketCumulativeInterest(args) => {
                UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateAmms(args) => {
                UPDATE_AMMS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketExpiry(args) => {
                UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateUserQuoteAssetInsuranceStake(args) => {
                UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializeInsuranceFundStake(args) => {
                INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::AddInsuranceFundStake(args) => {
                ADD_INSURANCE_FUND_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RequestRemoveInsuranceFundStake(args) => {
                REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CancelRequestRemoveInsuranceFundStake(args) => {
                CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RemoveInsuranceFundStake(args) => {
                REMOVE_INSURANCE_FUND_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::Initialize(args) => {
                INITIALIZE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializeSpotMarket(args) => {
                INITIALIZE_SPOT_MARKET_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializeSerumFulfillmentConfig(args) => {
                INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSerumFulfillmentConfigStatus(args) => {
                UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializePhoenixFulfillmentConfig(args) => {
                INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PhoenixFulfillmentConfigStatus(args) => {
                PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSerumVault(args) => {
                UPDATE_SERUM_VAULT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::InitializePerpMarket(args) => {
                INITIALIZE_PERP_MARKET_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeleteInitializedPerpMarket(args) => {
                DELETE_INITIALIZED_PERP_MARKET_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::MoveAmmPrice(args) => {
                MOVE_AMM_PRICE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketExpiry(args) => {
                UPDATE_PERP_MARKET_EXPIRY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SettleExpiredMarketPoolsToRevenuePool(args) => {
                SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DepositIntoPerpMarketFeePool(args) => {
                DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RepegAmmCurve(args) => {
                REPEG_AMM_CURVE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketAmmOracleTwap(args) => {
                UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ResetPerpMarketAmmOracleTwap(args) => {
                RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateK(args) => {
                UPDATE_K_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMarginRatio(args) => {
                UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMaxImbalances(args) => {
                UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketLiquidationFee(args) => {
                UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateInsuranceFundUnstakingPeriod(args) => {
                UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketLiquidationFee(args) => {
                UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateWithdrawGuardThreshold(args) => {
                UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketIfFactor(args) => {
                UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketRevenueSettlePeriod(args) => {
                UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketStatus(args) => {
                UPDATE_SPOT_MARKET_STATUS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketAssetTier(args) => {
                UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketMarginWeights(args) => {
                UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketBorrowRate(args) => {
                UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketMaxTokenDeposits(args) => {
                UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketOracle(args) => {
                UPDATE_SPOT_MARKET_ORACLE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketStepSizeAndTickSize(args) => {
                UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketMinOrderSize(args) => {
                UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketOrdersEnabled(args) => {
                UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotMarketName(args) => {
                UPDATE_SPOT_MARKET_NAME_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketStatus(args) => {
                UPDATE_PERP_MARKET_STATUS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketContractTier(args) => {
                UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketImfFactor(args) => {
                UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketUnrealizedAssetWeight(args) => {
                UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketConcentrationCoef(args) => {
                UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketCurveUpdateIntensity(args) => {
                UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketTargetBaseAssetAmountPerLp(args) => {
                UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM
                    .serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateLpCooldownTime(args) => {
                UPDATE_LP_COOLDOWN_TIME_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpFeeStructure(args) => {
                UPDATE_PERP_FEE_STRUCTURE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotFeeStructure(args) => {
                UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateInitialPctToLiquidate(args) => {
                UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateLiquidationDuration(args) => {
                UPDATE_LIQUIDATION_DURATION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateOracleGuardRails(args) => {
                UPDATE_ORACLE_GUARD_RAILS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateStateSettlementDuration(args) => {
                UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketOracle(args) => {
                UPDATE_PERP_MARKET_ORACLE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketBaseSpread(args) => {
                UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateAmmJitIntensity(args) => {
                UPDATE_AMM_JIT_INTENSITY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMaxSpread(args) => {
                UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketStepSizeAndTickSize(args) => {
                UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketName(args) => {
                UPDATE_PERP_MARKET_NAME_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMinOrderSize(args) => {
                UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMaxSlippageRatio(args) => {
                UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMaxFillReserveFraction(args) => {
                UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpMarketMaxOpenInterest(args) => {
                UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateAdmin(args) => {
                UPDATE_ADMIN_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateWhitelistMint(args) => {
                UPDATE_WHITELIST_MINT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateDiscountMint(args) => {
                UPDATE_DISCOUNT_MINT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateExchangeStatus(args) => {
                UPDATE_EXCHANGE_STATUS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePerpAuctionDuration(args) => {
                UPDATE_PERP_AUCTION_DURATION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSpotAuctionDuration(args) => {
                UPDATE_SPOT_AUCTION_DURATION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::AdminRemoveInsuranceFundStake(args) => {
                ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const INITIALIZE_USER_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUserKeys {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializeUserAccounts<'_, '_>> for InitializeUserKeys {
    fn from(accounts: &InitializeUserAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeUserKeys> for [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeUserKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]> for InitializeUserKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_stats: pubkeys[1],
            state: pubkeys[2],
            authority: pubkeys[3],
            payer: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<&InitializeUserAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeUserAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]>
    for InitializeUserAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_stats: &arr[1],
            state: &arr[2],
            authority: &arr[3],
            payer: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeUserIxArgs {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUserIxData(pub InitializeUserIxArgs);
pub const INITIALIZE_USER_IX_DISCM: [u8; 8] = [111, 17, 185, 250, 60, 122, 38, 254];
impl From<InitializeUserIxArgs> for InitializeUserIxData {
    fn from(args: InitializeUserIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_USER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeUserIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_user_ix<K: Into<InitializeUserKeys>, A: Into<InitializeUserIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeUserKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeUserIxArgs = args.into();
    let data: InitializeUserIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_user_invoke<'info, A: Into<InitializeUserIxArgs>>(
    accounts: &InitializeUserAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_user_invoke_signed<'info, A: Into<InitializeUserIxArgs>>(
    accounts: &InitializeUserAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_user_verify_account_keys(
    accounts: &InitializeUserAccounts<'_, '_>,
    keys: &InitializeUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.payer.key, &keys.payer),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_user_verify_account_privileges<'me, 'info>(
    accounts: &InitializeUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.state,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserStatsAccounts<'me, 'info> {
    pub user_stats: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeUserStatsKeys {
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializeUserStatsAccounts<'_, '_>> for InitializeUserStatsKeys {
    fn from(accounts: &InitializeUserStatsAccounts) -> Self {
        Self {
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeUserStatsKeys> for [AccountMeta; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeUserStatsKeys) -> Self {
        [
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN]> for InitializeUserStatsKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_stats: pubkeys[0],
            state: pubkeys[1],
            authority: pubkeys[2],
            payer: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<&InitializeUserStatsAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeUserStatsAccounts<'_, 'info>) -> Self {
        [
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN]>
    for InitializeUserStatsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_stats: &arr[0],
            state: &arr[1],
            authority: &arr[2],
            payer: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeUserStatsIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeUserStatsIxData(pub InitializeUserStatsIxArgs);
pub const INITIALIZE_USER_STATS_IX_DISCM: [u8; 8] = [254, 243, 72, 98, 251, 130, 168, 213];
impl From<InitializeUserStatsIxArgs> for InitializeUserStatsIxData {
    fn from(args: InitializeUserStatsIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeUserStatsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_USER_STATS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_USER_STATS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeUserStatsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_STATS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_user_stats_ix<
    K: Into<InitializeUserStatsKeys>,
    A: Into<InitializeUserStatsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeUserStatsKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeUserStatsIxArgs = args.into();
    let data: InitializeUserStatsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_user_stats_invoke<'info, A: Into<InitializeUserStatsIxArgs>>(
    accounts: &InitializeUserStatsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_user_stats_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_user_stats_invoke_signed<'info, A: Into<InitializeUserStatsIxArgs>>(
    accounts: &InitializeUserStatsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_user_stats_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_user_stats_verify_account_keys(
    accounts: &InitializeUserStatsAccounts<'_, '_>,
    keys: &InitializeUserStatsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.payer.key, &keys.payer),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_user_stats_verify_account_privileges<'me, 'info>(
    accounts: &InitializeUserStatsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user_stats, accounts.state, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeReferrerNameAccounts<'me, 'info> {
    pub referrer_name: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeReferrerNameKeys {
    pub referrer_name: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializeReferrerNameAccounts<'_, '_>> for InitializeReferrerNameKeys {
    fn from(accounts: &InitializeReferrerNameAccounts) -> Self {
        Self {
            referrer_name: *accounts.referrer_name.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeReferrerNameKeys> for [AccountMeta; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeReferrerNameKeys) -> Self {
        [
            AccountMeta::new(keys.referrer_name, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN]> for InitializeReferrerNameKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            referrer_name: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            payer: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<&InitializeReferrerNameAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeReferrerNameAccounts<'_, 'info>) -> Self {
        [
            accounts.referrer_name.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN]>
    for InitializeReferrerNameAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            referrer_name: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            payer: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeReferrerNameIxArgs {
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeReferrerNameIxData(pub InitializeReferrerNameIxArgs);
pub const INITIALIZE_REFERRER_NAME_IX_DISCM: [u8; 8] = [235, 126, 231, 10, 42, 164, 26, 61];
impl From<InitializeReferrerNameIxArgs> for InitializeReferrerNameIxData {
    fn from(args: InitializeReferrerNameIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeReferrerNameIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_REFERRER_NAME_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_REFERRER_NAME_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeReferrerNameIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REFERRER_NAME_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_referrer_name_ix<
    K: Into<InitializeReferrerNameKeys>,
    A: Into<InitializeReferrerNameIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeReferrerNameKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeReferrerNameIxArgs = args.into();
    let data: InitializeReferrerNameIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_referrer_name_invoke<'info, A: Into<InitializeReferrerNameIxArgs>>(
    accounts: &InitializeReferrerNameAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_referrer_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_referrer_name_invoke_signed<'info, A: Into<InitializeReferrerNameIxArgs>>(
    accounts: &InitializeReferrerNameAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_referrer_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_referrer_name_verify_account_keys(
    accounts: &InitializeReferrerNameAccounts<'_, '_>,
    keys: &InitializeReferrerNameKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.referrer_name.key, &keys.referrer_name),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (accounts.payer.key, &keys.payer),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_referrer_name_verify_account_privileges<'me, 'info>(
    accounts: &InitializeReferrerNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.referrer_name,
        accounts.user,
        accounts.user_stats,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<&DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: &DepositAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &DepositKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            spot_market_vault: pubkeys[4],
            user_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<&DepositAccounts<'_, 'info>> for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: &DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]>
    for DepositAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            spot_market_vault: &arr[4],
            user_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositIxArgs {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIxData(pub DepositIxArgs);
pub const DEPOSIT_IX_DISCM: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
impl From<DepositIxArgs> for DepositIxData {
    fn from(args: DepositIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_ix<K: Into<DepositKeys>, A: Into<DepositIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositIxArgs = args.into();
    let data: DepositIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_invoke<'info, A: Into<DepositIxArgs>>(
    accounts: &DepositAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_invoke_signed<'info, A: Into<DepositIxArgs>>(
    accounts: &DepositAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deposit_verify_account_keys(
    accounts: &DepositAccounts<'_, '_>,
    keys: &DepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (accounts.user_token_account.key, &keys.user_token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_verify_account_privileges<'me, 'info>(
    accounts: &DepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.spot_market_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<&WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: &WithdrawAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            drift_signer: *accounts.drift_signer.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: &WithdrawKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            spot_market_vault: pubkeys[4],
            drift_signer: pubkeys[5],
            user_token_account: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<&WithdrawAccounts<'_, 'info>> for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: &WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
    for WithdrawAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            spot_market_vault: &arr[4],
            drift_signer: &arr[5],
            user_token_account: &arr[6],
            token_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawIxArgs {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIxData(pub WithdrawIxArgs);
pub const WITHDRAW_IX_DISCM: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
impl From<WithdrawIxArgs> for WithdrawIxData {
    fn from(args: WithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ix<K: Into<WithdrawKeys>, A: Into<WithdrawIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: WithdrawKeys = accounts.into();
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: WithdrawIxArgs = args.into();
    let data: WithdrawIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_invoke<'info, A: Into<WithdrawIxArgs>>(
    accounts: &WithdrawAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = withdraw_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn withdraw_invoke_signed<'info, A: Into<WithdrawIxArgs>>(
    accounts: &WithdrawAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = withdraw_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: &WithdrawAccounts<'_, '_>,
    keys: &WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.user_token_account.key, &keys.user_token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: &WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.spot_market_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct TransferDepositAccounts<'me, 'info> {
    pub from_user: &'me AccountInfo<'info>,
    pub to_user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransferDepositKeys {
    pub from_user: Pubkey,
    pub to_user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub state: Pubkey,
    pub spot_market_vault: Pubkey,
}
impl From<&TransferDepositAccounts<'_, '_>> for TransferDepositKeys {
    fn from(accounts: &TransferDepositAccounts) -> Self {
        Self {
            from_user: *accounts.from_user.key,
            to_user: *accounts.to_user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            state: *accounts.state.key,
            spot_market_vault: *accounts.spot_market_vault.key,
        }
    }
}
impl From<&TransferDepositKeys> for [AccountMeta; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &TransferDepositKeys) -> Self {
        [
            AccountMeta::new(keys.from_user, false),
            AccountMeta::new(keys.to_user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market_vault, false),
        ]
    }
}
impl From<[Pubkey; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN]> for TransferDepositKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from_user: pubkeys[0],
            to_user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            state: pubkeys[4],
            spot_market_vault: pubkeys[5],
        }
    }
}
impl<'info> From<&TransferDepositAccounts<'_, 'info>>
    for [AccountInfo<'info>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &TransferDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.from_user.clone(),
            accounts.to_user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.state.clone(),
            accounts.spot_market_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN]>
    for TransferDepositAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from_user: &arr[0],
            to_user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            state: &arr[4],
            spot_market_vault: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransferDepositIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransferDepositIxData(pub TransferDepositIxArgs);
pub const TRANSFER_DEPOSIT_IX_DISCM: [u8; 8] = [20, 20, 147, 223, 41, 63, 204, 111];
impl From<TransferDepositIxArgs> for TransferDepositIxData {
    fn from(args: TransferDepositIxArgs) -> Self {
        Self(args)
    }
}
impl TransferDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRANSFER_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRANSFER_DEPOSIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TransferDepositIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_DEPOSIT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn transfer_deposit_ix<K: Into<TransferDepositKeys>, A: Into<TransferDepositIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: TransferDepositKeys = accounts.into();
    let metas: [AccountMeta; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: TransferDepositIxArgs = args.into();
    let data: TransferDepositIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_deposit_invoke<'info, A: Into<TransferDepositIxArgs>>(
    accounts: &TransferDepositAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = transfer_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn transfer_deposit_invoke_signed<'info, A: Into<TransferDepositIxArgs>>(
    accounts: &TransferDepositAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = transfer_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn transfer_deposit_verify_account_keys(
    accounts: &TransferDepositAccounts<'_, '_>,
    keys: &TransferDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.from_user.key, &keys.from_user),
        (accounts.to_user.key, &keys.to_user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (accounts.state.key, &keys.state),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn transfer_deposit_verify_account_privileges<'me, 'info>(
    accounts: &TransferDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from_user, accounts.to_user, accounts.user_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PLACE_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PlacePerpOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlacePerpOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&PlacePerpOrderAccounts<'_, '_>> for PlacePerpOrderKeys {
    fn from(accounts: &PlacePerpOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlacePerpOrderKeys> for [AccountMeta; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlacePerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN]> for PlacePerpOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&PlacePerpOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlacePerpOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN]>
    for PlacePerpOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlacePerpOrderIxArgs {
    pub params: OrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlacePerpOrderIxData(pub PlacePerpOrderIxArgs);
pub const PLACE_PERP_ORDER_IX_DISCM: [u8; 8] = [69, 161, 93, 202, 120, 126, 76, 185];
impl From<PlacePerpOrderIxArgs> for PlacePerpOrderIxData {
    fn from(args: PlacePerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlacePerpOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_PERP_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_PERP_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlacePerpOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_perp_order_ix<K: Into<PlacePerpOrderKeys>, A: Into<PlacePerpOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlacePerpOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlacePerpOrderIxArgs = args.into();
    let data: PlacePerpOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_perp_order_invoke<'info, A: Into<PlacePerpOrderIxArgs>>(
    accounts: &PlacePerpOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_perp_order_invoke_signed<'info, A: Into<PlacePerpOrderIxArgs>>(
    accounts: &PlacePerpOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_perp_order_verify_account_keys(
    accounts: &PlacePerpOrderAccounts<'_, '_>,
    keys: &PlacePerpOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_perp_order_verify_account_privileges<'me, 'info>(
    accounts: &PlacePerpOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CANCEL_ORDER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&CancelOrderAccounts<'_, '_>> for CancelOrderKeys {
    fn from(accounts: &CancelOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&CancelOrderKeys> for [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]> for CancelOrderKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&CancelOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]>
    for CancelOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrderIxArgs {
    pub order_id: Option<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderIxData(pub CancelOrderIxArgs);
pub const CANCEL_ORDER_IX_DISCM: [u8; 8] = [95, 129, 237, 240, 8, 49, 223, 132];
impl From<CancelOrderIxArgs> for CancelOrderIxData {
    fn from(args: CancelOrderIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_ix<K: Into<CancelOrderKeys>, A: Into<CancelOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelOrderKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelOrderIxArgs = args.into();
    let data: CancelOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_invoke<'info, A: Into<CancelOrderIxArgs>>(
    accounts: &CancelOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_order_invoke_signed<'info, A: Into<CancelOrderIxArgs>>(
    accounts: &CancelOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_order_verify_account_keys(
    accounts: &CancelOrderAccounts<'_, '_>,
    keys: &CancelOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_order_verify_account_privileges<'me, 'info>(
    accounts: &CancelOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByUserIdAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrderByUserIdKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&CancelOrderByUserIdAccounts<'_, '_>> for CancelOrderByUserIdKeys {
    fn from(accounts: &CancelOrderByUserIdAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&CancelOrderByUserIdKeys> for [AccountMeta; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelOrderByUserIdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]> for CancelOrderByUserIdKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&CancelOrderByUserIdAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelOrderByUserIdAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]>
    for CancelOrderByUserIdAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrderByUserIdIxArgs {
    pub user_order_id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrderByUserIdIxData(pub CancelOrderByUserIdIxArgs);
pub const CANCEL_ORDER_BY_USER_ID_IX_DISCM: [u8; 8] = [107, 211, 250, 133, 18, 37, 57, 100];
impl From<CancelOrderByUserIdIxArgs> for CancelOrderByUserIdIxData {
    fn from(args: CancelOrderByUserIdIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrderByUserIdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDER_BY_USER_ID_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ORDER_BY_USER_ID_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelOrderByUserIdIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_BY_USER_ID_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_order_by_user_id_ix<
    K: Into<CancelOrderByUserIdKeys>,
    A: Into<CancelOrderByUserIdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelOrderByUserIdKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelOrderByUserIdIxArgs = args.into();
    let data: CancelOrderByUserIdIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_by_user_id_invoke<'info, A: Into<CancelOrderByUserIdIxArgs>>(
    accounts: &CancelOrderByUserIdAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_order_by_user_id_invoke_signed<'info, A: Into<CancelOrderByUserIdIxArgs>>(
    accounts: &CancelOrderByUserIdAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_order_by_user_id_verify_account_keys(
    accounts: &CancelOrderByUserIdAccounts<'_, '_>,
    keys: &CancelOrderByUserIdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_order_by_user_id_verify_account_privileges<'me, 'info>(
    accounts: &CancelOrderByUserIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CANCEL_ORDERS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrdersAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelOrdersKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&CancelOrdersAccounts<'_, '_>> for CancelOrdersKeys {
    fn from(accounts: &CancelOrdersAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&CancelOrdersKeys> for [AccountMeta; CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; CANCEL_ORDERS_IX_ACCOUNTS_LEN]> for CancelOrdersKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&CancelOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ORDERS_IX_ACCOUNTS_LEN]>
    for CancelOrdersAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrdersIxArgs {
    pub market_type: Option<MarketType>,
    pub market_index: Option<u16>,
    pub direction: Option<PositionDirection>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelOrdersIxData(pub CancelOrdersIxArgs);
pub const CANCEL_ORDERS_IX_DISCM: [u8; 8] = [238, 225, 95, 158, 227, 103, 8, 194];
impl From<CancelOrdersIxArgs> for CancelOrdersIxData {
    fn from(args: CancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl CancelOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelOrdersIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDERS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_orders_ix<K: Into<CancelOrdersKeys>, A: Into<CancelOrdersIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelOrdersKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ORDERS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelOrdersIxArgs = args.into();
    let data: CancelOrdersIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_orders_invoke<'info, A: Into<CancelOrdersIxArgs>>(
    accounts: &CancelOrdersAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_orders_invoke_signed<'info, A: Into<CancelOrdersIxArgs>>(
    accounts: &CancelOrdersAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_orders_verify_account_keys(
    accounts: &CancelOrdersAccounts<'_, '_>,
    keys: &CancelOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_orders_verify_account_privileges<'me, 'info>(
    accounts: &CancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const MODIFY_ORDER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ModifyOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&ModifyOrderAccounts<'_, '_>> for ModifyOrderKeys {
    fn from(accounts: &ModifyOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&ModifyOrderKeys> for [AccountMeta; MODIFY_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &ModifyOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; MODIFY_ORDER_IX_ACCOUNTS_LEN]> for ModifyOrderKeys {
    fn from(pubkeys: [Pubkey; MODIFY_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&ModifyOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; MODIFY_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ModifyOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MODIFY_ORDER_IX_ACCOUNTS_LEN]>
    for ModifyOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MODIFY_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyOrderIxArgs {
    pub order_id: Option<u32>,
    pub modify_order_params: ModifyOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModifyOrderIxData(pub ModifyOrderIxArgs);
pub const MODIFY_ORDER_IX_DISCM: [u8; 8] = [47, 124, 117, 255, 201, 197, 130, 94];
impl From<ModifyOrderIxArgs> for ModifyOrderIxData {
    fn from(args: ModifyOrderIxArgs) -> Self {
        Self(args)
    }
}
impl ModifyOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODIFY_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MODIFY_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ModifyOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODIFY_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn modify_order_ix<K: Into<ModifyOrderKeys>, A: Into<ModifyOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ModifyOrderKeys = accounts.into();
    let metas: [AccountMeta; MODIFY_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ModifyOrderIxArgs = args.into();
    let data: ModifyOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn modify_order_invoke<'info, A: Into<ModifyOrderIxArgs>>(
    accounts: &ModifyOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = modify_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MODIFY_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn modify_order_invoke_signed<'info, A: Into<ModifyOrderIxArgs>>(
    accounts: &ModifyOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = modify_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MODIFY_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn modify_order_verify_account_keys(
    accounts: &ModifyOrderAccounts<'_, '_>,
    keys: &ModifyOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn modify_order_verify_account_privileges<'me, 'info>(
    accounts: &ModifyOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderByUserIdAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ModifyOrderByUserIdKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&ModifyOrderByUserIdAccounts<'_, '_>> for ModifyOrderByUserIdKeys {
    fn from(accounts: &ModifyOrderByUserIdAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&ModifyOrderByUserIdKeys> for [AccountMeta; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] {
    fn from(keys: &ModifyOrderByUserIdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]> for ModifyOrderByUserIdKeys {
    fn from(pubkeys: [Pubkey; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&ModifyOrderByUserIdAccounts<'_, 'info>>
    for [AccountInfo<'info>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ModifyOrderByUserIdAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]>
    for ModifyOrderByUserIdAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyOrderByUserIdIxArgs {
    pub user_order_id: u8,
    pub modify_order_params: ModifyOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ModifyOrderByUserIdIxData(pub ModifyOrderByUserIdIxArgs);
pub const MODIFY_ORDER_BY_USER_ID_IX_DISCM: [u8; 8] = [158, 77, 4, 253, 252, 194, 161, 179];
impl From<ModifyOrderByUserIdIxArgs> for ModifyOrderByUserIdIxData {
    fn from(args: ModifyOrderByUserIdIxArgs) -> Self {
        Self(args)
    }
}
impl ModifyOrderByUserIdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MODIFY_ORDER_BY_USER_ID_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MODIFY_ORDER_BY_USER_ID_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ModifyOrderByUserIdIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MODIFY_ORDER_BY_USER_ID_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn modify_order_by_user_id_ix<
    K: Into<ModifyOrderByUserIdKeys>,
    A: Into<ModifyOrderByUserIdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ModifyOrderByUserIdKeys = accounts.into();
    let metas: [AccountMeta; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ModifyOrderByUserIdIxArgs = args.into();
    let data: ModifyOrderByUserIdIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn modify_order_by_user_id_invoke<'info, A: Into<ModifyOrderByUserIdIxArgs>>(
    accounts: &ModifyOrderByUserIdAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = modify_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn modify_order_by_user_id_invoke_signed<'info, A: Into<ModifyOrderByUserIdIxArgs>>(
    accounts: &ModifyOrderByUserIdAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = modify_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn modify_order_by_user_id_verify_account_keys(
    accounts: &ModifyOrderByUserIdAccounts<'_, '_>,
    keys: &ModifyOrderByUserIdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn modify_order_by_user_id_verify_account_privileges<'me, 'info>(
    accounts: &ModifyOrderByUserIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakePerpOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceAndTakePerpOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
}
impl From<&PlaceAndTakePerpOrderAccounts<'_, '_>> for PlaceAndTakePerpOrderKeys {
    fn from(accounts: &PlaceAndTakePerpOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndTakePerpOrderKeys> for [AccountMeta; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndTakePerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN]> for PlaceAndTakePerpOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
        }
    }
}
impl<'info> From<&PlaceAndTakePerpOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndTakePerpOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN]>
    for PlaceAndTakePerpOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceAndTakePerpOrderIxArgs {
    pub params: OrderParams,
    pub maker_order_id: Option<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceAndTakePerpOrderIxData(pub PlaceAndTakePerpOrderIxArgs);
pub const PLACE_AND_TAKE_PERP_ORDER_IX_DISCM: [u8; 8] = [213, 51, 1, 187, 108, 220, 230, 224];
impl From<PlaceAndTakePerpOrderIxArgs> for PlaceAndTakePerpOrderIxData {
    fn from(args: PlaceAndTakePerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceAndTakePerpOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_AND_TAKE_PERP_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_AND_TAKE_PERP_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceAndTakePerpOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_TAKE_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_and_take_perp_order_ix<
    K: Into<PlaceAndTakePerpOrderKeys>,
    A: Into<PlaceAndTakePerpOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndTakePerpOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndTakePerpOrderIxArgs = args.into();
    let data: PlaceAndTakePerpOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_take_perp_order_invoke<'info, A: Into<PlaceAndTakePerpOrderIxArgs>>(
    accounts: &PlaceAndTakePerpOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_and_take_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_take_perp_order_invoke_signed<'info, A: Into<PlaceAndTakePerpOrderIxArgs>>(
    accounts: &PlaceAndTakePerpOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_take_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_and_take_perp_order_verify_account_keys(
    accounts: &PlaceAndTakePerpOrderAccounts<'_, '_>,
    keys: &PlaceAndTakePerpOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_and_take_perp_order_verify_account_privileges<'me, 'info>(
    accounts: &PlaceAndTakePerpOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.user_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakePerpOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub taker: &'me AccountInfo<'info>,
    pub taker_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceAndMakePerpOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub taker: Pubkey,
    pub taker_stats: Pubkey,
    pub authority: Pubkey,
}
impl From<&PlaceAndMakePerpOrderAccounts<'_, '_>> for PlaceAndMakePerpOrderKeys {
    fn from(accounts: &PlaceAndMakePerpOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            taker: *accounts.taker.key,
            taker_stats: *accounts.taker_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndMakePerpOrderKeys> for [AccountMeta; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndMakePerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.taker, false),
            AccountMeta::new(keys.taker_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN]> for PlaceAndMakePerpOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            taker: pubkeys[3],
            taker_stats: pubkeys[4],
            authority: pubkeys[5],
        }
    }
}
impl<'info> From<&PlaceAndMakePerpOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndMakePerpOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.taker.clone(),
            accounts.taker_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN]>
    for PlaceAndMakePerpOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            taker: &arr[3],
            taker_stats: &arr[4],
            authority: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceAndMakePerpOrderIxArgs {
    pub params: OrderParams,
    pub taker_order_id: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceAndMakePerpOrderIxData(pub PlaceAndMakePerpOrderIxArgs);
pub const PLACE_AND_MAKE_PERP_ORDER_IX_DISCM: [u8; 8] = [149, 117, 11, 237, 47, 95, 89, 237];
impl From<PlaceAndMakePerpOrderIxArgs> for PlaceAndMakePerpOrderIxData {
    fn from(args: PlaceAndMakePerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceAndMakePerpOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_AND_MAKE_PERP_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_AND_MAKE_PERP_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceAndMakePerpOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_MAKE_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_and_make_perp_order_ix<
    K: Into<PlaceAndMakePerpOrderKeys>,
    A: Into<PlaceAndMakePerpOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndMakePerpOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndMakePerpOrderIxArgs = args.into();
    let data: PlaceAndMakePerpOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_make_perp_order_invoke<'info, A: Into<PlaceAndMakePerpOrderIxArgs>>(
    accounts: &PlaceAndMakePerpOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_and_make_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_make_perp_order_invoke_signed<'info, A: Into<PlaceAndMakePerpOrderIxArgs>>(
    accounts: &PlaceAndMakePerpOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_make_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_and_make_perp_order_verify_account_keys(
    accounts: &PlaceAndMakePerpOrderAccounts<'_, '_>,
    keys: &PlaceAndMakePerpOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.taker.key, &keys.taker),
        (accounts.taker_stats.key, &keys.taker_stats),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_and_make_perp_order_verify_account_privileges<'me, 'info>(
    accounts: &PlaceAndMakePerpOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.taker,
        accounts.taker_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PlaceSpotOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceSpotOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&PlaceSpotOrderAccounts<'_, '_>> for PlaceSpotOrderKeys {
    fn from(accounts: &PlaceSpotOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceSpotOrderKeys> for [AccountMeta; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN]> for PlaceSpotOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&PlaceSpotOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceSpotOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN]>
    for PlaceSpotOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceSpotOrderIxArgs {
    pub params: OrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceSpotOrderIxData(pub PlaceSpotOrderIxArgs);
pub const PLACE_SPOT_ORDER_IX_DISCM: [u8; 8] = [45, 79, 81, 160, 248, 90, 91, 220];
impl From<PlaceSpotOrderIxArgs> for PlaceSpotOrderIxData {
    fn from(args: PlaceSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceSpotOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_SPOT_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_SPOT_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceSpotOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_spot_order_ix<K: Into<PlaceSpotOrderKeys>, A: Into<PlaceSpotOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceSpotOrderIxArgs = args.into();
    let data: PlaceSpotOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_spot_order_invoke<'info, A: Into<PlaceSpotOrderIxArgs>>(
    accounts: &PlaceSpotOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_spot_order_invoke_signed<'info, A: Into<PlaceSpotOrderIxArgs>>(
    accounts: &PlaceSpotOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_spot_order_verify_account_keys(
    accounts: &PlaceSpotOrderAccounts<'_, '_>,
    keys: &PlaceSpotOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_spot_order_verify_account_privileges<'me, 'info>(
    accounts: &PlaceSpotOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakeSpotOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceAndTakeSpotOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
}
impl From<&PlaceAndTakeSpotOrderAccounts<'_, '_>> for PlaceAndTakeSpotOrderKeys {
    fn from(accounts: &PlaceAndTakeSpotOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndTakeSpotOrderKeys> for [AccountMeta; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndTakeSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]> for PlaceAndTakeSpotOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
        }
    }
}
impl<'info> From<&PlaceAndTakeSpotOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndTakeSpotOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]>
    for PlaceAndTakeSpotOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceAndTakeSpotOrderIxArgs {
    pub params: OrderParams,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceAndTakeSpotOrderIxData(pub PlaceAndTakeSpotOrderIxArgs);
pub const PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM: [u8; 8] = [191, 3, 138, 71, 114, 198, 202, 100];
impl From<PlaceAndTakeSpotOrderIxArgs> for PlaceAndTakeSpotOrderIxData {
    fn from(args: PlaceAndTakeSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceAndTakeSpotOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceAndTakeSpotOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_and_take_spot_order_ix<
    K: Into<PlaceAndTakeSpotOrderKeys>,
    A: Into<PlaceAndTakeSpotOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndTakeSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndTakeSpotOrderIxArgs = args.into();
    let data: PlaceAndTakeSpotOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_take_spot_order_invoke<'info, A: Into<PlaceAndTakeSpotOrderIxArgs>>(
    accounts: &PlaceAndTakeSpotOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_and_take_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_take_spot_order_invoke_signed<'info, A: Into<PlaceAndTakeSpotOrderIxArgs>>(
    accounts: &PlaceAndTakeSpotOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_take_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_and_take_spot_order_verify_account_keys(
    accounts: &PlaceAndTakeSpotOrderAccounts<'_, '_>,
    keys: &PlaceAndTakeSpotOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_and_take_spot_order_verify_account_privileges<'me, 'info>(
    accounts: &PlaceAndTakeSpotOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.user_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakeSpotOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub taker: &'me AccountInfo<'info>,
    pub taker_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaceAndMakeSpotOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub taker: Pubkey,
    pub taker_stats: Pubkey,
    pub authority: Pubkey,
}
impl From<&PlaceAndMakeSpotOrderAccounts<'_, '_>> for PlaceAndMakeSpotOrderKeys {
    fn from(accounts: &PlaceAndMakeSpotOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            taker: *accounts.taker.key,
            taker_stats: *accounts.taker_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndMakeSpotOrderKeys> for [AccountMeta; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndMakeSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.taker, false),
            AccountMeta::new(keys.taker_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]> for PlaceAndMakeSpotOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            taker: pubkeys[3],
            taker_stats: pubkeys[4],
            authority: pubkeys[5],
        }
    }
}
impl<'info> From<&PlaceAndMakeSpotOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndMakeSpotOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.taker.clone(),
            accounts.taker_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]>
    for PlaceAndMakeSpotOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            taker: &arr[3],
            taker_stats: &arr[4],
            authority: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceAndMakeSpotOrderIxArgs {
    pub params: OrderParams,
    pub taker_order_id: u32,
    pub fulfillment_type: Option<SpotFulfillmentType>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceAndMakeSpotOrderIxData(pub PlaceAndMakeSpotOrderIxArgs);
pub const PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM: [u8; 8] = [149, 158, 85, 66, 239, 9, 243, 98];
impl From<PlaceAndMakeSpotOrderIxArgs> for PlaceAndMakeSpotOrderIxData {
    fn from(args: PlaceAndMakeSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceAndMakeSpotOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceAndMakeSpotOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_and_make_spot_order_ix<
    K: Into<PlaceAndMakeSpotOrderKeys>,
    A: Into<PlaceAndMakeSpotOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndMakeSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndMakeSpotOrderIxArgs = args.into();
    let data: PlaceAndMakeSpotOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_make_spot_order_invoke<'info, A: Into<PlaceAndMakeSpotOrderIxArgs>>(
    accounts: &PlaceAndMakeSpotOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_and_make_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_make_spot_order_invoke_signed<'info, A: Into<PlaceAndMakeSpotOrderIxArgs>>(
    accounts: &PlaceAndMakeSpotOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_make_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_and_make_spot_order_verify_account_keys(
    accounts: &PlaceAndMakeSpotOrderAccounts<'_, '_>,
    keys: &PlaceAndMakeSpotOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.taker.key, &keys.taker),
        (accounts.taker_stats.key, &keys.taker_stats),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_and_make_spot_order_verify_account_privileges<'me, 'info>(
    accounts: &PlaceAndMakeSpotOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.taker,
        accounts.taker_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const BEGIN_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct BeginSwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub out_spot_market_vault: &'me AccountInfo<'info>,
    pub in_spot_market_vault: &'me AccountInfo<'info>,
    pub out_token_account: &'me AccountInfo<'info>,
    pub in_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BeginSwapKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub out_spot_market_vault: Pubkey,
    pub in_spot_market_vault: Pubkey,
    pub out_token_account: Pubkey,
    pub in_token_account: Pubkey,
    pub token_program: Pubkey,
    pub drift_signer: Pubkey,
    pub instructions: Pubkey,
}
impl From<&BeginSwapAccounts<'_, '_>> for BeginSwapKeys {
    fn from(accounts: &BeginSwapAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            out_spot_market_vault: *accounts.out_spot_market_vault.key,
            in_spot_market_vault: *accounts.in_spot_market_vault.key,
            out_token_account: *accounts.out_token_account.key,
            in_token_account: *accounts.in_token_account.key,
            token_program: *accounts.token_program.key,
            drift_signer: *accounts.drift_signer.key,
            instructions: *accounts.instructions.key,
        }
    }
}
impl From<&BeginSwapKeys> for [AccountMeta; BEGIN_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: &BeginSwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.out_spot_market_vault, false),
            AccountMeta::new(keys.in_spot_market_vault, false),
            AccountMeta::new(keys.out_token_account, false),
            AccountMeta::new(keys.in_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.instructions, false),
        ]
    }
}
impl From<[Pubkey; BEGIN_SWAP_IX_ACCOUNTS_LEN]> for BeginSwapKeys {
    fn from(pubkeys: [Pubkey; BEGIN_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            out_spot_market_vault: pubkeys[4],
            in_spot_market_vault: pubkeys[5],
            out_token_account: pubkeys[6],
            in_token_account: pubkeys[7],
            token_program: pubkeys[8],
            drift_signer: pubkeys[9],
            instructions: pubkeys[10],
        }
    }
}
impl<'info> From<&BeginSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; BEGIN_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &BeginSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.out_spot_market_vault.clone(),
            accounts.in_spot_market_vault.clone(),
            accounts.out_token_account.clone(),
            accounts.in_token_account.clone(),
            accounts.token_program.clone(),
            accounts.drift_signer.clone(),
            accounts.instructions.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BEGIN_SWAP_IX_ACCOUNTS_LEN]>
    for BeginSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; BEGIN_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            out_spot_market_vault: &arr[4],
            in_spot_market_vault: &arr[5],
            out_token_account: &arr[6],
            in_token_account: &arr[7],
            token_program: &arr[8],
            drift_signer: &arr[9],
            instructions: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeginSwapIxArgs {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub amount_in: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BeginSwapIxData(pub BeginSwapIxArgs);
pub const BEGIN_SWAP_IX_DISCM: [u8; 8] = [174, 109, 228, 1, 242, 105, 232, 105];
impl From<BeginSwapIxArgs> for BeginSwapIxData {
    fn from(args: BeginSwapIxArgs) -> Self {
        Self(args)
    }
}
impl BeginSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BEGIN_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BEGIN_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BeginSwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BEGIN_SWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn begin_swap_ix<K: Into<BeginSwapKeys>, A: Into<BeginSwapIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: BeginSwapKeys = accounts.into();
    let metas: [AccountMeta; BEGIN_SWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: BeginSwapIxArgs = args.into();
    let data: BeginSwapIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn begin_swap_invoke<'info, A: Into<BeginSwapIxArgs>>(
    accounts: &BeginSwapAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = begin_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; BEGIN_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn begin_swap_invoke_signed<'info, A: Into<BeginSwapIxArgs>>(
    accounts: &BeginSwapAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = begin_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; BEGIN_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn begin_swap_verify_account_keys(
    accounts: &BeginSwapAccounts<'_, '_>,
    keys: &BeginSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (
            accounts.out_spot_market_vault.key,
            &keys.out_spot_market_vault,
        ),
        (
            accounts.in_spot_market_vault.key,
            &keys.in_spot_market_vault,
        ),
        (accounts.out_token_account.key, &keys.out_token_account),
        (accounts.in_token_account.key, &keys.in_token_account),
        (accounts.token_program.key, &keys.token_program),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.instructions.key, &keys.instructions),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn begin_swap_verify_account_privileges<'me, 'info>(
    accounts: &BeginSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.out_spot_market_vault,
        accounts.in_spot_market_vault,
        accounts.out_token_account,
        accounts.in_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const END_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct EndSwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub out_spot_market_vault: &'me AccountInfo<'info>,
    pub in_spot_market_vault: &'me AccountInfo<'info>,
    pub out_token_account: &'me AccountInfo<'info>,
    pub in_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub instructions: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EndSwapKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub out_spot_market_vault: Pubkey,
    pub in_spot_market_vault: Pubkey,
    pub out_token_account: Pubkey,
    pub in_token_account: Pubkey,
    pub token_program: Pubkey,
    pub drift_signer: Pubkey,
    pub instructions: Pubkey,
}
impl From<&EndSwapAccounts<'_, '_>> for EndSwapKeys {
    fn from(accounts: &EndSwapAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            out_spot_market_vault: *accounts.out_spot_market_vault.key,
            in_spot_market_vault: *accounts.in_spot_market_vault.key,
            out_token_account: *accounts.out_token_account.key,
            in_token_account: *accounts.in_token_account.key,
            token_program: *accounts.token_program.key,
            drift_signer: *accounts.drift_signer.key,
            instructions: *accounts.instructions.key,
        }
    }
}
impl From<&EndSwapKeys> for [AccountMeta; END_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: &EndSwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.out_spot_market_vault, false),
            AccountMeta::new(keys.in_spot_market_vault, false),
            AccountMeta::new(keys.out_token_account, false),
            AccountMeta::new(keys.in_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.instructions, false),
        ]
    }
}
impl From<[Pubkey; END_SWAP_IX_ACCOUNTS_LEN]> for EndSwapKeys {
    fn from(pubkeys: [Pubkey; END_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            out_spot_market_vault: pubkeys[4],
            in_spot_market_vault: pubkeys[5],
            out_token_account: pubkeys[6],
            in_token_account: pubkeys[7],
            token_program: pubkeys[8],
            drift_signer: pubkeys[9],
            instructions: pubkeys[10],
        }
    }
}
impl<'info> From<&EndSwapAccounts<'_, 'info>> for [AccountInfo<'info>; END_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: &EndSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.out_spot_market_vault.clone(),
            accounts.in_spot_market_vault.clone(),
            accounts.out_token_account.clone(),
            accounts.in_token_account.clone(),
            accounts.token_program.clone(),
            accounts.drift_signer.clone(),
            accounts.instructions.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; END_SWAP_IX_ACCOUNTS_LEN]>
    for EndSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; END_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            out_spot_market_vault: &arr[4],
            in_spot_market_vault: &arr[5],
            out_token_account: &arr[6],
            in_token_account: &arr[7],
            token_program: &arr[8],
            drift_signer: &arr[9],
            instructions: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndSwapIxArgs {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub limit_price: Option<u64>,
    pub reduce_only: Option<SwapReduceOnly>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EndSwapIxData(pub EndSwapIxArgs);
pub const END_SWAP_IX_DISCM: [u8; 8] = [177, 184, 27, 193, 34, 13, 210, 145];
impl From<EndSwapIxArgs> for EndSwapIxData {
    fn from(args: EndSwapIxArgs) -> Self {
        Self(args)
    }
}
impl EndSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != END_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    END_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EndSwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&END_SWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn end_swap_ix<K: Into<EndSwapKeys>, A: Into<EndSwapIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EndSwapKeys = accounts.into();
    let metas: [AccountMeta; END_SWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EndSwapIxArgs = args.into();
    let data: EndSwapIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn end_swap_invoke<'info, A: Into<EndSwapIxArgs>>(
    accounts: &EndSwapAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = end_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; END_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn end_swap_invoke_signed<'info, A: Into<EndSwapIxArgs>>(
    accounts: &EndSwapAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = end_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; END_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn end_swap_verify_account_keys(
    accounts: &EndSwapAccounts<'_, '_>,
    keys: &EndSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (
            accounts.out_spot_market_vault.key,
            &keys.out_spot_market_vault,
        ),
        (
            accounts.in_spot_market_vault.key,
            &keys.in_spot_market_vault,
        ),
        (accounts.out_token_account.key, &keys.out_token_account),
        (accounts.in_token_account.key, &keys.in_token_account),
        (accounts.token_program.key, &keys.token_program),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.instructions.key, &keys.instructions),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn end_swap_verify_account_privileges<'me, 'info>(
    accounts: &EndSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user,
        accounts.user_stats,
        accounts.out_spot_market_vault,
        accounts.in_spot_market_vault,
        accounts.out_token_account,
        accounts.in_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AddPerpLpSharesAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddPerpLpSharesKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&AddPerpLpSharesAccounts<'_, '_>> for AddPerpLpSharesKeys {
    fn from(accounts: &AddPerpLpSharesAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&AddPerpLpSharesKeys> for [AccountMeta; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddPerpLpSharesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN]> for AddPerpLpSharesKeys {
    fn from(pubkeys: [Pubkey; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&AddPerpLpSharesAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddPerpLpSharesAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN]>
    for AddPerpLpSharesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddPerpLpSharesIxArgs {
    pub n_shares: u64,
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddPerpLpSharesIxData(pub AddPerpLpSharesIxArgs);
pub const ADD_PERP_LP_SHARES_IX_DISCM: [u8; 8] = [56, 209, 56, 197, 119, 254, 188, 117];
impl From<AddPerpLpSharesIxArgs> for AddPerpLpSharesIxData {
    fn from(args: AddPerpLpSharesIxArgs) -> Self {
        Self(args)
    }
}
impl AddPerpLpSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_PERP_LP_SHARES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_PERP_LP_SHARES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddPerpLpSharesIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_PERP_LP_SHARES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_perp_lp_shares_ix<K: Into<AddPerpLpSharesKeys>, A: Into<AddPerpLpSharesIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddPerpLpSharesKeys = accounts.into();
    let metas: [AccountMeta; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddPerpLpSharesIxArgs = args.into();
    let data: AddPerpLpSharesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_perp_lp_shares_invoke<'info, A: Into<AddPerpLpSharesIxArgs>>(
    accounts: &AddPerpLpSharesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_perp_lp_shares_invoke_signed<'info, A: Into<AddPerpLpSharesIxArgs>>(
    accounts: &AddPerpLpSharesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_perp_lp_shares_verify_account_keys(
    accounts: &AddPerpLpSharesAccounts<'_, '_>,
    keys: &AddPerpLpSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_perp_lp_shares_verify_account_privileges<'me, 'info>(
    accounts: &AddPerpLpSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePerpLpSharesKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&RemovePerpLpSharesAccounts<'_, '_>> for RemovePerpLpSharesKeys {
    fn from(accounts: &RemovePerpLpSharesAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&RemovePerpLpSharesKeys> for [AccountMeta; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemovePerpLpSharesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN]> for RemovePerpLpSharesKeys {
    fn from(pubkeys: [Pubkey; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<&RemovePerpLpSharesAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemovePerpLpSharesAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN]>
    for RemovePerpLpSharesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemovePerpLpSharesIxArgs {
    pub shares_to_burn: u64,
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePerpLpSharesIxData(pub RemovePerpLpSharesIxArgs);
pub const REMOVE_PERP_LP_SHARES_IX_DISCM: [u8; 8] = [213, 89, 217, 18, 160, 55, 53, 141];
impl From<RemovePerpLpSharesIxArgs> for RemovePerpLpSharesIxData {
    fn from(args: RemovePerpLpSharesIxArgs) -> Self {
        Self(args)
    }
}
impl RemovePerpLpSharesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_PERP_LP_SHARES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_PERP_LP_SHARES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemovePerpLpSharesIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PERP_LP_SHARES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_perp_lp_shares_ix<
    K: Into<RemovePerpLpSharesKeys>,
    A: Into<RemovePerpLpSharesIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemovePerpLpSharesKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemovePerpLpSharesIxArgs = args.into();
    let data: RemovePerpLpSharesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_perp_lp_shares_invoke<'info, A: Into<RemovePerpLpSharesIxArgs>>(
    accounts: &RemovePerpLpSharesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_perp_lp_shares_invoke_signed<'info, A: Into<RemovePerpLpSharesIxArgs>>(
    accounts: &RemovePerpLpSharesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_perp_lp_shares_verify_account_keys(
    accounts: &RemovePerpLpSharesAccounts<'_, '_>,
    keys: &RemovePerpLpSharesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_perp_lp_shares_verify_account_privileges<'me, 'info>(
    accounts: &RemovePerpLpSharesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesInExpiringMarketAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemovePerpLpSharesInExpiringMarketKeys {
    pub state: Pubkey,
    pub user: Pubkey,
}
impl From<&RemovePerpLpSharesInExpiringMarketAccounts<'_, '_>>
    for RemovePerpLpSharesInExpiringMarketKeys
{
    fn from(accounts: &RemovePerpLpSharesInExpiringMarketAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&RemovePerpLpSharesInExpiringMarketKeys>
    for [AccountMeta; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RemovePerpLpSharesInExpiringMarketKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]>
    for RemovePerpLpSharesInExpiringMarketKeys
{
    fn from(pubkeys: [Pubkey; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
        }
    }
}
impl<'info> From<&RemovePerpLpSharesInExpiringMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.user.clone()]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]>
    for RemovePerpLpSharesInExpiringMarketAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemovePerpLpSharesInExpiringMarketIxArgs {
    pub shares_to_burn: u64,
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePerpLpSharesInExpiringMarketIxData(pub RemovePerpLpSharesInExpiringMarketIxArgs);
pub const REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM: [u8; 8] =
    [83, 254, 253, 137, 59, 122, 68, 156];
impl From<RemovePerpLpSharesInExpiringMarketIxArgs> for RemovePerpLpSharesInExpiringMarketIxData {
    fn from(args: RemovePerpLpSharesInExpiringMarketIxArgs) -> Self {
        Self(args)
    }
}
impl RemovePerpLpSharesInExpiringMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemovePerpLpSharesInExpiringMarketIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_perp_lp_shares_in_expiring_market_ix<
    K: Into<RemovePerpLpSharesInExpiringMarketKeys>,
    A: Into<RemovePerpLpSharesInExpiringMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemovePerpLpSharesInExpiringMarketKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: RemovePerpLpSharesInExpiringMarketIxArgs = args.into();
    let data: RemovePerpLpSharesInExpiringMarketIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_perp_lp_shares_in_expiring_market_invoke<
    'info,
    A: Into<RemovePerpLpSharesInExpiringMarketIxArgs>,
>(
    accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_perp_lp_shares_in_expiring_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_perp_lp_shares_in_expiring_market_invoke_signed<
    'info,
    A: Into<RemovePerpLpSharesInExpiringMarketIxArgs>,
>(
    accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_perp_lp_shares_in_expiring_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_perp_lp_shares_in_expiring_market_verify_account_keys(
    accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, '_>,
    keys: &RemovePerpLpSharesInExpiringMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_perp_lp_shares_in_expiring_market_verify_account_privileges<'me, 'info>(
    accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_USER_NAME_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserNameAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserNameKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&UpdateUserNameAccounts<'_, '_>> for UpdateUserNameKeys {
    fn from(accounts: &UpdateUserNameAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserNameKeys> for [AccountMeta; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateUserNameKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_NAME_IX_ACCOUNTS_LEN]> for UpdateUserNameKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USER_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateUserNameAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserNameAccounts<'_, 'info>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN]>
    for UpdateUserNameAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserNameIxArgs {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserNameIxData(pub UpdateUserNameIxArgs);
pub const UPDATE_USER_NAME_IX_DISCM: [u8; 8] = [135, 25, 185, 56, 165, 53, 34, 136];
impl From<UpdateUserNameIxArgs> for UpdateUserNameIxData {
    fn from(args: UpdateUserNameIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserNameIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_NAME_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_NAME_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserNameIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_NAME_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_name_ix<K: Into<UpdateUserNameKeys>, A: Into<UpdateUserNameIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserNameKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserNameIxArgs = args.into();
    let data: UpdateUserNameIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_name_invoke<'info, A: Into<UpdateUserNameIxArgs>>(
    accounts: &UpdateUserNameAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_name_invoke_signed<'info, A: Into<UpdateUserNameIxArgs>>(
    accounts: &UpdateUserNameAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_name_verify_account_keys(
    accounts: &UpdateUserNameAccounts<'_, '_>,
    keys: &UpdateUserNameKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_name_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserCustomMarginRatioAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserCustomMarginRatioKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&UpdateUserCustomMarginRatioAccounts<'_, '_>> for UpdateUserCustomMarginRatioKeys {
    fn from(accounts: &UpdateUserCustomMarginRatioAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserCustomMarginRatioKeys>
    for [AccountMeta; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserCustomMarginRatioKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]>
    for UpdateUserCustomMarginRatioKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateUserCustomMarginRatioAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserCustomMarginRatioAccounts<'_, 'info>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]>
    for UpdateUserCustomMarginRatioAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserCustomMarginRatioIxArgs {
    pub sub_account_id: u16,
    pub margin_ratio: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserCustomMarginRatioIxData(pub UpdateUserCustomMarginRatioIxArgs);
pub const UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM: [u8; 8] = [21, 221, 140, 187, 32, 129, 11, 123];
impl From<UpdateUserCustomMarginRatioIxArgs> for UpdateUserCustomMarginRatioIxData {
    fn from(args: UpdateUserCustomMarginRatioIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserCustomMarginRatioIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserCustomMarginRatioIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_custom_margin_ratio_ix<
    K: Into<UpdateUserCustomMarginRatioKeys>,
    A: Into<UpdateUserCustomMarginRatioIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserCustomMarginRatioKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserCustomMarginRatioIxArgs = args.into();
    let data: UpdateUserCustomMarginRatioIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_custom_margin_ratio_invoke<'info, A: Into<UpdateUserCustomMarginRatioIxArgs>>(
    accounts: &UpdateUserCustomMarginRatioAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_custom_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_custom_margin_ratio_invoke_signed<
    'info,
    A: Into<UpdateUserCustomMarginRatioIxArgs>,
>(
    accounts: &UpdateUserCustomMarginRatioAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_custom_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_custom_margin_ratio_verify_account_keys(
    accounts: &UpdateUserCustomMarginRatioAccounts<'_, '_>,
    keys: &UpdateUserCustomMarginRatioKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_custom_margin_ratio_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserCustomMarginRatioAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserMarginTradingEnabledAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserMarginTradingEnabledKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&UpdateUserMarginTradingEnabledAccounts<'_, '_>> for UpdateUserMarginTradingEnabledKeys {
    fn from(accounts: &UpdateUserMarginTradingEnabledAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserMarginTradingEnabledKeys>
    for [AccountMeta; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserMarginTradingEnabledKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]>
    for UpdateUserMarginTradingEnabledKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateUserMarginTradingEnabledAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserMarginTradingEnabledAccounts<'_, 'info>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]>
    for UpdateUserMarginTradingEnabledAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserMarginTradingEnabledIxArgs {
    pub sub_account_id: u16,
    pub margin_trading_enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserMarginTradingEnabledIxData(pub UpdateUserMarginTradingEnabledIxArgs);
pub const UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM: [u8; 8] =
    [194, 92, 204, 223, 246, 188, 31, 203];
impl From<UpdateUserMarginTradingEnabledIxArgs> for UpdateUserMarginTradingEnabledIxData {
    fn from(args: UpdateUserMarginTradingEnabledIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserMarginTradingEnabledIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserMarginTradingEnabledIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_margin_trading_enabled_ix<
    K: Into<UpdateUserMarginTradingEnabledKeys>,
    A: Into<UpdateUserMarginTradingEnabledIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserMarginTradingEnabledKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserMarginTradingEnabledIxArgs = args.into();
    let data: UpdateUserMarginTradingEnabledIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_margin_trading_enabled_invoke<
    'info,
    A: Into<UpdateUserMarginTradingEnabledIxArgs>,
>(
    accounts: &UpdateUserMarginTradingEnabledAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_margin_trading_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_margin_trading_enabled_invoke_signed<
    'info,
    A: Into<UpdateUserMarginTradingEnabledIxArgs>,
>(
    accounts: &UpdateUserMarginTradingEnabledAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_margin_trading_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_margin_trading_enabled_verify_account_keys(
    accounts: &UpdateUserMarginTradingEnabledAccounts<'_, '_>,
    keys: &UpdateUserMarginTradingEnabledKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_margin_trading_enabled_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserMarginTradingEnabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserDelegateAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserDelegateKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl From<&UpdateUserDelegateAccounts<'_, '_>> for UpdateUserDelegateKeys {
    fn from(accounts: &UpdateUserDelegateAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserDelegateKeys> for [AccountMeta; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateUserDelegateKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN]> for UpdateUserDelegateKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateUserDelegateAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserDelegateAccounts<'_, 'info>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN]>
    for UpdateUserDelegateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserDelegateIxArgs {
    pub sub_account_id: u16,
    pub delegate: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserDelegateIxData(pub UpdateUserDelegateIxArgs);
pub const UPDATE_USER_DELEGATE_IX_DISCM: [u8; 8] = [139, 205, 141, 141, 113, 36, 94, 187];
impl From<UpdateUserDelegateIxArgs> for UpdateUserDelegateIxData {
    fn from(args: UpdateUserDelegateIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserDelegateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_DELEGATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_DELEGATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserDelegateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_DELEGATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_delegate_ix<
    K: Into<UpdateUserDelegateKeys>,
    A: Into<UpdateUserDelegateIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserDelegateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserDelegateIxArgs = args.into();
    let data: UpdateUserDelegateIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_delegate_invoke<'info, A: Into<UpdateUserDelegateIxArgs>>(
    accounts: &UpdateUserDelegateAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_delegate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_delegate_invoke_signed<'info, A: Into<UpdateUserDelegateIxArgs>>(
    accounts: &UpdateUserDelegateAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_delegate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_delegate_verify_account_keys(
    accounts: &UpdateUserDelegateAccounts<'_, '_>,
    keys: &UpdateUserDelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_delegate_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserDelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DELETE_USER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct DeleteUserAccounts<'me, 'info> {
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteUserKeys {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<&DeleteUserAccounts<'_, '_>> for DeleteUserKeys {
    fn from(accounts: &DeleteUserAccounts) -> Self {
        Self {
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&DeleteUserKeys> for [AccountMeta; DELETE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: &DeleteUserKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; DELETE_USER_IX_ACCOUNTS_LEN]> for DeleteUserKeys {
    fn from(pubkeys: [Pubkey; DELETE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: pubkeys[0],
            user_stats: pubkeys[1],
            state: pubkeys[2],
            authority: pubkeys[3],
        }
    }
}
impl<'info> From<&DeleteUserAccounts<'_, 'info>>
    for [AccountInfo<'info>; DELETE_USER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeleteUserAccounts<'_, 'info>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_USER_IX_ACCOUNTS_LEN]>
    for DeleteUserAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DELETE_USER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user: &arr[0],
            user_stats: &arr[1],
            state: &arr[2],
            authority: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteUserIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteUserIxData(pub DeleteUserIxArgs);
pub const DELETE_USER_IX_DISCM: [u8; 8] = [186, 85, 17, 249, 219, 231, 98, 251];
impl From<DeleteUserIxArgs> for DeleteUserIxData {
    fn from(args: DeleteUserIxArgs) -> Self {
        Self(args)
    }
}
impl DeleteUserIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_USER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DELETE_USER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeleteUserIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_USER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_user_ix<K: Into<DeleteUserKeys>, A: Into<DeleteUserIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeleteUserKeys = accounts.into();
    let metas: [AccountMeta; DELETE_USER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeleteUserIxArgs = args.into();
    let data: DeleteUserIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delete_user_invoke<'info, A: Into<DeleteUserIxArgs>>(
    accounts: &DeleteUserAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = delete_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DELETE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn delete_user_invoke_signed<'info, A: Into<DeleteUserIxArgs>>(
    accounts: &DeleteUserAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = delete_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DELETE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn delete_user_verify_account_keys(
    accounts: &DeleteUserAccounts<'_, '_>,
    keys: &DeleteUserKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn delete_user_verify_account_privileges<'me, 'info>(
    accounts: &DeleteUserAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user, accounts.user_stats, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const FILL_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct FillPerpOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub filler_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillPerpOrderKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl From<&FillPerpOrderAccounts<'_, '_>> for FillPerpOrderKeys {
    fn from(accounts: &FillPerpOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            filler_stats: *accounts.filler_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&FillPerpOrderKeys> for [AccountMeta; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &FillPerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.filler_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl From<[Pubkey; FILL_PERP_ORDER_IX_ACCOUNTS_LEN]> for FillPerpOrderKeys {
    fn from(pubkeys: [Pubkey; FILL_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            filler_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
        }
    }
}
impl<'info> From<&FillPerpOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &FillPerpOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.filler_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN]>
    for FillPerpOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            filler_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FillPerpOrderIxArgs {
    pub order_id: Option<u32>,
    pub maker_order_id: Option<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillPerpOrderIxData(pub FillPerpOrderIxArgs);
pub const FILL_PERP_ORDER_IX_DISCM: [u8; 8] = [13, 188, 248, 103, 134, 217, 106, 240];
impl From<FillPerpOrderIxArgs> for FillPerpOrderIxData {
    fn from(args: FillPerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl FillPerpOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_PERP_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FILL_PERP_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FillPerpOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_perp_order_ix<K: Into<FillPerpOrderKeys>, A: Into<FillPerpOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: FillPerpOrderKeys = accounts.into();
    let metas: [AccountMeta; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: FillPerpOrderIxArgs = args.into();
    let data: FillPerpOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_perp_order_invoke<'info, A: Into<FillPerpOrderIxArgs>>(
    accounts: &FillPerpOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = fill_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn fill_perp_order_invoke_signed<'info, A: Into<FillPerpOrderIxArgs>>(
    accounts: &FillPerpOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = fill_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn fill_perp_order_verify_account_keys(
    accounts: &FillPerpOrderAccounts<'_, '_>,
    keys: &FillPerpOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.filler_stats.key, &keys.filler_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn fill_perp_order_verify_account_privileges<'me, 'info>(
    accounts: &FillPerpOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.filler,
        accounts.filler_stats,
        accounts.user,
        accounts.user_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REVERT_FILL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RevertFillAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub filler_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevertFillKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
}
impl From<&RevertFillAccounts<'_, '_>> for RevertFillKeys {
    fn from(accounts: &RevertFillAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            filler_stats: *accounts.filler_stats.key,
        }
    }
}
impl From<&RevertFillKeys> for [AccountMeta; REVERT_FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: &RevertFillKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.filler_stats, false),
        ]
    }
}
impl From<[Pubkey; REVERT_FILL_IX_ACCOUNTS_LEN]> for RevertFillKeys {
    fn from(pubkeys: [Pubkey; REVERT_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            filler_stats: pubkeys[3],
        }
    }
}
impl<'info> From<&RevertFillAccounts<'_, 'info>>
    for [AccountInfo<'info>; REVERT_FILL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RevertFillAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.filler_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVERT_FILL_IX_ACCOUNTS_LEN]>
    for RevertFillAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REVERT_FILL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            filler_stats: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RevertFillIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RevertFillIxData(pub RevertFillIxArgs);
pub const REVERT_FILL_IX_DISCM: [u8; 8] = [236, 238, 176, 69, 239, 10, 181, 193];
impl From<RevertFillIxArgs> for RevertFillIxData {
    fn from(args: RevertFillIxArgs) -> Self {
        Self(args)
    }
}
impl RevertFillIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVERT_FILL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REVERT_FILL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RevertFillIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVERT_FILL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revert_fill_ix<K: Into<RevertFillKeys>, A: Into<RevertFillIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RevertFillKeys = accounts.into();
    let metas: [AccountMeta; REVERT_FILL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RevertFillIxArgs = args.into();
    let data: RevertFillIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revert_fill_invoke<'info, A: Into<RevertFillIxArgs>>(
    accounts: &RevertFillAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = revert_fill_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REVERT_FILL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revert_fill_invoke_signed<'info, A: Into<RevertFillIxArgs>>(
    accounts: &RevertFillAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revert_fill_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REVERT_FILL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn revert_fill_verify_account_keys(
    accounts: &RevertFillAccounts<'_, '_>,
    keys: &RevertFillKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.filler_stats.key, &keys.filler_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn revert_fill_verify_account_privileges<'me, 'info>(
    accounts: &RevertFillAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.filler, accounts.filler_stats] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const FILL_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct FillSpotOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub filler_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillSpotOrderKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl From<&FillSpotOrderAccounts<'_, '_>> for FillSpotOrderKeys {
    fn from(accounts: &FillSpotOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            filler_stats: *accounts.filler_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&FillSpotOrderKeys> for [AccountMeta; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &FillSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.filler_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl From<[Pubkey; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN]> for FillSpotOrderKeys {
    fn from(pubkeys: [Pubkey; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            filler_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
        }
    }
}
impl<'info> From<&FillSpotOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &FillSpotOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.filler_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN]>
    for FillSpotOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            filler_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FillSpotOrderIxArgs {
    pub order_id: Option<u32>,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FillSpotOrderIxData(pub FillSpotOrderIxArgs);
pub const FILL_SPOT_ORDER_IX_DISCM: [u8; 8] = [212, 206, 130, 173, 21, 34, 199, 40];
impl From<FillSpotOrderIxArgs> for FillSpotOrderIxData {
    fn from(args: FillSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl FillSpotOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FILL_SPOT_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FILL_SPOT_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FillSpotOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FILL_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn fill_spot_order_ix<K: Into<FillSpotOrderKeys>, A: Into<FillSpotOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: FillSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: FillSpotOrderIxArgs = args.into();
    let data: FillSpotOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_spot_order_invoke<'info, A: Into<FillSpotOrderIxArgs>>(
    accounts: &FillSpotOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = fill_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn fill_spot_order_invoke_signed<'info, A: Into<FillSpotOrderIxArgs>>(
    accounts: &FillSpotOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = fill_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn fill_spot_order_verify_account_keys(
    accounts: &FillSpotOrderAccounts<'_, '_>,
    keys: &FillSpotOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.filler_stats.key, &keys.filler_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn fill_spot_order_verify_account_privileges<'me, 'info>(
    accounts: &FillSpotOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.filler,
        accounts.filler_stats,
        accounts.user,
        accounts.user_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const TRIGGER_ORDER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct TriggerOrderAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TriggerOrderKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl From<&TriggerOrderAccounts<'_, '_>> for TriggerOrderKeys {
    fn from(accounts: &TriggerOrderAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&TriggerOrderKeys> for [AccountMeta; TRIGGER_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &TriggerOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; TRIGGER_ORDER_IX_ACCOUNTS_LEN]> for TriggerOrderKeys {
    fn from(pubkeys: [Pubkey; TRIGGER_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            user: pubkeys[3],
        }
    }
}
impl<'info> From<&TriggerOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; TRIGGER_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &TriggerOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRIGGER_ORDER_IX_ACCOUNTS_LEN]>
    for TriggerOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TRIGGER_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            user: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriggerOrderIxArgs {
    pub order_id: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerOrderIxData(pub TriggerOrderIxArgs);
pub const TRIGGER_ORDER_IX_DISCM: [u8; 8] = [63, 112, 51, 233, 232, 47, 240, 199];
impl From<TriggerOrderIxArgs> for TriggerOrderIxData {
    fn from(args: TriggerOrderIxArgs) -> Self {
        Self(args)
    }
}
impl TriggerOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TRIGGER_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TRIGGER_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TriggerOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TRIGGER_ORDER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn trigger_order_ix<K: Into<TriggerOrderKeys>, A: Into<TriggerOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: TriggerOrderKeys = accounts.into();
    let metas: [AccountMeta; TRIGGER_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: TriggerOrderIxArgs = args.into();
    let data: TriggerOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn trigger_order_invoke<'info, A: Into<TriggerOrderIxArgs>>(
    accounts: &TriggerOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = trigger_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; TRIGGER_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn trigger_order_invoke_signed<'info, A: Into<TriggerOrderIxArgs>>(
    accounts: &TriggerOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = trigger_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; TRIGGER_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn trigger_order_verify_account_keys(
    accounts: &TriggerOrderAccounts<'_, '_>,
    keys: &TriggerOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn trigger_order_verify_account_privileges<'me, 'info>(
    accounts: &TriggerOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.filler, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ForceCancelOrdersAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ForceCancelOrdersKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl From<&ForceCancelOrdersAccounts<'_, '_>> for ForceCancelOrdersKeys {
    fn from(accounts: &ForceCancelOrdersAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&ForceCancelOrdersKeys> for [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: &ForceCancelOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]> for ForceCancelOrdersKeys {
    fn from(pubkeys: [Pubkey; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            user: pubkeys[3],
        }
    }
}
impl<'info> From<&ForceCancelOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ForceCancelOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]>
    for ForceCancelOrdersAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            user: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForceCancelOrdersIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ForceCancelOrdersIxData(pub ForceCancelOrdersIxArgs);
pub const FORCE_CANCEL_ORDERS_IX_DISCM: [u8; 8] = [64, 181, 196, 63, 222, 72, 64, 232];
impl From<ForceCancelOrdersIxArgs> for ForceCancelOrdersIxData {
    fn from(args: ForceCancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl ForceCancelOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FORCE_CANCEL_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FORCE_CANCEL_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ForceCancelOrdersIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FORCE_CANCEL_ORDERS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn force_cancel_orders_ix<K: Into<ForceCancelOrdersKeys>, A: Into<ForceCancelOrdersIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ForceCancelOrdersKeys = accounts.into();
    let metas: [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ForceCancelOrdersIxArgs = args.into();
    let data: ForceCancelOrdersIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn force_cancel_orders_invoke<'info, A: Into<ForceCancelOrdersIxArgs>>(
    accounts: &ForceCancelOrdersAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = force_cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn force_cancel_orders_invoke_signed<'info, A: Into<ForceCancelOrdersIxArgs>>(
    accounts: &ForceCancelOrdersAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = force_cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn force_cancel_orders_verify_account_keys(
    accounts: &ForceCancelOrdersAccounts<'_, '_>,
    keys: &ForceCancelOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn force_cancel_orders_verify_account_privileges<'me, 'info>(
    accounts: &ForceCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.filler, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_USER_IDLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserIdleAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserIdleKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl From<&UpdateUserIdleAccounts<'_, '_>> for UpdateUserIdleKeys {
    fn from(accounts: &UpdateUserIdleAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&UpdateUserIdleKeys> for [AccountMeta; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateUserIdleKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN]> for UpdateUserIdleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            user: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdateUserIdleAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserIdleAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN]>
    for UpdateUserIdleAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            user: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserIdleIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserIdleIxData(pub UpdateUserIdleIxArgs);
pub const UPDATE_USER_IDLE_IX_DISCM: [u8; 8] = [253, 133, 67, 22, 103, 161, 20, 100];
impl From<UpdateUserIdleIxArgs> for UpdateUserIdleIxData {
    fn from(args: UpdateUserIdleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserIdleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_IDLE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_IDLE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserIdleIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_IDLE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_idle_ix<K: Into<UpdateUserIdleKeys>, A: Into<UpdateUserIdleIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserIdleKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserIdleIxArgs = args.into();
    let data: UpdateUserIdleIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_idle_invoke<'info, A: Into<UpdateUserIdleIxArgs>>(
    accounts: &UpdateUserIdleAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_idle_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_idle_invoke_signed<'info, A: Into<UpdateUserIdleIxArgs>>(
    accounts: &UpdateUserIdleAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_idle_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_idle_verify_account_keys(
    accounts: &UpdateUserIdleAccounts<'_, '_>,
    keys: &UpdateUserIdleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_idle_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserIdleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.filler, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserOpenOrdersCountAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub filler: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserOpenOrdersCountKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl From<&UpdateUserOpenOrdersCountAccounts<'_, '_>> for UpdateUserOpenOrdersCountKeys {
    fn from(accounts: &UpdateUserOpenOrdersCountAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&UpdateUserOpenOrdersCountKeys>
    for [AccountMeta; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserOpenOrdersCountKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]>
    for UpdateUserOpenOrdersCountKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            filler: pubkeys[2],
            user: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdateUserOpenOrdersCountAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserOpenOrdersCountAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]>
    for UpdateUserOpenOrdersCountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            filler: &arr[2],
            user: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserOpenOrdersCountIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserOpenOrdersCountIxData(pub UpdateUserOpenOrdersCountIxArgs);
pub const UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM: [u8; 8] = [104, 39, 65, 210, 250, 163, 100, 134];
impl From<UpdateUserOpenOrdersCountIxArgs> for UpdateUserOpenOrdersCountIxData {
    fn from(args: UpdateUserOpenOrdersCountIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserOpenOrdersCountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserOpenOrdersCountIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_open_orders_count_ix<
    K: Into<UpdateUserOpenOrdersCountKeys>,
    A: Into<UpdateUserOpenOrdersCountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserOpenOrdersCountKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserOpenOrdersCountIxArgs = args.into();
    let data: UpdateUserOpenOrdersCountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_open_orders_count_invoke<'info, A: Into<UpdateUserOpenOrdersCountIxArgs>>(
    accounts: &UpdateUserOpenOrdersCountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_open_orders_count_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_open_orders_count_invoke_signed<
    'info,
    A: Into<UpdateUserOpenOrdersCountIxArgs>,
>(
    accounts: &UpdateUserOpenOrdersCountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_open_orders_count_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_open_orders_count_verify_account_keys(
    accounts: &UpdateUserOpenOrdersCountAccounts<'_, '_>,
    keys: &UpdateUserOpenOrdersCountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.filler.key, &keys.filler),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_open_orders_count_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserOpenOrdersCountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.filler, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SETTLE_PNL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SettlePnlAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettlePnlKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
}
impl From<&SettlePnlAccounts<'_, '_>> for SettlePnlKeys {
    fn from(accounts: &SettlePnlAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
        }
    }
}
impl From<&SettlePnlKeys> for [AccountMeta; SETTLE_PNL_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettlePnlKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new_readonly(keys.spot_market_vault, false),
        ]
    }
}
impl From<[Pubkey; SETTLE_PNL_IX_ACCOUNTS_LEN]> for SettlePnlKeys {
    fn from(pubkeys: [Pubkey; SETTLE_PNL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
            authority: pubkeys[2],
            spot_market_vault: pubkeys[3],
        }
    }
}
impl<'info> From<&SettlePnlAccounts<'_, 'info>>
    for [AccountInfo<'info>; SETTLE_PNL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettlePnlAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_PNL_IX_ACCOUNTS_LEN]>
    for SettlePnlAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_PNL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
            authority: &arr[2],
            spot_market_vault: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettlePnlIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettlePnlIxData(pub SettlePnlIxArgs);
pub const SETTLE_PNL_IX_DISCM: [u8; 8] = [43, 61, 234, 45, 15, 95, 152, 153];
impl From<SettlePnlIxArgs> for SettlePnlIxData {
    fn from(args: SettlePnlIxArgs) -> Self {
        Self(args)
    }
}
impl SettlePnlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_PNL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_PNL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SettlePnlIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_PNL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_pnl_ix<K: Into<SettlePnlKeys>, A: Into<SettlePnlIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettlePnlKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_PNL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettlePnlIxArgs = args.into();
    let data: SettlePnlIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_pnl_invoke<'info, A: Into<SettlePnlIxArgs>>(
    accounts: &SettlePnlAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = settle_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_PNL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_pnl_invoke_signed<'info, A: Into<SettlePnlIxArgs>>(
    accounts: &SettlePnlAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_PNL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn settle_pnl_verify_account_keys(
    accounts: &SettlePnlAccounts<'_, '_>,
    keys: &SettlePnlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
        (accounts.authority.key, &keys.authority),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn settle_pnl_verify_account_privileges<'me, 'info>(
    accounts: &SettlePnlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SettleFundingPaymentAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleFundingPaymentKeys {
    pub state: Pubkey,
    pub user: Pubkey,
}
impl From<&SettleFundingPaymentAccounts<'_, '_>> for SettleFundingPaymentKeys {
    fn from(accounts: &SettleFundingPaymentAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&SettleFundingPaymentKeys> for [AccountMeta; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettleFundingPaymentKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN]> for SettleFundingPaymentKeys {
    fn from(pubkeys: [Pubkey; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
        }
    }
}
impl<'info> From<&SettleFundingPaymentAccounts<'_, 'info>>
    for [AccountInfo<'info>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleFundingPaymentAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.user.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN]>
    for SettleFundingPaymentAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleFundingPaymentIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleFundingPaymentIxData(pub SettleFundingPaymentIxArgs);
pub const SETTLE_FUNDING_PAYMENT_IX_DISCM: [u8; 8] = [222, 90, 202, 94, 28, 45, 115, 183];
impl From<SettleFundingPaymentIxArgs> for SettleFundingPaymentIxData {
    fn from(args: SettleFundingPaymentIxArgs) -> Self {
        Self(args)
    }
}
impl SettleFundingPaymentIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_FUNDING_PAYMENT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_FUNDING_PAYMENT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SettleFundingPaymentIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_FUNDING_PAYMENT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_funding_payment_ix<
    K: Into<SettleFundingPaymentKeys>,
    A: Into<SettleFundingPaymentIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleFundingPaymentKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleFundingPaymentIxArgs = args.into();
    let data: SettleFundingPaymentIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_funding_payment_invoke<'info, A: Into<SettleFundingPaymentIxArgs>>(
    accounts: &SettleFundingPaymentAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = settle_funding_payment_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_funding_payment_invoke_signed<'info, A: Into<SettleFundingPaymentIxArgs>>(
    accounts: &SettleFundingPaymentAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_funding_payment_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn settle_funding_payment_verify_account_keys(
    accounts: &SettleFundingPaymentAccounts<'_, '_>,
    keys: &SettleFundingPaymentKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn settle_funding_payment_verify_account_privileges<'me, 'info>(
    accounts: &SettleFundingPaymentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const SETTLE_LP_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SettleLpAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleLpKeys {
    pub state: Pubkey,
    pub user: Pubkey,
}
impl From<&SettleLpAccounts<'_, '_>> for SettleLpKeys {
    fn from(accounts: &SettleLpAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&SettleLpKeys> for [AccountMeta; SETTLE_LP_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettleLpKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl From<[Pubkey; SETTLE_LP_IX_ACCOUNTS_LEN]> for SettleLpKeys {
    fn from(pubkeys: [Pubkey; SETTLE_LP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            user: pubkeys[1],
        }
    }
}
impl<'info> From<&SettleLpAccounts<'_, 'info>> for [AccountInfo<'info>; SETTLE_LP_IX_ACCOUNTS_LEN] {
    fn from(accounts: &SettleLpAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.user.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_LP_IX_ACCOUNTS_LEN]>
    for SettleLpAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_LP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            user: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleLpIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleLpIxData(pub SettleLpIxArgs);
pub const SETTLE_LP_IX_DISCM: [u8; 8] = [155, 231, 116, 113, 97, 229, 139, 141];
impl From<SettleLpIxArgs> for SettleLpIxData {
    fn from(args: SettleLpIxArgs) -> Self {
        Self(args)
    }
}
impl SettleLpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_LP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_LP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SettleLpIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_LP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_lp_ix<K: Into<SettleLpKeys>, A: Into<SettleLpIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleLpKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_LP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleLpIxArgs = args.into();
    let data: SettleLpIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_lp_invoke<'info, A: Into<SettleLpIxArgs>>(
    accounts: &SettleLpAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = settle_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_lp_invoke_signed<'info, A: Into<SettleLpIxArgs>>(
    accounts: &SettleLpAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn settle_lp_verify_account_keys(
    accounts: &SettleLpAccounts<'_, '_>,
    keys: &SettleLpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.user.key, &keys.user),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn settle_lp_verify_account_privileges<'me, 'info>(
    accounts: &SettleLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleExpiredMarketKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<&SettleExpiredMarketAccounts<'_, '_>> for SettleExpiredMarketKeys {
    fn from(accounts: &SettleExpiredMarketAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&SettleExpiredMarketKeys> for [AccountMeta; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettleExpiredMarketKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN]> for SettleExpiredMarketKeys {
    fn from(pubkeys: [Pubkey; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<&SettleExpiredMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleExpiredMarketAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN]>
    for SettleExpiredMarketAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleExpiredMarketIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleExpiredMarketIxData(pub SettleExpiredMarketIxArgs);
pub const SETTLE_EXPIRED_MARKET_IX_DISCM: [u8; 8] = [120, 89, 11, 25, 122, 77, 72, 193];
impl From<SettleExpiredMarketIxArgs> for SettleExpiredMarketIxData {
    fn from(args: SettleExpiredMarketIxArgs) -> Self {
        Self(args)
    }
}
impl SettleExpiredMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_EXPIRED_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_EXPIRED_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SettleExpiredMarketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_EXPIRED_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_expired_market_ix<
    K: Into<SettleExpiredMarketKeys>,
    A: Into<SettleExpiredMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleExpiredMarketKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleExpiredMarketIxArgs = args.into();
    let data: SettleExpiredMarketIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_expired_market_invoke<'info, A: Into<SettleExpiredMarketIxArgs>>(
    accounts: &SettleExpiredMarketAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = settle_expired_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_expired_market_invoke_signed<'info, A: Into<SettleExpiredMarketIxArgs>>(
    accounts: &SettleExpiredMarketAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_expired_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn settle_expired_market_verify_account_keys(
    accounts: &SettleExpiredMarketAccounts<'_, '_>,
    keys: &SettleExpiredMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn settle_expired_market_verify_account_privileges<'me, 'info>(
    accounts: &SettleExpiredMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const LIQUIDATE_PERP_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidator: &'me AccountInfo<'info>,
    pub liquidator_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidatePerpKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl From<&LiquidatePerpAccounts<'_, '_>> for LiquidatePerpKeys {
    fn from(accounts: &LiquidatePerpAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidatePerpKeys> for [AccountMeta; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] {
    fn from(keys: &LiquidatePerpKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_PERP_IX_ACCOUNTS_LEN]> for LiquidatePerpKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_PERP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            liquidator: pubkeys[2],
            liquidator_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
        }
    }
}
impl<'info> From<&LiquidatePerpAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidatePerpAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN]>
    for LiquidatePerpAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            liquidator: &arr[2],
            liquidator_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidatePerpIxArgs {
    pub market_index: u16,
    pub liquidator_max_base_asset_amount: u64,
    pub limit_price: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidatePerpIxData(pub LiquidatePerpIxArgs);
pub const LIQUIDATE_PERP_IX_DISCM: [u8; 8] = [75, 35, 119, 247, 191, 18, 139, 2];
impl From<LiquidatePerpIxArgs> for LiquidatePerpIxData {
    fn from(args: LiquidatePerpIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidatePerpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_PERP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDATE_PERP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidatePerpIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_PERP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_perp_ix<K: Into<LiquidatePerpKeys>, A: Into<LiquidatePerpIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidatePerpKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidatePerpIxArgs = args.into();
    let data: LiquidatePerpIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_perp_invoke<'info, A: Into<LiquidatePerpIxArgs>>(
    accounts: &LiquidatePerpAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_perp_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_perp_invoke_signed<'info, A: Into<LiquidatePerpIxArgs>>(
    accounts: &LiquidatePerpAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_perp_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn liquidate_perp_verify_account_keys(
    accounts: &LiquidatePerpAccounts<'_, '_>,
    keys: &LiquidatePerpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.liquidator.key, &keys.liquidator),
        (accounts.liquidator_stats.key, &keys.liquidator_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn liquidate_perp_verify_account_privileges<'me, 'info>(
    accounts: &LiquidatePerpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidator,
        accounts.liquidator_stats,
        accounts.user,
        accounts.user_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const LIQUIDATE_SPOT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateSpotAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidator: &'me AccountInfo<'info>,
    pub liquidator_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateSpotKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl From<&LiquidateSpotAccounts<'_, '_>> for LiquidateSpotKeys {
    fn from(accounts: &LiquidateSpotAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidateSpotKeys> for [AccountMeta; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] {
    fn from(keys: &LiquidateSpotKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN]> for LiquidateSpotKeys {
    fn from(pubkeys: [Pubkey; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            liquidator: pubkeys[2],
            liquidator_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
        }
    }
}
impl<'info> From<&LiquidateSpotAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidateSpotAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN]>
    for LiquidateSpotAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            liquidator: &arr[2],
            liquidator_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidateSpotIxArgs {
    pub asset_market_index: u16,
    pub liability_market_index: u16,
    pub liquidator_max_liability_transfer: u128,
    pub limit_price: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateSpotIxData(pub LiquidateSpotIxArgs);
pub const LIQUIDATE_SPOT_IX_DISCM: [u8; 8] = [107, 0, 128, 41, 35, 229, 251, 18];
impl From<LiquidateSpotIxArgs> for LiquidateSpotIxData {
    fn from(args: LiquidateSpotIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateSpotIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_SPOT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDATE_SPOT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidateSpotIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_SPOT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_spot_ix<K: Into<LiquidateSpotKeys>, A: Into<LiquidateSpotIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidateSpotKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidateSpotIxArgs = args.into();
    let data: LiquidateSpotIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_spot_invoke<'info, A: Into<LiquidateSpotIxArgs>>(
    accounts: &LiquidateSpotAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_spot_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_spot_invoke_signed<'info, A: Into<LiquidateSpotIxArgs>>(
    accounts: &LiquidateSpotAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_spot_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn liquidate_spot_verify_account_keys(
    accounts: &LiquidateSpotAccounts<'_, '_>,
    keys: &LiquidateSpotKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.liquidator.key, &keys.liquidator),
        (accounts.liquidator_stats.key, &keys.liquidator_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn liquidate_spot_verify_account_privileges<'me, 'info>(
    accounts: &LiquidateSpotAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidator,
        accounts.liquidator_stats,
        accounts.user,
        accounts.user_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateBorrowForPerpPnlAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidator: &'me AccountInfo<'info>,
    pub liquidator_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateBorrowForPerpPnlKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl From<&LiquidateBorrowForPerpPnlAccounts<'_, '_>> for LiquidateBorrowForPerpPnlKeys {
    fn from(accounts: &LiquidateBorrowForPerpPnlAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidateBorrowForPerpPnlKeys>
    for [AccountMeta; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &LiquidateBorrowForPerpPnlKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]>
    for LiquidateBorrowForPerpPnlKeys
{
    fn from(pubkeys: [Pubkey; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            liquidator: pubkeys[2],
            liquidator_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
        }
    }
}
impl<'info> From<&LiquidateBorrowForPerpPnlAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidateBorrowForPerpPnlAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]>
    for LiquidateBorrowForPerpPnlAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            liquidator: &arr[2],
            liquidator_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidateBorrowForPerpPnlIxArgs {
    pub perp_market_index: u16,
    pub spot_market_index: u16,
    pub liquidator_max_liability_transfer: u128,
    pub limit_price: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateBorrowForPerpPnlIxData(pub LiquidateBorrowForPerpPnlIxArgs);
pub const LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM: [u8; 8] = [169, 17, 32, 90, 207, 148, 209, 27];
impl From<LiquidateBorrowForPerpPnlIxArgs> for LiquidateBorrowForPerpPnlIxData {
    fn from(args: LiquidateBorrowForPerpPnlIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateBorrowForPerpPnlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidateBorrowForPerpPnlIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_borrow_for_perp_pnl_ix<
    K: Into<LiquidateBorrowForPerpPnlKeys>,
    A: Into<LiquidateBorrowForPerpPnlIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidateBorrowForPerpPnlKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidateBorrowForPerpPnlIxArgs = args.into();
    let data: LiquidateBorrowForPerpPnlIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_borrow_for_perp_pnl_invoke<'info, A: Into<LiquidateBorrowForPerpPnlIxArgs>>(
    accounts: &LiquidateBorrowForPerpPnlAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_borrow_for_perp_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_borrow_for_perp_pnl_invoke_signed<
    'info,
    A: Into<LiquidateBorrowForPerpPnlIxArgs>,
>(
    accounts: &LiquidateBorrowForPerpPnlAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_borrow_for_perp_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn liquidate_borrow_for_perp_pnl_verify_account_keys(
    accounts: &LiquidateBorrowForPerpPnlAccounts<'_, '_>,
    keys: &LiquidateBorrowForPerpPnlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.liquidator.key, &keys.liquidator),
        (accounts.liquidator_stats.key, &keys.liquidator_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn liquidate_borrow_for_perp_pnl_verify_account_privileges<'me, 'info>(
    accounts: &LiquidateBorrowForPerpPnlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidator,
        accounts.liquidator_stats,
        accounts.user,
        accounts.user_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpPnlForDepositAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidator: &'me AccountInfo<'info>,
    pub liquidator_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidatePerpPnlForDepositKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl From<&LiquidatePerpPnlForDepositAccounts<'_, '_>> for LiquidatePerpPnlForDepositKeys {
    fn from(accounts: &LiquidatePerpPnlForDepositAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidatePerpPnlForDepositKeys>
    for [AccountMeta; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(keys: &LiquidatePerpPnlForDepositKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]>
    for LiquidatePerpPnlForDepositKeys
{
    fn from(pubkeys: [Pubkey; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            liquidator: pubkeys[2],
            liquidator_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
        }
    }
}
impl<'info> From<&LiquidatePerpPnlForDepositAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidatePerpPnlForDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]>
    for LiquidatePerpPnlForDepositAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            liquidator: &arr[2],
            liquidator_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidatePerpPnlForDepositIxArgs {
    pub perp_market_index: u16,
    pub spot_market_index: u16,
    pub liquidator_max_pnl_transfer: u128,
    pub limit_price: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidatePerpPnlForDepositIxData(pub LiquidatePerpPnlForDepositIxArgs);
pub const LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM: [u8; 8] = [237, 75, 198, 235, 233, 186, 75, 35];
impl From<LiquidatePerpPnlForDepositIxArgs> for LiquidatePerpPnlForDepositIxData {
    fn from(args: LiquidatePerpPnlForDepositIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidatePerpPnlForDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidatePerpPnlForDepositIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_perp_pnl_for_deposit_ix<
    K: Into<LiquidatePerpPnlForDepositKeys>,
    A: Into<LiquidatePerpPnlForDepositIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidatePerpPnlForDepositKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidatePerpPnlForDepositIxArgs = args.into();
    let data: LiquidatePerpPnlForDepositIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_perp_pnl_for_deposit_invoke<'info, A: Into<LiquidatePerpPnlForDepositIxArgs>>(
    accounts: &LiquidatePerpPnlForDepositAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_perp_pnl_for_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_perp_pnl_for_deposit_invoke_signed<
    'info,
    A: Into<LiquidatePerpPnlForDepositIxArgs>,
>(
    accounts: &LiquidatePerpPnlForDepositAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_perp_pnl_for_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn liquidate_perp_pnl_for_deposit_verify_account_keys(
    accounts: &LiquidatePerpPnlForDepositAccounts<'_, '_>,
    keys: &LiquidatePerpPnlForDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.liquidator.key, &keys.liquidator),
        (accounts.liquidator_stats.key, &keys.liquidator_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn liquidate_perp_pnl_for_deposit_verify_account_privileges<'me, 'info>(
    accounts: &LiquidatePerpPnlForDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidator,
        accounts.liquidator_stats,
        accounts.user,
        accounts.user_stats,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpPnlDeficitAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResolvePerpPnlDeficitKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}
impl From<&ResolvePerpPnlDeficitAccounts<'_, '_>> for ResolvePerpPnlDeficitKeys {
    fn from(accounts: &ResolvePerpPnlDeficitAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ResolvePerpPnlDeficitKeys> for [AccountMeta; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &ResolvePerpPnlDeficitKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN]> for ResolvePerpPnlDeficitKeys {
    fn from(pubkeys: [Pubkey; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            spot_market_vault: pubkeys[2],
            insurance_fund_vault: pubkeys[3],
            drift_signer: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<&ResolvePerpPnlDeficitAccounts<'_, 'info>>
    for [AccountInfo<'info>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ResolvePerpPnlDeficitAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN]>
    for ResolvePerpPnlDeficitAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            spot_market_vault: &arr[2],
            insurance_fund_vault: &arr[3],
            drift_signer: &arr[4],
            token_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolvePerpPnlDeficitIxArgs {
    pub spot_market_index: u16,
    pub perp_market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvePerpPnlDeficitIxData(pub ResolvePerpPnlDeficitIxArgs);
pub const RESOLVE_PERP_PNL_DEFICIT_IX_DISCM: [u8; 8] = [168, 204, 68, 150, 159, 126, 95, 148];
impl From<ResolvePerpPnlDeficitIxArgs> for ResolvePerpPnlDeficitIxData {
    fn from(args: ResolvePerpPnlDeficitIxArgs) -> Self {
        Self(args)
    }
}
impl ResolvePerpPnlDeficitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESOLVE_PERP_PNL_DEFICIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RESOLVE_PERP_PNL_DEFICIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ResolvePerpPnlDeficitIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESOLVE_PERP_PNL_DEFICIT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resolve_perp_pnl_deficit_ix<
    K: Into<ResolvePerpPnlDeficitKeys>,
    A: Into<ResolvePerpPnlDeficitIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResolvePerpPnlDeficitKeys = accounts.into();
    let metas: [AccountMeta; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResolvePerpPnlDeficitIxArgs = args.into();
    let data: ResolvePerpPnlDeficitIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn resolve_perp_pnl_deficit_invoke<'info, A: Into<ResolvePerpPnlDeficitIxArgs>>(
    accounts: &ResolvePerpPnlDeficitAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = resolve_perp_pnl_deficit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn resolve_perp_pnl_deficit_invoke_signed<'info, A: Into<ResolvePerpPnlDeficitIxArgs>>(
    accounts: &ResolvePerpPnlDeficitAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = resolve_perp_pnl_deficit_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn resolve_perp_pnl_deficit_verify_account_keys(
    accounts: &ResolvePerpPnlDeficitAccounts<'_, '_>,
    keys: &ResolvePerpPnlDeficitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn resolve_perp_pnl_deficit_verify_account_privileges<'me, 'info>(
    accounts: &ResolvePerpPnlDeficitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market_vault, accounts.insurance_fund_vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpBankruptcyAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidator: &'me AccountInfo<'info>,
    pub liquidator_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResolvePerpBankruptcyKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}
impl From<&ResolvePerpBankruptcyAccounts<'_, '_>> for ResolvePerpBankruptcyKeys {
    fn from(accounts: &ResolvePerpBankruptcyAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ResolvePerpBankruptcyKeys> for [AccountMeta; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ResolvePerpBankruptcyKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN]> for ResolvePerpBankruptcyKeys {
    fn from(pubkeys: [Pubkey; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            liquidator: pubkeys[2],
            liquidator_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
            spot_market_vault: pubkeys[6],
            insurance_fund_vault: pubkeys[7],
            drift_signer: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&ResolvePerpBankruptcyAccounts<'_, 'info>>
    for [AccountInfo<'info>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ResolvePerpBankruptcyAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN]>
    for ResolvePerpBankruptcyAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            liquidator: &arr[2],
            liquidator_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
            spot_market_vault: &arr[6],
            insurance_fund_vault: &arr[7],
            drift_signer: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolvePerpBankruptcyIxArgs {
    pub quote_spot_market_index: u16,
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvePerpBankruptcyIxData(pub ResolvePerpBankruptcyIxArgs);
pub const RESOLVE_PERP_BANKRUPTCY_IX_DISCM: [u8; 8] = [224, 16, 176, 214, 162, 213, 183, 222];
impl From<ResolvePerpBankruptcyIxArgs> for ResolvePerpBankruptcyIxData {
    fn from(args: ResolvePerpBankruptcyIxArgs) -> Self {
        Self(args)
    }
}
impl ResolvePerpBankruptcyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESOLVE_PERP_BANKRUPTCY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RESOLVE_PERP_BANKRUPTCY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ResolvePerpBankruptcyIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESOLVE_PERP_BANKRUPTCY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resolve_perp_bankruptcy_ix<
    K: Into<ResolvePerpBankruptcyKeys>,
    A: Into<ResolvePerpBankruptcyIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResolvePerpBankruptcyKeys = accounts.into();
    let metas: [AccountMeta; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResolvePerpBankruptcyIxArgs = args.into();
    let data: ResolvePerpBankruptcyIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn resolve_perp_bankruptcy_invoke<'info, A: Into<ResolvePerpBankruptcyIxArgs>>(
    accounts: &ResolvePerpBankruptcyAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = resolve_perp_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn resolve_perp_bankruptcy_invoke_signed<'info, A: Into<ResolvePerpBankruptcyIxArgs>>(
    accounts: &ResolvePerpBankruptcyAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = resolve_perp_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn resolve_perp_bankruptcy_verify_account_keys(
    accounts: &ResolvePerpBankruptcyAccounts<'_, '_>,
    keys: &ResolvePerpBankruptcyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.liquidator.key, &keys.liquidator),
        (accounts.liquidator_stats.key, &keys.liquidator_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn resolve_perp_bankruptcy_verify_account_privileges<'me, 'info>(
    accounts: &ResolvePerpBankruptcyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidator,
        accounts.liquidator_stats,
        accounts.user,
        accounts.user_stats,
        accounts.spot_market_vault,
        accounts.insurance_fund_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct ResolveSpotBankruptcyAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub liquidator: &'me AccountInfo<'info>,
    pub liquidator_stats: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResolveSpotBankruptcyKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}
impl From<&ResolveSpotBankruptcyAccounts<'_, '_>> for ResolveSpotBankruptcyKeys {
    fn from(accounts: &ResolveSpotBankruptcyAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ResolveSpotBankruptcyKeys> for [AccountMeta; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ResolveSpotBankruptcyKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN]> for ResolveSpotBankruptcyKeys {
    fn from(pubkeys: [Pubkey; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
            liquidator: pubkeys[2],
            liquidator_stats: pubkeys[3],
            user: pubkeys[4],
            user_stats: pubkeys[5],
            spot_market_vault: pubkeys[6],
            insurance_fund_vault: pubkeys[7],
            drift_signer: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&ResolveSpotBankruptcyAccounts<'_, 'info>>
    for [AccountInfo<'info>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ResolveSpotBankruptcyAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN]>
    for ResolveSpotBankruptcyAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
            liquidator: &arr[2],
            liquidator_stats: &arr[3],
            user: &arr[4],
            user_stats: &arr[5],
            spot_market_vault: &arr[6],
            insurance_fund_vault: &arr[7],
            drift_signer: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveSpotBankruptcyIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResolveSpotBankruptcyIxData(pub ResolveSpotBankruptcyIxArgs);
pub const RESOLVE_SPOT_BANKRUPTCY_IX_DISCM: [u8; 8] = [124, 194, 240, 254, 198, 213, 52, 122];
impl From<ResolveSpotBankruptcyIxArgs> for ResolveSpotBankruptcyIxData {
    fn from(args: ResolveSpotBankruptcyIxArgs) -> Self {
        Self(args)
    }
}
impl ResolveSpotBankruptcyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESOLVE_SPOT_BANKRUPTCY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RESOLVE_SPOT_BANKRUPTCY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ResolveSpotBankruptcyIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESOLVE_SPOT_BANKRUPTCY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn resolve_spot_bankruptcy_ix<
    K: Into<ResolveSpotBankruptcyKeys>,
    A: Into<ResolveSpotBankruptcyIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResolveSpotBankruptcyKeys = accounts.into();
    let metas: [AccountMeta; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResolveSpotBankruptcyIxArgs = args.into();
    let data: ResolveSpotBankruptcyIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn resolve_spot_bankruptcy_invoke<'info, A: Into<ResolveSpotBankruptcyIxArgs>>(
    accounts: &ResolveSpotBankruptcyAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = resolve_spot_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn resolve_spot_bankruptcy_invoke_signed<'info, A: Into<ResolveSpotBankruptcyIxArgs>>(
    accounts: &ResolveSpotBankruptcyAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = resolve_spot_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn resolve_spot_bankruptcy_verify_account_keys(
    accounts: &ResolveSpotBankruptcyAccounts<'_, '_>,
    keys: &ResolveSpotBankruptcyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.liquidator.key, &keys.liquidator),
        (accounts.liquidator_stats.key, &keys.liquidator_stats),
        (accounts.user.key, &keys.user),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn resolve_spot_bankruptcy_verify_account_privileges<'me, 'info>(
    accounts: &ResolveSpotBankruptcyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidator,
        accounts.liquidator_stats,
        accounts.user,
        accounts.user_stats,
        accounts.spot_market_vault,
        accounts.insurance_fund_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct SettleRevenueToInsuranceFundAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleRevenueToInsuranceFundKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<&SettleRevenueToInsuranceFundAccounts<'_, '_>> for SettleRevenueToInsuranceFundKeys {
    fn from(accounts: &SettleRevenueToInsuranceFundAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            drift_signer: *accounts.drift_signer.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&SettleRevenueToInsuranceFundKeys>
    for [AccountMeta; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SettleRevenueToInsuranceFundKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]>
    for SettleRevenueToInsuranceFundKeys
{
    fn from(pubkeys: [Pubkey; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            spot_market: pubkeys[1],
            spot_market_vault: pubkeys[2],
            drift_signer: pubkeys[3],
            insurance_fund_vault: pubkeys[4],
            token_program: pubkeys[5],
        }
    }
}
impl<'info> From<&SettleRevenueToInsuranceFundAccounts<'_, 'info>>
    for [AccountInfo<'info>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleRevenueToInsuranceFundAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.spot_market_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]>
    for SettleRevenueToInsuranceFundAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            spot_market: &arr[1],
            spot_market_vault: &arr[2],
            drift_signer: &arr[3],
            insurance_fund_vault: &arr[4],
            token_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleRevenueToInsuranceFundIxArgs {
    pub spot_market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleRevenueToInsuranceFundIxData(pub SettleRevenueToInsuranceFundIxArgs);
pub const SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM: [u8; 8] =
    [200, 120, 93, 136, 69, 38, 199, 159];
impl From<SettleRevenueToInsuranceFundIxArgs> for SettleRevenueToInsuranceFundIxData {
    fn from(args: SettleRevenueToInsuranceFundIxArgs) -> Self {
        Self(args)
    }
}
impl SettleRevenueToInsuranceFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SettleRevenueToInsuranceFundIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_revenue_to_insurance_fund_ix<
    K: Into<SettleRevenueToInsuranceFundKeys>,
    A: Into<SettleRevenueToInsuranceFundIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleRevenueToInsuranceFundKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleRevenueToInsuranceFundIxArgs = args.into();
    let data: SettleRevenueToInsuranceFundIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_revenue_to_insurance_fund_invoke<
    'info,
    A: Into<SettleRevenueToInsuranceFundIxArgs>,
>(
    accounts: &SettleRevenueToInsuranceFundAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = settle_revenue_to_insurance_fund_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_revenue_to_insurance_fund_invoke_signed<
    'info,
    A: Into<SettleRevenueToInsuranceFundIxArgs>,
>(
    accounts: &SettleRevenueToInsuranceFundAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_revenue_to_insurance_fund_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn settle_revenue_to_insurance_fund_verify_account_keys(
    accounts: &SettleRevenueToInsuranceFundAccounts<'_, '_>,
    keys: &SettleRevenueToInsuranceFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (accounts.drift_signer.key, &keys.drift_signer),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn settle_revenue_to_insurance_fund_verify_account_privileges<'me, 'info>(
    accounts: &SettleRevenueToInsuranceFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.spot_market,
        accounts.spot_market_vault,
        accounts.insurance_fund_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFundingRateAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateFundingRateKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}
impl From<&UpdateFundingRateAccounts<'_, '_>> for UpdateFundingRateKeys {
    fn from(accounts: &UpdateFundingRateAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateFundingRateKeys> for [AccountMeta; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateFundingRateKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN]> for UpdateFundingRateKeys {
    fn from(pubkeys: [Pubkey; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            perp_market: pubkeys[1],
            oracle: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateFundingRateAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateFundingRateAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN]>
    for UpdateFundingRateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            perp_market: &arr[1],
            oracle: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateFundingRateIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFundingRateIxData(pub UpdateFundingRateIxArgs);
pub const UPDATE_FUNDING_RATE_IX_DISCM: [u8; 8] = [201, 178, 116, 212, 166, 144, 72, 238];
impl From<UpdateFundingRateIxArgs> for UpdateFundingRateIxData {
    fn from(args: UpdateFundingRateIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateFundingRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_FUNDING_RATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_FUNDING_RATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateFundingRateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FUNDING_RATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_funding_rate_ix<K: Into<UpdateFundingRateKeys>, A: Into<UpdateFundingRateIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateFundingRateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateFundingRateIxArgs = args.into();
    let data: UpdateFundingRateIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_funding_rate_invoke<'info, A: Into<UpdateFundingRateIxArgs>>(
    accounts: &UpdateFundingRateAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_funding_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_funding_rate_invoke_signed<'info, A: Into<UpdateFundingRateIxArgs>>(
    accounts: &UpdateFundingRateAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_funding_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_funding_rate_verify_account_keys(
    accounts: &UpdateFundingRateAccounts<'_, '_>,
    keys: &UpdateFundingRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_funding_rate_verify_account_privileges<'me, 'info>(
    accounts: &UpdateFundingRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketCumulativeInterestAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketCumulativeInterestKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
}
impl From<&UpdateSpotMarketCumulativeInterestAccounts<'_, '_>>
    for UpdateSpotMarketCumulativeInterestKeys
{
    fn from(accounts: &UpdateSpotMarketCumulativeInterestAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateSpotMarketCumulativeInterestKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketCumulativeInterestKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketCumulativeInterestKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            spot_market: pubkeys[1],
            oracle: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketCumulativeInterestAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketCumulativeInterestAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            spot_market: &arr[1],
            oracle: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketCumulativeInterestIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketCumulativeInterestIxData(pub UpdateSpotMarketCumulativeInterestIxArgs);
pub const UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM: [u8; 8] =
    [39, 166, 139, 243, 158, 165, 155, 225];
impl From<UpdateSpotMarketCumulativeInterestIxArgs> for UpdateSpotMarketCumulativeInterestIxData {
    fn from(args: UpdateSpotMarketCumulativeInterestIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketCumulativeInterestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketCumulativeInterestIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_cumulative_interest_ix<
    K: Into<UpdateSpotMarketCumulativeInterestKeys>,
    A: Into<UpdateSpotMarketCumulativeInterestIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketCumulativeInterestKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketCumulativeInterestIxArgs = args.into();
    let data: UpdateSpotMarketCumulativeInterestIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_cumulative_interest_invoke<
    'info,
    A: Into<UpdateSpotMarketCumulativeInterestIxArgs>,
>(
    accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_cumulative_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_cumulative_interest_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketCumulativeInterestIxArgs>,
>(
    accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_cumulative_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_cumulative_interest_verify_account_keys(
    accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, '_>,
    keys: &UpdateSpotMarketCumulativeInterestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (accounts.oracle.key, &keys.oracle),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_cumulative_interest_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketCumulativeInterestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_AMMS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAmmsKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl From<&UpdateAmmsAccounts<'_, '_>> for UpdateAmmsKeys {
    fn from(accounts: &UpdateAmmsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateAmmsKeys> for [AccountMeta; UPDATE_AMMS_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateAmmsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_AMMS_IX_ACCOUNTS_LEN]> for UpdateAmmsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AMMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateAmmsAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_AMMS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateAmmsAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AMMS_IX_ACCOUNTS_LEN]>
    for UpdateAmmsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AMMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateAmmsIxArgs {
    pub market_indexes: [u16; 5],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAmmsIxData(pub UpdateAmmsIxArgs);
pub const UPDATE_AMMS_IX_DISCM: [u8; 8] = [201, 106, 217, 253, 4, 175, 228, 97];
impl From<UpdateAmmsIxArgs> for UpdateAmmsIxData {
    fn from(args: UpdateAmmsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAmmsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_AMMS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_AMMS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateAmmsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AMMS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_amms_ix<K: Into<UpdateAmmsKeys>, A: Into<UpdateAmmsIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateAmmsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_AMMS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateAmmsIxArgs = args.into();
    let data: UpdateAmmsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_amms_invoke<'info, A: Into<UpdateAmmsIxArgs>>(
    accounts: &UpdateAmmsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_amms_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_AMMS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_amms_invoke_signed<'info, A: Into<UpdateAmmsIxArgs>>(
    accounts: &UpdateAmmsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_amms_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_AMMS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_amms_verify_account_keys(
    accounts: &UpdateAmmsAccounts<'_, '_>,
    keys: &UpdateAmmsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_amms_verify_account_privileges<'me, 'info>(
    accounts: &UpdateAmmsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketExpiryAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketExpiryKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketExpiryAccounts<'_, '_>> for UpdateSpotMarketExpiryKeys {
    fn from(accounts: &UpdateSpotMarketExpiryAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketExpiryKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketExpiryKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]> for UpdateSpotMarketExpiryKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketExpiryAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketExpiryAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketExpiryAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketExpiryIxArgs {
    pub expiry_ts: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketExpiryIxData(pub UpdateSpotMarketExpiryIxArgs);
pub const UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM: [u8; 8] = [208, 11, 211, 159, 226, 24, 11, 247];
impl From<UpdateSpotMarketExpiryIxArgs> for UpdateSpotMarketExpiryIxData {
    fn from(args: UpdateSpotMarketExpiryIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketExpiryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketExpiryIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_expiry_ix<
    K: Into<UpdateSpotMarketExpiryKeys>,
    A: Into<UpdateSpotMarketExpiryIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketExpiryKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketExpiryIxArgs = args.into();
    let data: UpdateSpotMarketExpiryIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_expiry_invoke<'info, A: Into<UpdateSpotMarketExpiryIxArgs>>(
    accounts: &UpdateSpotMarketExpiryAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_expiry_invoke_signed<'info, A: Into<UpdateSpotMarketExpiryIxArgs>>(
    accounts: &UpdateSpotMarketExpiryAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_expiry_verify_account_keys(
    accounts: &UpdateSpotMarketExpiryAccounts<'_, '_>,
    keys: &UpdateSpotMarketExpiryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_expiry_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketExpiryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserQuoteAssetInsuranceStakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_stake: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUserQuoteAssetInsuranceStakeKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}
impl From<&UpdateUserQuoteAssetInsuranceStakeAccounts<'_, '_>>
    for UpdateUserQuoteAssetInsuranceStakeKeys
{
    fn from(accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
        }
    }
}
impl From<&UpdateUserQuoteAssetInsuranceStakeKeys>
    for [AccountMeta; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserQuoteAssetInsuranceStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]>
    for UpdateUserQuoteAssetInsuranceStakeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            spot_market: pubkeys[1],
            insurance_fund_stake: pubkeys[2],
            user_stats: pubkeys[3],
            authority: pubkeys[4],
            insurance_fund_vault: pubkeys[5],
        }
    }
}
impl<'info> From<&UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]>
    for UpdateUserQuoteAssetInsuranceStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            spot_market: &arr[1],
            insurance_fund_stake: &arr[2],
            user_stats: &arr[3],
            authority: &arr[4],
            insurance_fund_vault: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUserQuoteAssetInsuranceStakeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUserQuoteAssetInsuranceStakeIxData(pub UpdateUserQuoteAssetInsuranceStakeIxArgs);
pub const UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM: [u8; 8] =
    [251, 101, 156, 7, 2, 63, 30, 23];
impl From<UpdateUserQuoteAssetInsuranceStakeIxArgs> for UpdateUserQuoteAssetInsuranceStakeIxData {
    fn from(args: UpdateUserQuoteAssetInsuranceStakeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUserQuoteAssetInsuranceStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateUserQuoteAssetInsuranceStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_user_quote_asset_insurance_stake_ix<
    K: Into<UpdateUserQuoteAssetInsuranceStakeKeys>,
    A: Into<UpdateUserQuoteAssetInsuranceStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserQuoteAssetInsuranceStakeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateUserQuoteAssetInsuranceStakeIxArgs = args.into();
    let data: UpdateUserQuoteAssetInsuranceStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_quote_asset_insurance_stake_invoke<
    'info,
    A: Into<UpdateUserQuoteAssetInsuranceStakeIxArgs>,
>(
    accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_user_quote_asset_insurance_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_quote_asset_insurance_stake_invoke_signed<
    'info,
    A: Into<UpdateUserQuoteAssetInsuranceStakeIxArgs>,
>(
    accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_quote_asset_insurance_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_user_quote_asset_insurance_stake_verify_account_keys(
    accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, '_>,
    keys: &UpdateUserQuoteAssetInsuranceStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_stake.key,
            &keys.insurance_fund_stake,
        ),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_user_quote_asset_insurance_stake_verify_account_privileges<'me, 'info>(
    accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.insurance_fund_stake,
        accounts.user_stats,
        accounts.insurance_fund_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct InitializeInsuranceFundStakeAccounts<'me, 'info> {
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_stake: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeInsuranceFundStakeKeys {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializeInsuranceFundStakeAccounts<'_, '_>> for InitializeInsuranceFundStakeKeys {
    fn from(accounts: &InitializeInsuranceFundStakeAccounts) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeInsuranceFundStakeKeys>
    for [AccountMeta; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &InitializeInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for InitializeInsuranceFundStakeKeys
{
    fn from(pubkeys: [Pubkey; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spot_market: pubkeys[0],
            insurance_fund_stake: pubkeys[1],
            user_stats: pubkeys[2],
            state: pubkeys[3],
            authority: pubkeys[4],
            payer: pubkeys[5],
            rent: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<&InitializeInsuranceFundStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeInsuranceFundStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for InitializeInsuranceFundStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            spot_market: &arr[0],
            insurance_fund_stake: &arr[1],
            user_stats: &arr[2],
            state: &arr[3],
            authority: &arr[4],
            payer: &arr[5],
            rent: &arr[6],
            system_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeInsuranceFundStakeIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeInsuranceFundStakeIxData(pub InitializeInsuranceFundStakeIxArgs);
pub const INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] = [187, 179, 243, 70, 248, 90, 92, 147];
impl From<InitializeInsuranceFundStakeIxArgs> for InitializeInsuranceFundStakeIxData {
    fn from(args: InitializeInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeInsuranceFundStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeInsuranceFundStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_insurance_fund_stake_ix<
    K: Into<InitializeInsuranceFundStakeKeys>,
    A: Into<InitializeInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeInsuranceFundStakeIxArgs = args.into();
    let data: InitializeInsuranceFundStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_insurance_fund_stake_invoke<
    'info,
    A: Into<InitializeInsuranceFundStakeIxArgs>,
>(
    accounts: &InitializeInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_insurance_fund_stake_invoke_signed<
    'info,
    A: Into<InitializeInsuranceFundStakeIxArgs>,
>(
    accounts: &InitializeInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_insurance_fund_stake_verify_account_keys(
    accounts: &InitializeInsuranceFundStakeAccounts<'_, '_>,
    keys: &InitializeInsuranceFundStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_stake.key,
            &keys.insurance_fund_stake,
        ),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.state.key, &keys.state),
        (accounts.authority.key, &keys.authority),
        (accounts.payer.key, &keys.payer),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_insurance_fund_stake_verify_account_privileges<'me, 'info>(
    accounts: &InitializeInsuranceFundStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.insurance_fund_stake,
        accounts.user_stats,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct AddInsuranceFundStakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_stake: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddInsuranceFundStakeKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<&AddInsuranceFundStakeAccounts<'_, '_>> for AddInsuranceFundStakeKeys {
    fn from(accounts: &AddInsuranceFundStakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&AddInsuranceFundStakeKeys> for [AccountMeta; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]> for AddInsuranceFundStakeKeys {
    fn from(pubkeys: [Pubkey; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            spot_market: pubkeys[1],
            insurance_fund_stake: pubkeys[2],
            user_stats: pubkeys[3],
            authority: pubkeys[4],
            spot_market_vault: pubkeys[5],
            insurance_fund_vault: pubkeys[6],
            drift_signer: pubkeys[7],
            user_token_account: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&AddInsuranceFundStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddInsuranceFundStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for AddInsuranceFundStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            spot_market: &arr[1],
            insurance_fund_stake: &arr[2],
            user_stats: &arr[3],
            authority: &arr[4],
            spot_market_vault: &arr[5],
            insurance_fund_vault: &arr[6],
            drift_signer: &arr[7],
            user_token_account: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddInsuranceFundStakeIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddInsuranceFundStakeIxData(pub AddInsuranceFundStakeIxArgs);
pub const ADD_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] = [251, 144, 115, 11, 222, 47, 62, 236];
impl From<AddInsuranceFundStakeIxArgs> for AddInsuranceFundStakeIxData {
    fn from(args: AddInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl AddInsuranceFundStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_INSURANCE_FUND_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_INSURANCE_FUND_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddInsuranceFundStakeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_insurance_fund_stake_ix<
    K: Into<AddInsuranceFundStakeKeys>,
    A: Into<AddInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddInsuranceFundStakeIxArgs = args.into();
    let data: AddInsuranceFundStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_insurance_fund_stake_invoke<'info, A: Into<AddInsuranceFundStakeIxArgs>>(
    accounts: &AddInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_insurance_fund_stake_invoke_signed<'info, A: Into<AddInsuranceFundStakeIxArgs>>(
    accounts: &AddInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_insurance_fund_stake_verify_account_keys(
    accounts: &AddInsuranceFundStakeAccounts<'_, '_>,
    keys: &AddInsuranceFundStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_stake.key,
            &keys.insurance_fund_stake,
        ),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.user_token_account.key, &keys.user_token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_insurance_fund_stake_verify_account_privileges<'me, 'info>(
    accounts: &AddInsuranceFundStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.insurance_fund_stake,
        accounts.user_stats,
        accounts.spot_market_vault,
        accounts.insurance_fund_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RequestRemoveInsuranceFundStakeAccounts<'me, 'info> {
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_stake: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestRemoveInsuranceFundStakeKeys {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}
impl From<&RequestRemoveInsuranceFundStakeAccounts<'_, '_>>
    for RequestRemoveInsuranceFundStakeKeys
{
    fn from(accounts: &RequestRemoveInsuranceFundStakeAccounts) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
        }
    }
}
impl From<&RequestRemoveInsuranceFundStakeKeys>
    for [AccountMeta; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RequestRemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
        ]
    }
}
impl From<[Pubkey; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for RequestRemoveInsuranceFundStakeKeys
{
    fn from(pubkeys: [Pubkey; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spot_market: pubkeys[0],
            insurance_fund_stake: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            insurance_fund_vault: pubkeys[4],
        }
    }
}
impl<'info> From<&RequestRemoveInsuranceFundStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for RequestRemoveInsuranceFundStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            spot_market: &arr[0],
            insurance_fund_stake: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            insurance_fund_vault: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestRemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestRemoveInsuranceFundStakeIxData(pub RequestRemoveInsuranceFundStakeIxArgs);
pub const REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] =
    [142, 70, 204, 92, 73, 106, 180, 52];
impl From<RequestRemoveInsuranceFundStakeIxArgs> for RequestRemoveInsuranceFundStakeIxData {
    fn from(args: RequestRemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl RequestRemoveInsuranceFundStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RequestRemoveInsuranceFundStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_remove_insurance_fund_stake_ix<
    K: Into<RequestRemoveInsuranceFundStakeKeys>,
    A: Into<RequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RequestRemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RequestRemoveInsuranceFundStakeIxArgs = args.into();
    let data: RequestRemoveInsuranceFundStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_remove_insurance_fund_stake_invoke<
    'info,
    A: Into<RequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn request_remove_insurance_fund_stake_invoke_signed<
    'info,
    A: Into<RequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn request_remove_insurance_fund_stake_verify_account_keys(
    accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, '_>,
    keys: &RequestRemoveInsuranceFundStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_stake.key,
            &keys.insurance_fund_stake,
        ),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn request_remove_insurance_fund_stake_verify_account_privileges<'me, 'info>(
    accounts: &RequestRemoveInsuranceFundStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.insurance_fund_stake,
        accounts.user_stats,
        accounts.insurance_fund_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CancelRequestRemoveInsuranceFundStakeAccounts<'me, 'info> {
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_stake: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelRequestRemoveInsuranceFundStakeKeys {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}
impl From<&CancelRequestRemoveInsuranceFundStakeAccounts<'_, '_>>
    for CancelRequestRemoveInsuranceFundStakeKeys
{
    fn from(accounts: &CancelRequestRemoveInsuranceFundStakeAccounts) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
        }
    }
}
impl From<&CancelRequestRemoveInsuranceFundStakeKeys>
    for [AccountMeta; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CancelRequestRemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
        ]
    }
}
impl From<[Pubkey; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for CancelRequestRemoveInsuranceFundStakeKeys
{
    fn from(pubkeys: [Pubkey; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spot_market: pubkeys[0],
            insurance_fund_stake: pubkeys[1],
            user_stats: pubkeys[2],
            authority: pubkeys[3],
            insurance_fund_vault: pubkeys[4],
        }
    }
}
impl<'info> From<&CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for CancelRequestRemoveInsuranceFundStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            spot_market: &arr[0],
            insurance_fund_stake: &arr[1],
            user_stats: &arr[2],
            authority: &arr[3],
            insurance_fund_vault: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelRequestRemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelRequestRemoveInsuranceFundStakeIxData(
    pub CancelRequestRemoveInsuranceFundStakeIxArgs,
);
pub const CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] =
    [97, 235, 78, 62, 212, 42, 241, 127];
impl From<CancelRequestRemoveInsuranceFundStakeIxArgs>
    for CancelRequestRemoveInsuranceFundStakeIxData
{
    fn from(args: CancelRequestRemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl CancelRequestRemoveInsuranceFundStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            CancelRequestRemoveInsuranceFundStakeIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_request_remove_insurance_fund_stake_ix<
    K: Into<CancelRequestRemoveInsuranceFundStakeKeys>,
    A: Into<CancelRequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelRequestRemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: CancelRequestRemoveInsuranceFundStakeIxArgs = args.into();
    let data: CancelRequestRemoveInsuranceFundStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_request_remove_insurance_fund_stake_invoke<
    'info,
    A: Into<CancelRequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_request_remove_insurance_fund_stake_invoke_signed<
    'info,
    A: Into<CancelRequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_request_remove_insurance_fund_stake_verify_account_keys(
    accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, '_>,
    keys: &CancelRequestRemoveInsuranceFundStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_stake.key,
            &keys.insurance_fund_stake,
        ),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_request_remove_insurance_fund_stake_verify_account_privileges<'me, 'info>(
    accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.insurance_fund_stake,
        accounts.user_stats,
        accounts.insurance_fund_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RemoveInsuranceFundStakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_stake: &'me AccountInfo<'info>,
    pub user_stats: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub user_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveInsuranceFundStakeKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<&RemoveInsuranceFundStakeAccounts<'_, '_>> for RemoveInsuranceFundStakeKeys {
    fn from(accounts: &RemoveInsuranceFundStakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&RemoveInsuranceFundStakeKeys>
    for [AccountMeta; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]> for RemoveInsuranceFundStakeKeys {
    fn from(pubkeys: [Pubkey; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            spot_market: pubkeys[1],
            insurance_fund_stake: pubkeys[2],
            user_stats: pubkeys[3],
            authority: pubkeys[4],
            insurance_fund_vault: pubkeys[5],
            drift_signer: pubkeys[6],
            user_token_account: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&RemoveInsuranceFundStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveInsuranceFundStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for RemoveInsuranceFundStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            spot_market: &arr[1],
            insurance_fund_stake: &arr[2],
            user_stats: &arr[3],
            authority: &arr[4],
            insurance_fund_vault: &arr[5],
            drift_signer: &arr[6],
            user_token_account: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveInsuranceFundStakeIxData(pub RemoveInsuranceFundStakeIxArgs);
pub const REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] = [128, 166, 142, 9, 254, 187, 143, 174];
impl From<RemoveInsuranceFundStakeIxArgs> for RemoveInsuranceFundStakeIxData {
    fn from(args: RemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveInsuranceFundStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REMOVE_INSURANCE_FUND_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_INSURANCE_FUND_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveInsuranceFundStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_insurance_fund_stake_ix<
    K: Into<RemoveInsuranceFundStakeKeys>,
    A: Into<RemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveInsuranceFundStakeIxArgs = args.into();
    let data: RemoveInsuranceFundStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_insurance_fund_stake_invoke<'info, A: Into<RemoveInsuranceFundStakeIxArgs>>(
    accounts: &RemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_insurance_fund_stake_invoke_signed<'info, A: Into<RemoveInsuranceFundStakeIxArgs>>(
    accounts: &RemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_insurance_fund_stake_verify_account_keys(
    accounts: &RemoveInsuranceFundStakeAccounts<'_, '_>,
    keys: &RemoveInsuranceFundStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_stake.key,
            &keys.insurance_fund_stake,
        ),
        (accounts.user_stats.key, &keys.user_stats),
        (accounts.authority.key, &keys.authority),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.user_token_account.key, &keys.user_token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_insurance_fund_stake_verify_account_privileges<'me, 'info>(
    accounts: &RemoveInsuranceFundStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.insurance_fund_stake,
        accounts.user_stats,
        accounts.insurance_fund_vault,
        accounts.user_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub quote_asset_mint: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub quote_asset_mint: Pubkey,
    pub drift_signer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: &InitializeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            quote_asset_mint: *accounts.quote_asset_mint.key,
            drift_signer: *accounts.drift_signer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeKeys) -> Self {
        [
            AccountMeta::new(keys.admin, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.quote_asset_mint, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            quote_asset_mint: pubkeys[2],
            drift_signer: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<&InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.quote_asset_mint.clone(),
            accounts.drift_signer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            quote_asset_mint: &arr[2],
            drift_signer: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
            token_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix<K: Into<InitializeKeys>, A: Into<InitializeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeIxArgs = args.into();
    let data: InitializeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_invoke<'info, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_invoke_signed<'info, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: &InitializeAccounts<'_, '_>,
    keys: &InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.quote_asset_mint.key, &keys.quote_asset_mint),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: &InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeSpotMarketAccounts<'me, 'info> {
    pub spot_market: &'me AccountInfo<'info>,
    pub spot_market_mint: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeSpotMarketKeys {
    pub spot_market: Pubkey,
    pub spot_market_mint: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub state: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&InitializeSpotMarketAccounts<'_, '_>> for InitializeSpotMarketKeys {
    fn from(accounts: &InitializeSpotMarketAccounts) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            spot_market_mint: *accounts.spot_market_mint.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            state: *accounts.state.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&InitializeSpotMarketKeys> for [AccountMeta; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeSpotMarketKeys) -> Self {
        [
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new_readonly(keys.spot_market_mint, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN]> for InitializeSpotMarketKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spot_market: pubkeys[0],
            spot_market_mint: pubkeys[1],
            spot_market_vault: pubkeys[2],
            insurance_fund_vault: pubkeys[3],
            drift_signer: pubkeys[4],
            state: pubkeys[5],
            oracle: pubkeys[6],
            admin: pubkeys[7],
            rent: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<&InitializeSpotMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeSpotMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.spot_market_mint.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.state.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN]>
    for InitializeSpotMarketAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            spot_market: &arr[0],
            spot_market_mint: &arr[1],
            spot_market_vault: &arr[2],
            insurance_fund_vault: &arr[3],
            drift_signer: &arr[4],
            state: &arr[5],
            oracle: &arr[6],
            admin: &arr[7],
            rent: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeSpotMarketIxArgs {
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
    pub oracle_source: OracleSource,
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
    pub liquidator_fee: u32,
    pub active_status: bool,
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeSpotMarketIxData(pub InitializeSpotMarketIxArgs);
pub const INITIALIZE_SPOT_MARKET_IX_DISCM: [u8; 8] = [234, 196, 128, 44, 94, 15, 48, 201];
impl From<InitializeSpotMarketIxArgs> for InitializeSpotMarketIxData {
    fn from(args: InitializeSpotMarketIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeSpotMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_SPOT_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_SPOT_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeSpotMarketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_SPOT_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_spot_market_ix<
    K: Into<InitializeSpotMarketKeys>,
    A: Into<InitializeSpotMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeSpotMarketKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeSpotMarketIxArgs = args.into();
    let data: InitializeSpotMarketIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_spot_market_invoke<'info, A: Into<InitializeSpotMarketIxArgs>>(
    accounts: &InitializeSpotMarketAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_spot_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_spot_market_invoke_signed<'info, A: Into<InitializeSpotMarketIxArgs>>(
    accounts: &InitializeSpotMarketAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_spot_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_spot_market_verify_account_keys(
    accounts: &InitializeSpotMarketAccounts<'_, '_>,
    keys: &InitializeSpotMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.spot_market.key, &keys.spot_market),
        (accounts.spot_market_mint.key, &keys.spot_market_mint),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.state.key, &keys.state),
        (accounts.oracle.key, &keys.oracle),
        (accounts.admin.key, &keys.admin),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_spot_market_verify_account_privileges<'me, 'info>(
    accounts: &InitializeSpotMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.spot_market,
        accounts.spot_market_vault,
        accounts.insurance_fund_vault,
        accounts.state,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeSerumFulfillmentConfigAccounts<'me, 'info> {
    pub base_spot_market: &'me AccountInfo<'info>,
    pub quote_spot_market: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub serum_program: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_open_orders: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub serum_fulfillment_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeSerumFulfillmentConfigKeys {
    pub base_spot_market: Pubkey,
    pub quote_spot_market: Pubkey,
    pub state: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_open_orders: Pubkey,
    pub drift_signer: Pubkey,
    pub serum_fulfillment_config: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializeSerumFulfillmentConfigAccounts<'_, '_>>
    for InitializeSerumFulfillmentConfigKeys
{
    fn from(accounts: &InitializeSerumFulfillmentConfigAccounts) -> Self {
        Self {
            base_spot_market: *accounts.base_spot_market.key,
            quote_spot_market: *accounts.quote_spot_market.key,
            state: *accounts.state.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_open_orders: *accounts.serum_open_orders.key,
            drift_signer: *accounts.drift_signer.key,
            serum_fulfillment_config: *accounts.serum_fulfillment_config.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeSerumFulfillmentConfigKeys>
    for [AccountMeta; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(keys: &InitializeSerumFulfillmentConfigKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.base_spot_market, false),
            AccountMeta::new_readonly(keys.quote_spot_market, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.serum_program, false),
            AccountMeta::new_readonly(keys.serum_market, false),
            AccountMeta::new(keys.serum_open_orders, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.serum_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]>
    for InitializeSerumFulfillmentConfigKeys
{
    fn from(pubkeys: [Pubkey; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base_spot_market: pubkeys[0],
            quote_spot_market: pubkeys[1],
            state: pubkeys[2],
            serum_program: pubkeys[3],
            serum_market: pubkeys[4],
            serum_open_orders: pubkeys[5],
            drift_signer: pubkeys[6],
            serum_fulfillment_config: pubkeys[7],
            admin: pubkeys[8],
            rent: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<&InitializeSerumFulfillmentConfigAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeSerumFulfillmentConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.base_spot_market.clone(),
            accounts.quote_spot_market.clone(),
            accounts.state.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_open_orders.clone(),
            accounts.drift_signer.clone(),
            accounts.serum_fulfillment_config.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]>
    for InitializeSerumFulfillmentConfigAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base_spot_market: &arr[0],
            quote_spot_market: &arr[1],
            state: &arr[2],
            serum_program: &arr[3],
            serum_market: &arr[4],
            serum_open_orders: &arr[5],
            drift_signer: &arr[6],
            serum_fulfillment_config: &arr[7],
            admin: &arr[8],
            rent: &arr[9],
            system_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeSerumFulfillmentConfigIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeSerumFulfillmentConfigIxData(pub InitializeSerumFulfillmentConfigIxArgs);
pub const INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM: [u8; 8] =
    [193, 211, 132, 172, 70, 171, 7, 94];
impl From<InitializeSerumFulfillmentConfigIxArgs> for InitializeSerumFulfillmentConfigIxData {
    fn from(args: InitializeSerumFulfillmentConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeSerumFulfillmentConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeSerumFulfillmentConfigIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_serum_fulfillment_config_ix<
    K: Into<InitializeSerumFulfillmentConfigKeys>,
    A: Into<InitializeSerumFulfillmentConfigIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeSerumFulfillmentConfigKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeSerumFulfillmentConfigIxArgs = args.into();
    let data: InitializeSerumFulfillmentConfigIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_serum_fulfillment_config_invoke<
    'info,
    A: Into<InitializeSerumFulfillmentConfigIxArgs>,
>(
    accounts: &InitializeSerumFulfillmentConfigAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_serum_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_serum_fulfillment_config_invoke_signed<
    'info,
    A: Into<InitializeSerumFulfillmentConfigIxArgs>,
>(
    accounts: &InitializeSerumFulfillmentConfigAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_serum_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_serum_fulfillment_config_verify_account_keys(
    accounts: &InitializeSerumFulfillmentConfigAccounts<'_, '_>,
    keys: &InitializeSerumFulfillmentConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.base_spot_market.key, &keys.base_spot_market),
        (accounts.quote_spot_market.key, &keys.quote_spot_market),
        (accounts.state.key, &keys.state),
        (accounts.serum_program.key, &keys.serum_program),
        (accounts.serum_market.key, &keys.serum_market),
        (accounts.serum_open_orders.key, &keys.serum_open_orders),
        (accounts.drift_signer.key, &keys.drift_signer),
        (
            accounts.serum_fulfillment_config.key,
            &keys.serum_fulfillment_config,
        ),
        (accounts.admin.key, &keys.admin),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_serum_fulfillment_config_verify_account_privileges<'me, 'info>(
    accounts: &InitializeSerumFulfillmentConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.serum_open_orders,
        accounts.serum_fulfillment_config,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumFulfillmentConfigStatusAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub serum_fulfillment_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSerumFulfillmentConfigStatusKeys {
    pub state: Pubkey,
    pub serum_fulfillment_config: Pubkey,
    pub admin: Pubkey,
}
impl From<&UpdateSerumFulfillmentConfigStatusAccounts<'_, '_>>
    for UpdateSerumFulfillmentConfigStatusKeys
{
    fn from(accounts: &UpdateSerumFulfillmentConfigStatusAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            serum_fulfillment_config: *accounts.serum_fulfillment_config.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&UpdateSerumFulfillmentConfigStatusKeys>
    for [AccountMeta; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSerumFulfillmentConfigStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.serum_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]>
    for UpdateSerumFulfillmentConfigStatusKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            serum_fulfillment_config: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSerumFulfillmentConfigStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.serum_fulfillment_config.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]>
    for UpdateSerumFulfillmentConfigStatusAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            serum_fulfillment_config: &arr[1],
            admin: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSerumFulfillmentConfigStatusIxArgs {
    pub status: SpotFulfillmentConfigStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSerumFulfillmentConfigStatusIxData(pub UpdateSerumFulfillmentConfigStatusIxArgs);
pub const UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM: [u8; 8] =
    [171, 109, 240, 251, 95, 1, 149, 89];
impl From<UpdateSerumFulfillmentConfigStatusIxArgs> for UpdateSerumFulfillmentConfigStatusIxData {
    fn from(args: UpdateSerumFulfillmentConfigStatusIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSerumFulfillmentConfigStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSerumFulfillmentConfigStatusIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_serum_fulfillment_config_status_ix<
    K: Into<UpdateSerumFulfillmentConfigStatusKeys>,
    A: Into<UpdateSerumFulfillmentConfigStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSerumFulfillmentConfigStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSerumFulfillmentConfigStatusIxArgs = args.into();
    let data: UpdateSerumFulfillmentConfigStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_serum_fulfillment_config_status_invoke<
    'info,
    A: Into<UpdateSerumFulfillmentConfigStatusIxArgs>,
>(
    accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_serum_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_serum_fulfillment_config_status_invoke_signed<
    'info,
    A: Into<UpdateSerumFulfillmentConfigStatusIxArgs>,
>(
    accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_serum_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_serum_fulfillment_config_status_verify_account_keys(
    accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, '_>,
    keys: &UpdateSerumFulfillmentConfigStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (
            accounts.serum_fulfillment_config.key,
            &keys.serum_fulfillment_config,
        ),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_serum_fulfillment_config_status_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.serum_fulfillment_config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializePhoenixFulfillmentConfigAccounts<'me, 'info> {
    pub base_spot_market: &'me AccountInfo<'info>,
    pub quote_spot_market: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub phoenix_program: &'me AccountInfo<'info>,
    pub phoenix_market: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub phoenix_fulfillment_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePhoenixFulfillmentConfigKeys {
    pub base_spot_market: Pubkey,
    pub quote_spot_market: Pubkey,
    pub state: Pubkey,
    pub phoenix_program: Pubkey,
    pub phoenix_market: Pubkey,
    pub drift_signer: Pubkey,
    pub phoenix_fulfillment_config: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializePhoenixFulfillmentConfigAccounts<'_, '_>>
    for InitializePhoenixFulfillmentConfigKeys
{
    fn from(accounts: &InitializePhoenixFulfillmentConfigAccounts) -> Self {
        Self {
            base_spot_market: *accounts.base_spot_market.key,
            quote_spot_market: *accounts.quote_spot_market.key,
            state: *accounts.state.key,
            phoenix_program: *accounts.phoenix_program.key,
            phoenix_market: *accounts.phoenix_market.key,
            drift_signer: *accounts.drift_signer.key,
            phoenix_fulfillment_config: *accounts.phoenix_fulfillment_config.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializePhoenixFulfillmentConfigKeys>
    for [AccountMeta; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(keys: &InitializePhoenixFulfillmentConfigKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.base_spot_market, false),
            AccountMeta::new_readonly(keys.quote_spot_market, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.phoenix_market, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.phoenix_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]>
    for InitializePhoenixFulfillmentConfigKeys
{
    fn from(pubkeys: [Pubkey; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            base_spot_market: pubkeys[0],
            quote_spot_market: pubkeys[1],
            state: pubkeys[2],
            phoenix_program: pubkeys[3],
            phoenix_market: pubkeys[4],
            drift_signer: pubkeys[5],
            phoenix_fulfillment_config: pubkeys[6],
            admin: pubkeys[7],
            rent: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<&InitializePhoenixFulfillmentConfigAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializePhoenixFulfillmentConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.base_spot_market.clone(),
            accounts.quote_spot_market.clone(),
            accounts.state.clone(),
            accounts.phoenix_program.clone(),
            accounts.phoenix_market.clone(),
            accounts.drift_signer.clone(),
            accounts.phoenix_fulfillment_config.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]>
    for InitializePhoenixFulfillmentConfigAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            base_spot_market: &arr[0],
            quote_spot_market: &arr[1],
            state: &arr[2],
            phoenix_program: &arr[3],
            phoenix_market: &arr[4],
            drift_signer: &arr[5],
            phoenix_fulfillment_config: &arr[6],
            admin: &arr[7],
            rent: &arr[8],
            system_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePhoenixFulfillmentConfigIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePhoenixFulfillmentConfigIxData(pub InitializePhoenixFulfillmentConfigIxArgs);
pub const INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM: [u8; 8] =
    [135, 132, 110, 107, 185, 160, 169, 154];
impl From<InitializePhoenixFulfillmentConfigIxArgs> for InitializePhoenixFulfillmentConfigIxData {
    fn from(args: InitializePhoenixFulfillmentConfigIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePhoenixFulfillmentConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializePhoenixFulfillmentConfigIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_phoenix_fulfillment_config_ix<
    K: Into<InitializePhoenixFulfillmentConfigKeys>,
    A: Into<InitializePhoenixFulfillmentConfigIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializePhoenixFulfillmentConfigKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: InitializePhoenixFulfillmentConfigIxArgs = args.into();
    let data: InitializePhoenixFulfillmentConfigIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_phoenix_fulfillment_config_invoke<
    'info,
    A: Into<InitializePhoenixFulfillmentConfigIxArgs>,
>(
    accounts: &InitializePhoenixFulfillmentConfigAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_phoenix_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_phoenix_fulfillment_config_invoke_signed<
    'info,
    A: Into<InitializePhoenixFulfillmentConfigIxArgs>,
>(
    accounts: &InitializePhoenixFulfillmentConfigAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_phoenix_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_phoenix_fulfillment_config_verify_account_keys(
    accounts: &InitializePhoenixFulfillmentConfigAccounts<'_, '_>,
    keys: &InitializePhoenixFulfillmentConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.base_spot_market.key, &keys.base_spot_market),
        (accounts.quote_spot_market.key, &keys.quote_spot_market),
        (accounts.state.key, &keys.state),
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.phoenix_market.key, &keys.phoenix_market),
        (accounts.drift_signer.key, &keys.drift_signer),
        (
            accounts.phoenix_fulfillment_config.key,
            &keys.phoenix_fulfillment_config,
        ),
        (accounts.admin.key, &keys.admin),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_phoenix_fulfillment_config_verify_account_privileges<'me, 'info>(
    accounts: &InitializePhoenixFulfillmentConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.phoenix_fulfillment_config,
        accounts.admin,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct PhoenixFulfillmentConfigStatusAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub phoenix_fulfillment_config: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PhoenixFulfillmentConfigStatusKeys {
    pub state: Pubkey,
    pub phoenix_fulfillment_config: Pubkey,
    pub admin: Pubkey,
}
impl From<&PhoenixFulfillmentConfigStatusAccounts<'_, '_>> for PhoenixFulfillmentConfigStatusKeys {
    fn from(accounts: &PhoenixFulfillmentConfigStatusAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            phoenix_fulfillment_config: *accounts.phoenix_fulfillment_config.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&PhoenixFulfillmentConfigStatusKeys>
    for [AccountMeta; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &PhoenixFulfillmentConfigStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.phoenix_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
        ]
    }
}
impl From<[Pubkey; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]>
    for PhoenixFulfillmentConfigStatusKeys
{
    fn from(pubkeys: [Pubkey; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            phoenix_fulfillment_config: pubkeys[1],
            admin: pubkeys[2],
        }
    }
}
impl<'info> From<&PhoenixFulfillmentConfigStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.phoenix_fulfillment_config.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]>
    for PhoenixFulfillmentConfigStatusAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            phoenix_fulfillment_config: &arr[1],
            admin: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoenixFulfillmentConfigStatusIxArgs {
    pub status: SpotFulfillmentConfigStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PhoenixFulfillmentConfigStatusIxData(pub PhoenixFulfillmentConfigStatusIxArgs);
pub const PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM: [u8; 8] = [96, 31, 113, 32, 12, 203, 7, 154];
impl From<PhoenixFulfillmentConfigStatusIxArgs> for PhoenixFulfillmentConfigStatusIxData {
    fn from(args: PhoenixFulfillmentConfigStatusIxArgs) -> Self {
        Self(args)
    }
}
impl PhoenixFulfillmentConfigStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PhoenixFulfillmentConfigStatusIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn phoenix_fulfillment_config_status_ix<
    K: Into<PhoenixFulfillmentConfigStatusKeys>,
    A: Into<PhoenixFulfillmentConfigStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PhoenixFulfillmentConfigStatusKeys = accounts.into();
    let metas: [AccountMeta; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PhoenixFulfillmentConfigStatusIxArgs = args.into();
    let data: PhoenixFulfillmentConfigStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn phoenix_fulfillment_config_status_invoke<
    'info,
    A: Into<PhoenixFulfillmentConfigStatusIxArgs>,
>(
    accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = phoenix_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn phoenix_fulfillment_config_status_invoke_signed<
    'info,
    A: Into<PhoenixFulfillmentConfigStatusIxArgs>,
>(
    accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = phoenix_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn phoenix_fulfillment_config_status_verify_account_keys(
    accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, '_>,
    keys: &PhoenixFulfillmentConfigStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (
            accounts.phoenix_fulfillment_config.key,
            &keys.phoenix_fulfillment_config,
        ),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn phoenix_fulfillment_config_status_verify_account_privileges<'me, 'info>(
    accounts: &PhoenixFulfillmentConfigStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.phoenix_fulfillment_config, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumVaultAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub srm_vault: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSerumVaultKeys {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub srm_vault: Pubkey,
}
impl From<&UpdateSerumVaultAccounts<'_, '_>> for UpdateSerumVaultKeys {
    fn from(accounts: &UpdateSerumVaultAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            srm_vault: *accounts.srm_vault.key,
        }
    }
}
impl From<&UpdateSerumVaultKeys> for [AccountMeta; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateSerumVaultKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.srm_vault, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN]> for UpdateSerumVaultKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            admin: pubkeys[1],
            srm_vault: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSerumVaultAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSerumVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.srm_vault.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN]>
    for UpdateSerumVaultAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            admin: &arr[1],
            srm_vault: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSerumVaultIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSerumVaultIxData(pub UpdateSerumVaultIxArgs);
pub const UPDATE_SERUM_VAULT_IX_DISCM: [u8; 8] = [219, 8, 246, 96, 169, 121, 91, 110];
impl From<UpdateSerumVaultIxArgs> for UpdateSerumVaultIxData {
    fn from(args: UpdateSerumVaultIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSerumVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SERUM_VAULT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SERUM_VAULT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSerumVaultIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SERUM_VAULT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_serum_vault_ix<K: Into<UpdateSerumVaultKeys>, A: Into<UpdateSerumVaultIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSerumVaultKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSerumVaultIxArgs = args.into();
    let data: UpdateSerumVaultIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_serum_vault_invoke<'info, A: Into<UpdateSerumVaultIxArgs>>(
    accounts: &UpdateSerumVaultAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_serum_vault_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_serum_vault_invoke_signed<'info, A: Into<UpdateSerumVaultIxArgs>>(
    accounts: &UpdateSerumVaultAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_serum_vault_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_serum_vault_verify_account_keys(
    accounts: &UpdateSerumVaultAccounts<'_, '_>,
    keys: &UpdateSerumVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.admin.key, &keys.admin),
        (accounts.srm_vault.key, &keys.srm_vault),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_serum_vault_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSerumVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.admin] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitializePerpMarketAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializePerpMarketKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitializePerpMarketAccounts<'_, '_>> for InitializePerpMarketKeys {
    fn from(accounts: &InitializePerpMarketAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializePerpMarketKeys> for [AccountMeta; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializePerpMarketKeys) -> Self {
        [
            AccountMeta::new(keys.admin, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN]> for InitializePerpMarketKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
            oracle: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<&InitializePerpMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializePerpMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN]>
    for InitializePerpMarketAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
            oracle: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializePerpMarketIxArgs {
    pub market_index: u16,
    pub amm_base_asset_reserve: u128,
    pub amm_quote_asset_reserve: u128,
    pub amm_periodicity: i64,
    pub amm_peg_multiplier: u128,
    pub oracle_source: OracleSource,
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
    pub liquidator_fee: u32,
    pub active_status: bool,
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializePerpMarketIxData(pub InitializePerpMarketIxArgs);
pub const INITIALIZE_PERP_MARKET_IX_DISCM: [u8; 8] = [132, 9, 229, 118, 117, 118, 117, 62];
impl From<InitializePerpMarketIxArgs> for InitializePerpMarketIxData {
    fn from(args: InitializePerpMarketIxArgs) -> Self {
        Self(args)
    }
}
impl InitializePerpMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_PERP_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_PERP_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializePerpMarketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERP_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_perp_market_ix<
    K: Into<InitializePerpMarketKeys>,
    A: Into<InitializePerpMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializePerpMarketKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializePerpMarketIxArgs = args.into();
    let data: InitializePerpMarketIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_perp_market_invoke<'info, A: Into<InitializePerpMarketIxArgs>>(
    accounts: &InitializePerpMarketAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_perp_market_invoke_signed<'info, A: Into<InitializePerpMarketIxArgs>>(
    accounts: &InitializePerpMarketAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_perp_market_verify_account_keys(
    accounts: &InitializePerpMarketAccounts<'_, '_>,
    keys: &InitializePerpMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_perp_market_verify_account_privileges<'me, 'info>(
    accounts: &InitializePerpMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.state, accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DeleteInitializedPerpMarketAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteInitializedPerpMarketKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&DeleteInitializedPerpMarketAccounts<'_, '_>> for DeleteInitializedPerpMarketKeys {
    fn from(accounts: &DeleteInitializedPerpMarketAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&DeleteInitializedPerpMarketKeys>
    for [AccountMeta; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeleteInitializedPerpMarketKeys) -> Self {
        [
            AccountMeta::new(keys.admin, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]>
    for DeleteInitializedPerpMarketKeys
{
    fn from(pubkeys: [Pubkey; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&DeleteInitializedPerpMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeleteInitializedPerpMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]>
    for DeleteInitializedPerpMarketAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteInitializedPerpMarketIxArgs {
    pub market_index: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteInitializedPerpMarketIxData(pub DeleteInitializedPerpMarketIxArgs);
pub const DELETE_INITIALIZED_PERP_MARKET_IX_DISCM: [u8; 8] = [91, 154, 24, 87, 106, 59, 190, 66];
impl From<DeleteInitializedPerpMarketIxArgs> for DeleteInitializedPerpMarketIxData {
    fn from(args: DeleteInitializedPerpMarketIxArgs) -> Self {
        Self(args)
    }
}
impl DeleteInitializedPerpMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_INITIALIZED_PERP_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DELETE_INITIALIZED_PERP_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeleteInitializedPerpMarketIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_INITIALIZED_PERP_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_initialized_perp_market_ix<
    K: Into<DeleteInitializedPerpMarketKeys>,
    A: Into<DeleteInitializedPerpMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeleteInitializedPerpMarketKeys = accounts.into();
    let metas: [AccountMeta; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeleteInitializedPerpMarketIxArgs = args.into();
    let data: DeleteInitializedPerpMarketIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delete_initialized_perp_market_invoke<'info, A: Into<DeleteInitializedPerpMarketIxArgs>>(
    accounts: &DeleteInitializedPerpMarketAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = delete_initialized_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn delete_initialized_perp_market_invoke_signed<
    'info,
    A: Into<DeleteInitializedPerpMarketIxArgs>,
>(
    accounts: &DeleteInitializedPerpMarketAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = delete_initialized_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn delete_initialized_perp_market_verify_account_keys(
    accounts: &DeleteInitializedPerpMarketAccounts<'_, '_>,
    keys: &DeleteInitializedPerpMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn delete_initialized_perp_market_verify_account_privileges<'me, 'info>(
    accounts: &DeleteInitializedPerpMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.state, accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const MOVE_AMM_PRICE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MoveAmmPriceAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MoveAmmPriceKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&MoveAmmPriceAccounts<'_, '_>> for MoveAmmPriceKeys {
    fn from(accounts: &MoveAmmPriceAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&MoveAmmPriceKeys> for [AccountMeta; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: &MoveAmmPriceKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN]> for MoveAmmPriceKeys {
    fn from(pubkeys: [Pubkey; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&MoveAmmPriceAccounts<'_, 'info>>
    for [AccountInfo<'info>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &MoveAmmPriceAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN]>
    for MoveAmmPriceAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveAmmPriceIxArgs {
    pub base_asset_reserve: u128,
    pub quote_asset_reserve: u128,
    pub sqrt_k: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MoveAmmPriceIxData(pub MoveAmmPriceIxArgs);
pub const MOVE_AMM_PRICE_IX_DISCM: [u8; 8] = [235, 109, 2, 82, 219, 118, 6, 159];
impl From<MoveAmmPriceIxArgs> for MoveAmmPriceIxData {
    fn from(args: MoveAmmPriceIxArgs) -> Self {
        Self(args)
    }
}
impl MoveAmmPriceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MOVE_AMM_PRICE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MOVE_AMM_PRICE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MoveAmmPriceIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MOVE_AMM_PRICE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn move_amm_price_ix<K: Into<MoveAmmPriceKeys>, A: Into<MoveAmmPriceIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MoveAmmPriceKeys = accounts.into();
    let metas: [AccountMeta; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: MoveAmmPriceIxArgs = args.into();
    let data: MoveAmmPriceIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn move_amm_price_invoke<'info, A: Into<MoveAmmPriceIxArgs>>(
    accounts: &MoveAmmPriceAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = move_amm_price_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn move_amm_price_invoke_signed<'info, A: Into<MoveAmmPriceIxArgs>>(
    accounts: &MoveAmmPriceAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = move_amm_price_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn move_amm_price_verify_account_keys(
    accounts: &MoveAmmPriceAccounts<'_, '_>,
    keys: &MoveAmmPriceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn move_amm_price_verify_account_privileges<'me, 'info>(
    accounts: &MoveAmmPriceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketExpiryAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketExpiryKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketExpiryAccounts<'_, '_>> for UpdatePerpMarketExpiryKeys {
    fn from(accounts: &UpdatePerpMarketExpiryAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketExpiryKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketExpiryKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]> for UpdatePerpMarketExpiryKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketExpiryAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketExpiryAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketExpiryAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketExpiryIxArgs {
    pub expiry_ts: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketExpiryIxData(pub UpdatePerpMarketExpiryIxArgs);
pub const UPDATE_PERP_MARKET_EXPIRY_IX_DISCM: [u8; 8] = [44, 221, 227, 151, 131, 140, 22, 110];
impl From<UpdatePerpMarketExpiryIxArgs> for UpdatePerpMarketExpiryIxData {
    fn from(args: UpdatePerpMarketExpiryIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketExpiryIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_EXPIRY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_EXPIRY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketExpiryIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_EXPIRY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_expiry_ix<
    K: Into<UpdatePerpMarketExpiryKeys>,
    A: Into<UpdatePerpMarketExpiryIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketExpiryKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketExpiryIxArgs = args.into();
    let data: UpdatePerpMarketExpiryIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_expiry_invoke<'info, A: Into<UpdatePerpMarketExpiryIxArgs>>(
    accounts: &UpdatePerpMarketExpiryAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_expiry_invoke_signed<'info, A: Into<UpdatePerpMarketExpiryIxArgs>>(
    accounts: &UpdatePerpMarketExpiryAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_expiry_verify_account_keys(
    accounts: &UpdatePerpMarketExpiryAccounts<'_, '_>,
    keys: &UpdatePerpMarketExpiryKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_expiry_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketExpiryAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketPoolsToRevenuePoolAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SettleExpiredMarketPoolsToRevenuePoolKeys {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub spot_market: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, '_>>
    for SettleExpiredMarketPoolsToRevenuePoolKeys
{
    fn from(accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            spot_market: *accounts.spot_market.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&SettleExpiredMarketPoolsToRevenuePoolKeys>
    for [AccountMeta; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SettleExpiredMarketPoolsToRevenuePoolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN]>
    for SettleExpiredMarketPoolsToRevenuePoolKeys
{
    fn from(
        pubkeys: [Pubkey; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: pubkeys[0],
            admin: pubkeys[1],
            spot_market: pubkeys[2],
            perp_market: pubkeys[3],
        }
    }
}
impl<'info> From<&SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.spot_market.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN]>
    for SettleExpiredMarketPoolsToRevenuePoolAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            admin: &arr[1],
            spot_market: &arr[2],
            perp_market: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleExpiredMarketPoolsToRevenuePoolIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SettleExpiredMarketPoolsToRevenuePoolIxData(
    pub SettleExpiredMarketPoolsToRevenuePoolIxArgs,
);
pub const SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM: [u8; 8] =
    [55, 19, 238, 169, 227, 90, 200, 184];
impl From<SettleExpiredMarketPoolsToRevenuePoolIxArgs>
    for SettleExpiredMarketPoolsToRevenuePoolIxData
{
    fn from(args: SettleExpiredMarketPoolsToRevenuePoolIxArgs) -> Self {
        Self(args)
    }
}
impl SettleExpiredMarketPoolsToRevenuePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            SettleExpiredMarketPoolsToRevenuePoolIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn settle_expired_market_pools_to_revenue_pool_ix<
    K: Into<SettleExpiredMarketPoolsToRevenuePoolKeys>,
    A: Into<SettleExpiredMarketPoolsToRevenuePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleExpiredMarketPoolsToRevenuePoolKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: SettleExpiredMarketPoolsToRevenuePoolIxArgs = args.into();
    let data: SettleExpiredMarketPoolsToRevenuePoolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_expired_market_pools_to_revenue_pool_invoke<
    'info,
    A: Into<SettleExpiredMarketPoolsToRevenuePoolIxArgs>,
>(
    accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = settle_expired_market_pools_to_revenue_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_expired_market_pools_to_revenue_pool_invoke_signed<
    'info,
    A: Into<SettleExpiredMarketPoolsToRevenuePoolIxArgs>,
>(
    accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_expired_market_pools_to_revenue_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn settle_expired_market_pools_to_revenue_pool_verify_account_keys(
    accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, '_>,
    keys: &SettleExpiredMarketPoolsToRevenuePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.admin.key, &keys.admin),
        (accounts.spot_market.key, &keys.spot_market),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn settle_expired_market_pools_to_revenue_pool_verify_account_privileges<'me, 'info>(
    accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market, accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DepositIntoPerpMarketFeePoolAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub source_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub quote_spot_market: &'me AccountInfo<'info>,
    pub spot_market_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositIntoPerpMarketFeePoolKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub admin: Pubkey,
    pub source_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub quote_spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<&DepositIntoPerpMarketFeePoolAccounts<'_, '_>> for DepositIntoPerpMarketFeePoolKeys {
    fn from(accounts: &DepositIntoPerpMarketFeePoolAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            admin: *accounts.admin.key,
            source_vault: *accounts.source_vault.key,
            drift_signer: *accounts.drift_signer.key,
            quote_spot_market: *accounts.quote_spot_market.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&DepositIntoPerpMarketFeePoolKeys>
    for [AccountMeta; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DepositIntoPerpMarketFeePoolKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.source_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.quote_spot_market, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]>
    for DepositIntoPerpMarketFeePoolKeys
{
    fn from(pubkeys: [Pubkey; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            perp_market: pubkeys[1],
            admin: pubkeys[2],
            source_vault: pubkeys[3],
            drift_signer: pubkeys[4],
            quote_spot_market: pubkeys[5],
            spot_market_vault: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<&DepositIntoPerpMarketFeePoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.admin.clone(),
            accounts.source_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.quote_spot_market.clone(),
            accounts.spot_market_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]>
    for DepositIntoPerpMarketFeePoolAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            perp_market: &arr[1],
            admin: &arr[2],
            source_vault: &arr[3],
            drift_signer: &arr[4],
            quote_spot_market: &arr[5],
            spot_market_vault: &arr[6],
            token_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositIntoPerpMarketFeePoolIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIntoPerpMarketFeePoolIxData(pub DepositIntoPerpMarketFeePoolIxArgs);
pub const DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM: [u8; 8] = [34, 58, 57, 68, 97, 80, 244, 6];
impl From<DepositIntoPerpMarketFeePoolIxArgs> for DepositIntoPerpMarketFeePoolIxData {
    fn from(args: DepositIntoPerpMarketFeePoolIxArgs) -> Self {
        Self(args)
    }
}
impl DepositIntoPerpMarketFeePoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositIntoPerpMarketFeePoolIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_into_perp_market_fee_pool_ix<
    K: Into<DepositIntoPerpMarketFeePoolKeys>,
    A: Into<DepositIntoPerpMarketFeePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositIntoPerpMarketFeePoolKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositIntoPerpMarketFeePoolIxArgs = args.into();
    let data: DepositIntoPerpMarketFeePoolIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_into_perp_market_fee_pool_invoke<
    'info,
    A: Into<DepositIntoPerpMarketFeePoolIxArgs>,
>(
    accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deposit_into_perp_market_fee_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_into_perp_market_fee_pool_invoke_signed<
    'info,
    A: Into<DepositIntoPerpMarketFeePoolIxArgs>,
>(
    accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_into_perp_market_fee_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deposit_into_perp_market_fee_pool_verify_account_keys(
    accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, '_>,
    keys: &DepositIntoPerpMarketFeePoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.admin.key, &keys.admin),
        (accounts.source_vault.key, &keys.source_vault),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.quote_spot_market.key, &keys.quote_spot_market),
        (accounts.spot_market_vault.key, &keys.spot_market_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_into_perp_market_fee_pool_verify_account_privileges<'me, 'info>(
    accounts: &DepositIntoPerpMarketFeePoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.perp_market,
        accounts.source_vault,
        accounts.quote_spot_market,
        accounts.spot_market_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REPEG_AMM_CURVE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RepegAmmCurveAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepegAmmCurveKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl From<&RepegAmmCurveAccounts<'_, '_>> for RepegAmmCurveKeys {
    fn from(accounts: &RepegAmmCurveAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&RepegAmmCurveKeys> for [AccountMeta; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: &RepegAmmCurveKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl From<[Pubkey; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN]> for RepegAmmCurveKeys {
    fn from(pubkeys: [Pubkey; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            perp_market: pubkeys[1],
            oracle: pubkeys[2],
            admin: pubkeys[3],
        }
    }
}
impl<'info> From<&RepegAmmCurveAccounts<'_, 'info>>
    for [AccountInfo<'info>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RepegAmmCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN]>
    for RepegAmmCurveAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            perp_market: &arr[1],
            oracle: &arr[2],
            admin: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RepegAmmCurveIxArgs {
    pub new_peg_candidate: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepegAmmCurveIxData(pub RepegAmmCurveIxArgs);
pub const REPEG_AMM_CURVE_IX_DISCM: [u8; 8] = [3, 36, 102, 89, 180, 128, 120, 213];
impl From<RepegAmmCurveIxArgs> for RepegAmmCurveIxData {
    fn from(args: RepegAmmCurveIxArgs) -> Self {
        Self(args)
    }
}
impl RepegAmmCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPEG_AMM_CURVE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REPEG_AMM_CURVE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RepegAmmCurveIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPEG_AMM_CURVE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repeg_amm_curve_ix<K: Into<RepegAmmCurveKeys>, A: Into<RepegAmmCurveIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RepegAmmCurveKeys = accounts.into();
    let metas: [AccountMeta; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RepegAmmCurveIxArgs = args.into();
    let data: RepegAmmCurveIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repeg_amm_curve_invoke<'info, A: Into<RepegAmmCurveIxArgs>>(
    accounts: &RepegAmmCurveAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = repeg_amm_curve_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn repeg_amm_curve_invoke_signed<'info, A: Into<RepegAmmCurveIxArgs>>(
    accounts: &RepegAmmCurveAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = repeg_amm_curve_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn repeg_amm_curve_verify_account_keys(
    accounts: &RepegAmmCurveAccounts<'_, '_>,
    keys: &RepegAmmCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn repeg_amm_curve_verify_account_privileges<'me, 'info>(
    accounts: &RepegAmmCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketAmmOracleTwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketAmmOracleTwapKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl From<&UpdatePerpMarketAmmOracleTwapAccounts<'_, '_>> for UpdatePerpMarketAmmOracleTwapKeys {
    fn from(accounts: &UpdatePerpMarketAmmOracleTwapAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&UpdatePerpMarketAmmOracleTwapKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketAmmOracleTwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketAmmOracleTwapKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            perp_market: pubkeys[1],
            oracle: pubkeys[2],
            admin: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdatePerpMarketAmmOracleTwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketAmmOracleTwapAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            perp_market: &arr[1],
            oracle: &arr[2],
            admin: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketAmmOracleTwapIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketAmmOracleTwapIxData(pub UpdatePerpMarketAmmOracleTwapIxArgs);
pub const UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM: [u8; 8] =
    [241, 74, 114, 123, 206, 153, 24, 202];
impl From<UpdatePerpMarketAmmOracleTwapIxArgs> for UpdatePerpMarketAmmOracleTwapIxData {
    fn from(args: UpdatePerpMarketAmmOracleTwapIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketAmmOracleTwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketAmmOracleTwapIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_amm_oracle_twap_ix<
    K: Into<UpdatePerpMarketAmmOracleTwapKeys>,
    A: Into<UpdatePerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketAmmOracleTwapKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketAmmOracleTwapIxArgs = args.into();
    let data: UpdatePerpMarketAmmOracleTwapIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_amm_oracle_twap_invoke<
    'info,
    A: Into<UpdatePerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_amm_oracle_twap_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_amm_oracle_twap_verify_account_keys(
    accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, '_>,
    keys: &UpdatePerpMarketAmmOracleTwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_amm_oracle_twap_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ResetPerpMarketAmmOracleTwapAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ResetPerpMarketAmmOracleTwapKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl From<&ResetPerpMarketAmmOracleTwapAccounts<'_, '_>> for ResetPerpMarketAmmOracleTwapKeys {
    fn from(accounts: &ResetPerpMarketAmmOracleTwapAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&ResetPerpMarketAmmOracleTwapKeys>
    for [AccountMeta; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(keys: &ResetPerpMarketAmmOracleTwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl From<[Pubkey; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]>
    for ResetPerpMarketAmmOracleTwapKeys
{
    fn from(pubkeys: [Pubkey; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            perp_market: pubkeys[1],
            oracle: pubkeys[2],
            admin: pubkeys[3],
        }
    }
}
impl<'info> From<&ResetPerpMarketAmmOracleTwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]>
    for ResetPerpMarketAmmOracleTwapAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            state: &arr[0],
            perp_market: &arr[1],
            oracle: &arr[2],
            admin: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResetPerpMarketAmmOracleTwapIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ResetPerpMarketAmmOracleTwapIxData(pub ResetPerpMarketAmmOracleTwapIxArgs);
pub const RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM: [u8; 8] =
    [127, 10, 55, 164, 123, 226, 47, 24];
impl From<ResetPerpMarketAmmOracleTwapIxArgs> for ResetPerpMarketAmmOracleTwapIxData {
    fn from(args: ResetPerpMarketAmmOracleTwapIxArgs) -> Self {
        Self(args)
    }
}
impl ResetPerpMarketAmmOracleTwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ResetPerpMarketAmmOracleTwapIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reset_perp_market_amm_oracle_twap_ix<
    K: Into<ResetPerpMarketAmmOracleTwapKeys>,
    A: Into<ResetPerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResetPerpMarketAmmOracleTwapKeys = accounts.into();
    let metas: [AccountMeta; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResetPerpMarketAmmOracleTwapIxArgs = args.into();
    let data: ResetPerpMarketAmmOracleTwapIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reset_perp_market_amm_oracle_twap_invoke<
    'info,
    A: Into<ResetPerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = reset_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn reset_perp_market_amm_oracle_twap_invoke_signed<
    'info,
    A: Into<ResetPerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = reset_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn reset_perp_market_amm_oracle_twap_verify_account_keys(
    accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, '_>,
    keys: &ResetPerpMarketAmmOracleTwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn reset_perp_market_amm_oracle_twap_verify_account_privileges<'me, 'info>(
    accounts: &ResetPerpMarketAmmOracleTwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_K_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateKAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateKKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}
impl From<&UpdateKAccounts<'_, '_>> for UpdateKKeys {
    fn from(accounts: &UpdateKAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateKKeys> for [AccountMeta; UPDATE_K_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateKKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_K_IX_ACCOUNTS_LEN]> for UpdateKKeys {
    fn from(pubkeys: [Pubkey; UPDATE_K_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
            oracle: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdateKAccounts<'_, 'info>> for [AccountInfo<'info>; UPDATE_K_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UpdateKAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_K_IX_ACCOUNTS_LEN]>
    for UpdateKAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_K_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
            oracle: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateKIxArgs {
    pub sqrt_k: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateKIxData(pub UpdateKIxArgs);
pub const UPDATE_K_IX_DISCM: [u8; 8] = [72, 98, 9, 139, 129, 229, 172, 56];
impl From<UpdateKIxArgs> for UpdateKIxData {
    fn from(args: UpdateKIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateKIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_K_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_K_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateKIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_K_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_k_ix<K: Into<UpdateKKeys>, A: Into<UpdateKIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateKKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_K_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateKIxArgs = args.into();
    let data: UpdateKIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_k_invoke<'info, A: Into<UpdateKIxArgs>>(
    accounts: &UpdateKAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_k_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_K_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_k_invoke_signed<'info, A: Into<UpdateKIxArgs>>(
    accounts: &UpdateKAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_k_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_K_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_k_verify_account_keys(
    accounts: &UpdateKAccounts<'_, '_>,
    keys: &UpdateKKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_k_verify_account_privileges<'me, 'info>(
    accounts: &UpdateKAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMarginRatioAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMarginRatioKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMarginRatioAccounts<'_, '_>> for UpdatePerpMarketMarginRatioKeys {
    fn from(accounts: &UpdatePerpMarketMarginRatioAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMarginRatioKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMarginRatioKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMarginRatioKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMarginRatioAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMarginRatioAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMarginRatioAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMarginRatioIxArgs {
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMarginRatioIxData(pub UpdatePerpMarketMarginRatioIxArgs);
pub const UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM: [u8; 8] =
    [130, 173, 107, 45, 119, 105, 26, 113];
impl From<UpdatePerpMarketMarginRatioIxArgs> for UpdatePerpMarketMarginRatioIxData {
    fn from(args: UpdatePerpMarketMarginRatioIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMarginRatioIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketMarginRatioIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_margin_ratio_ix<
    K: Into<UpdatePerpMarketMarginRatioKeys>,
    A: Into<UpdatePerpMarketMarginRatioIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMarginRatioKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMarginRatioIxArgs = args.into();
    let data: UpdatePerpMarketMarginRatioIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_margin_ratio_invoke<'info, A: Into<UpdatePerpMarketMarginRatioIxArgs>>(
    accounts: &UpdatePerpMarketMarginRatioAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_margin_ratio_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMarginRatioIxArgs>,
>(
    accounts: &UpdatePerpMarketMarginRatioAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_margin_ratio_verify_account_keys(
    accounts: &UpdatePerpMarketMarginRatioAccounts<'_, '_>,
    keys: &UpdatePerpMarketMarginRatioKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_margin_ratio_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMarginRatioAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxImbalancesAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxImbalancesKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMaxImbalancesAccounts<'_, '_>> for UpdatePerpMarketMaxImbalancesKeys {
    fn from(accounts: &UpdatePerpMarketMaxImbalancesAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxImbalancesKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxImbalancesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxImbalancesKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMaxImbalancesAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxImbalancesAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMaxImbalancesIxArgs {
    pub unrealized_max_imbalance: u64,
    pub max_revenue_withdraw_per_period: u64,
    pub quote_max_insurance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxImbalancesIxData(pub UpdatePerpMarketMaxImbalancesIxArgs);
pub const UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM: [u8; 8] = [15, 206, 73, 133, 60, 8, 86, 89];
impl From<UpdatePerpMarketMaxImbalancesIxArgs> for UpdatePerpMarketMaxImbalancesIxData {
    fn from(args: UpdatePerpMarketMaxImbalancesIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMaxImbalancesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketMaxImbalancesIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_max_imbalances_ix<
    K: Into<UpdatePerpMarketMaxImbalancesKeys>,
    A: Into<UpdatePerpMarketMaxImbalancesIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxImbalancesKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMaxImbalancesIxArgs = args.into();
    let data: UpdatePerpMarketMaxImbalancesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_imbalances_invoke<
    'info,
    A: Into<UpdatePerpMarketMaxImbalancesIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_imbalances_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_imbalances_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMaxImbalancesIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_imbalances_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_max_imbalances_verify_account_keys(
    accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, '_>,
    keys: &UpdatePerpMarketMaxImbalancesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_max_imbalances_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMaxImbalancesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketLiquidationFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketLiquidationFeeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketLiquidationFeeAccounts<'_, '_>> for UpdatePerpMarketLiquidationFeeKeys {
    fn from(accounts: &UpdatePerpMarketLiquidationFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketLiquidationFeeKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketLiquidationFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketLiquidationFeeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketLiquidationFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketLiquidationFeeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketLiquidationFeeIxArgs {
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketLiquidationFeeIxData(pub UpdatePerpMarketLiquidationFeeIxArgs);
pub const UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM: [u8; 8] = [90, 137, 9, 145, 41, 8, 148, 117];
impl From<UpdatePerpMarketLiquidationFeeIxArgs> for UpdatePerpMarketLiquidationFeeIxData {
    fn from(args: UpdatePerpMarketLiquidationFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketLiquidationFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketLiquidationFeeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_liquidation_fee_ix<
    K: Into<UpdatePerpMarketLiquidationFeeKeys>,
    A: Into<UpdatePerpMarketLiquidationFeeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketLiquidationFeeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketLiquidationFeeIxArgs = args.into();
    let data: UpdatePerpMarketLiquidationFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_liquidation_fee_invoke<
    'info,
    A: Into<UpdatePerpMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_liquidation_fee_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_liquidation_fee_verify_account_keys(
    accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, '_>,
    keys: &UpdatePerpMarketLiquidationFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_liquidation_fee_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketLiquidationFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateInsuranceFundUnstakingPeriodAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateInsuranceFundUnstakingPeriodKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateInsuranceFundUnstakingPeriodAccounts<'_, '_>>
    for UpdateInsuranceFundUnstakingPeriodKeys
{
    fn from(accounts: &UpdateInsuranceFundUnstakingPeriodAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateInsuranceFundUnstakingPeriodKeys>
    for [AccountMeta; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateInsuranceFundUnstakingPeriodKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]>
    for UpdateInsuranceFundUnstakingPeriodKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]>
    for UpdateInsuranceFundUnstakingPeriodAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateInsuranceFundUnstakingPeriodIxArgs {
    pub insurance_fund_unstaking_period: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateInsuranceFundUnstakingPeriodIxData(pub UpdateInsuranceFundUnstakingPeriodIxArgs);
pub const UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM: [u8; 8] =
    [44, 69, 43, 226, 204, 223, 202, 52];
impl From<UpdateInsuranceFundUnstakingPeriodIxArgs> for UpdateInsuranceFundUnstakingPeriodIxData {
    fn from(args: UpdateInsuranceFundUnstakingPeriodIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateInsuranceFundUnstakingPeriodIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateInsuranceFundUnstakingPeriodIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_insurance_fund_unstaking_period_ix<
    K: Into<UpdateInsuranceFundUnstakingPeriodKeys>,
    A: Into<UpdateInsuranceFundUnstakingPeriodIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateInsuranceFundUnstakingPeriodKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateInsuranceFundUnstakingPeriodIxArgs = args.into();
    let data: UpdateInsuranceFundUnstakingPeriodIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_insurance_fund_unstaking_period_invoke<
    'info,
    A: Into<UpdateInsuranceFundUnstakingPeriodIxArgs>,
>(
    accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_insurance_fund_unstaking_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_insurance_fund_unstaking_period_invoke_signed<
    'info,
    A: Into<UpdateInsuranceFundUnstakingPeriodIxArgs>,
>(
    accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_insurance_fund_unstaking_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_insurance_fund_unstaking_period_verify_account_keys(
    accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, '_>,
    keys: &UpdateInsuranceFundUnstakingPeriodKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_insurance_fund_unstaking_period_verify_account_privileges<'me, 'info>(
    accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketLiquidationFeeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketLiquidationFeeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketLiquidationFeeAccounts<'_, '_>> for UpdateSpotMarketLiquidationFeeKeys {
    fn from(accounts: &UpdateSpotMarketLiquidationFeeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketLiquidationFeeKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketLiquidationFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketLiquidationFeeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketLiquidationFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketLiquidationFeeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketLiquidationFeeIxArgs {
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketLiquidationFeeIxData(pub UpdateSpotMarketLiquidationFeeIxArgs);
pub const UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM: [u8; 8] =
    [11, 13, 255, 53, 56, 136, 104, 177];
impl From<UpdateSpotMarketLiquidationFeeIxArgs> for UpdateSpotMarketLiquidationFeeIxData {
    fn from(args: UpdateSpotMarketLiquidationFeeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketLiquidationFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketLiquidationFeeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_liquidation_fee_ix<
    K: Into<UpdateSpotMarketLiquidationFeeKeys>,
    A: Into<UpdateSpotMarketLiquidationFeeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketLiquidationFeeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketLiquidationFeeIxArgs = args.into();
    let data: UpdateSpotMarketLiquidationFeeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_liquidation_fee_invoke<
    'info,
    A: Into<UpdateSpotMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_liquidation_fee_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_liquidation_fee_verify_account_keys(
    accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, '_>,
    keys: &UpdateSpotMarketLiquidationFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_liquidation_fee_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketLiquidationFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateWithdrawGuardThresholdAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateWithdrawGuardThresholdKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateWithdrawGuardThresholdAccounts<'_, '_>> for UpdateWithdrawGuardThresholdKeys {
    fn from(accounts: &UpdateWithdrawGuardThresholdAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateWithdrawGuardThresholdKeys>
    for [AccountMeta; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateWithdrawGuardThresholdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]>
    for UpdateWithdrawGuardThresholdKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateWithdrawGuardThresholdAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateWithdrawGuardThresholdAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]>
    for UpdateWithdrawGuardThresholdAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateWithdrawGuardThresholdIxArgs {
    pub withdraw_guard_threshold: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateWithdrawGuardThresholdIxData(pub UpdateWithdrawGuardThresholdIxArgs);
pub const UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM: [u8; 8] = [56, 18, 39, 61, 155, 211, 44, 133];
impl From<UpdateWithdrawGuardThresholdIxArgs> for UpdateWithdrawGuardThresholdIxData {
    fn from(args: UpdateWithdrawGuardThresholdIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateWithdrawGuardThresholdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateWithdrawGuardThresholdIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_withdraw_guard_threshold_ix<
    K: Into<UpdateWithdrawGuardThresholdKeys>,
    A: Into<UpdateWithdrawGuardThresholdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateWithdrawGuardThresholdKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateWithdrawGuardThresholdIxArgs = args.into();
    let data: UpdateWithdrawGuardThresholdIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_withdraw_guard_threshold_invoke<
    'info,
    A: Into<UpdateWithdrawGuardThresholdIxArgs>,
>(
    accounts: &UpdateWithdrawGuardThresholdAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_withdraw_guard_threshold_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_withdraw_guard_threshold_invoke_signed<
    'info,
    A: Into<UpdateWithdrawGuardThresholdIxArgs>,
>(
    accounts: &UpdateWithdrawGuardThresholdAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_withdraw_guard_threshold_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_withdraw_guard_threshold_verify_account_keys(
    accounts: &UpdateWithdrawGuardThresholdAccounts<'_, '_>,
    keys: &UpdateWithdrawGuardThresholdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_withdraw_guard_threshold_verify_account_privileges<'me, 'info>(
    accounts: &UpdateWithdrawGuardThresholdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketIfFactorAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketIfFactorKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketIfFactorAccounts<'_, '_>> for UpdateSpotMarketIfFactorKeys {
    fn from(accounts: &UpdateSpotMarketIfFactorAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketIfFactorKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketIfFactorKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]> for UpdateSpotMarketIfFactorKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketIfFactorAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketIfFactorAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketIfFactorAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketIfFactorIxArgs {
    pub spot_market_index: u16,
    pub user_if_factor: u32,
    pub total_if_factor: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketIfFactorIxData(pub UpdateSpotMarketIfFactorIxArgs);
pub const UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM: [u8; 8] = [147, 30, 224, 34, 18, 230, 105, 4];
impl From<UpdateSpotMarketIfFactorIxArgs> for UpdateSpotMarketIfFactorIxData {
    fn from(args: UpdateSpotMarketIfFactorIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketIfFactorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketIfFactorIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_if_factor_ix<
    K: Into<UpdateSpotMarketIfFactorKeys>,
    A: Into<UpdateSpotMarketIfFactorIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketIfFactorKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketIfFactorIxArgs = args.into();
    let data: UpdateSpotMarketIfFactorIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_if_factor_invoke<'info, A: Into<UpdateSpotMarketIfFactorIxArgs>>(
    accounts: &UpdateSpotMarketIfFactorAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_if_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_if_factor_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketIfFactorIxArgs>,
>(
    accounts: &UpdateSpotMarketIfFactorAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_if_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_if_factor_verify_account_keys(
    accounts: &UpdateSpotMarketIfFactorAccounts<'_, '_>,
    keys: &UpdateSpotMarketIfFactorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_if_factor_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketIfFactorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketRevenueSettlePeriodAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketRevenueSettlePeriodKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketRevenueSettlePeriodAccounts<'_, '_>>
    for UpdateSpotMarketRevenueSettlePeriodKeys
{
    fn from(accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketRevenueSettlePeriodKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketRevenueSettlePeriodKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketRevenueSettlePeriodKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketRevenueSettlePeriodAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketRevenueSettlePeriodIxArgs {
    pub revenue_settle_period: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketRevenueSettlePeriodIxData(pub UpdateSpotMarketRevenueSettlePeriodIxArgs);
pub const UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM: [u8; 8] =
    [81, 92, 126, 41, 250, 225, 156, 219];
impl From<UpdateSpotMarketRevenueSettlePeriodIxArgs> for UpdateSpotMarketRevenueSettlePeriodIxData {
    fn from(args: UpdateSpotMarketRevenueSettlePeriodIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketRevenueSettlePeriodIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdateSpotMarketRevenueSettlePeriodIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_revenue_settle_period_ix<
    K: Into<UpdateSpotMarketRevenueSettlePeriodKeys>,
    A: Into<UpdateSpotMarketRevenueSettlePeriodIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketRevenueSettlePeriodKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketRevenueSettlePeriodIxArgs = args.into();
    let data: UpdateSpotMarketRevenueSettlePeriodIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_revenue_settle_period_invoke<
    'info,
    A: Into<UpdateSpotMarketRevenueSettlePeriodIxArgs>,
>(
    accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_revenue_settle_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_revenue_settle_period_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketRevenueSettlePeriodIxArgs>,
>(
    accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_revenue_settle_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_revenue_settle_period_verify_account_keys(
    accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, '_>,
    keys: &UpdateSpotMarketRevenueSettlePeriodKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_revenue_settle_period_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStatusAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketStatusKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketStatusAccounts<'_, '_>> for UpdateSpotMarketStatusKeys {
    fn from(accounts: &UpdateSpotMarketStatusAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketStatusKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]> for UpdateSpotMarketStatusKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketStatusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketStatusIxArgs {
    pub status: MarketStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketStatusIxData(pub UpdateSpotMarketStatusIxArgs);
pub const UPDATE_SPOT_MARKET_STATUS_IX_DISCM: [u8; 8] = [78, 94, 16, 188, 193, 110, 231, 31];
impl From<UpdateSpotMarketStatusIxArgs> for UpdateSpotMarketStatusIxData {
    fn from(args: UpdateSpotMarketStatusIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketStatusIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_STATUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_status_ix<
    K: Into<UpdateSpotMarketStatusKeys>,
    A: Into<UpdateSpotMarketStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketStatusIxArgs = args.into();
    let data: UpdateSpotMarketStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_status_invoke<'info, A: Into<UpdateSpotMarketStatusIxArgs>>(
    accounts: &UpdateSpotMarketStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_status_invoke_signed<'info, A: Into<UpdateSpotMarketStatusIxArgs>>(
    accounts: &UpdateSpotMarketStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_status_verify_account_keys(
    accounts: &UpdateSpotMarketStatusAccounts<'_, '_>,
    keys: &UpdateSpotMarketStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_status_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketAssetTierAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketAssetTierKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketAssetTierAccounts<'_, '_>> for UpdateSpotMarketAssetTierKeys {
    fn from(accounts: &UpdateSpotMarketAssetTierAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketAssetTierKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketAssetTierKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketAssetTierKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketAssetTierAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketAssetTierAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketAssetTierAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketAssetTierIxArgs {
    pub asset_tier: AssetTier,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketAssetTierIxData(pub UpdateSpotMarketAssetTierIxArgs);
pub const UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM: [u8; 8] = [253, 209, 231, 14, 242, 208, 243, 130];
impl From<UpdateSpotMarketAssetTierIxArgs> for UpdateSpotMarketAssetTierIxData {
    fn from(args: UpdateSpotMarketAssetTierIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketAssetTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketAssetTierIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_asset_tier_ix<
    K: Into<UpdateSpotMarketAssetTierKeys>,
    A: Into<UpdateSpotMarketAssetTierIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketAssetTierKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketAssetTierIxArgs = args.into();
    let data: UpdateSpotMarketAssetTierIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_asset_tier_invoke<'info, A: Into<UpdateSpotMarketAssetTierIxArgs>>(
    accounts: &UpdateSpotMarketAssetTierAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_asset_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_asset_tier_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketAssetTierIxArgs>,
>(
    accounts: &UpdateSpotMarketAssetTierAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_asset_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_asset_tier_verify_account_keys(
    accounts: &UpdateSpotMarketAssetTierAccounts<'_, '_>,
    keys: &UpdateSpotMarketAssetTierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_asset_tier_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketAssetTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMarginWeightsAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMarginWeightsKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketMarginWeightsAccounts<'_, '_>> for UpdateSpotMarketMarginWeightsKeys {
    fn from(accounts: &UpdateSpotMarketMarginWeightsAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketMarginWeightsKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketMarginWeightsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketMarginWeightsKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketMarginWeightsAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketMarginWeightsAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketMarginWeightsIxArgs {
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMarginWeightsIxData(pub UpdateSpotMarketMarginWeightsIxArgs);
pub const UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM: [u8; 8] = [109, 33, 87, 195, 255, 36, 6, 81];
impl From<UpdateSpotMarketMarginWeightsIxArgs> for UpdateSpotMarketMarginWeightsIxData {
    fn from(args: UpdateSpotMarketMarginWeightsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketMarginWeightsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketMarginWeightsIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_margin_weights_ix<
    K: Into<UpdateSpotMarketMarginWeightsKeys>,
    A: Into<UpdateSpotMarketMarginWeightsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketMarginWeightsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketMarginWeightsIxArgs = args.into();
    let data: UpdateSpotMarketMarginWeightsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_margin_weights_invoke<
    'info,
    A: Into<UpdateSpotMarketMarginWeightsIxArgs>,
>(
    accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_margin_weights_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_margin_weights_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketMarginWeightsIxArgs>,
>(
    accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_margin_weights_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_margin_weights_verify_account_keys(
    accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, '_>,
    keys: &UpdateSpotMarketMarginWeightsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_margin_weights_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketMarginWeightsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketBorrowRateAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketBorrowRateKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketBorrowRateAccounts<'_, '_>> for UpdateSpotMarketBorrowRateKeys {
    fn from(accounts: &UpdateSpotMarketBorrowRateAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketBorrowRateKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketBorrowRateKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketBorrowRateKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketBorrowRateAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketBorrowRateAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketBorrowRateAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketBorrowRateIxArgs {
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketBorrowRateIxData(pub UpdateSpotMarketBorrowRateIxArgs);
pub const UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM: [u8; 8] = [71, 239, 236, 153, 210, 62, 254, 76];
impl From<UpdateSpotMarketBorrowRateIxArgs> for UpdateSpotMarketBorrowRateIxData {
    fn from(args: UpdateSpotMarketBorrowRateIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketBorrowRateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketBorrowRateIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_borrow_rate_ix<
    K: Into<UpdateSpotMarketBorrowRateKeys>,
    A: Into<UpdateSpotMarketBorrowRateIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketBorrowRateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketBorrowRateIxArgs = args.into();
    let data: UpdateSpotMarketBorrowRateIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_borrow_rate_invoke<'info, A: Into<UpdateSpotMarketBorrowRateIxArgs>>(
    accounts: &UpdateSpotMarketBorrowRateAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_borrow_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_borrow_rate_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketBorrowRateIxArgs>,
>(
    accounts: &UpdateSpotMarketBorrowRateAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_borrow_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_borrow_rate_verify_account_keys(
    accounts: &UpdateSpotMarketBorrowRateAccounts<'_, '_>,
    keys: &UpdateSpotMarketBorrowRateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_borrow_rate_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketBorrowRateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMaxTokenDepositsAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMaxTokenDepositsKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketMaxTokenDepositsAccounts<'_, '_>>
    for UpdateSpotMarketMaxTokenDepositsKeys
{
    fn from(accounts: &UpdateSpotMarketMaxTokenDepositsAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketMaxTokenDepositsKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketMaxTokenDepositsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketMaxTokenDepositsKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketMaxTokenDepositsAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketMaxTokenDepositsIxArgs {
    pub max_token_deposits: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMaxTokenDepositsIxData(pub UpdateSpotMarketMaxTokenDepositsIxArgs);
pub const UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM: [u8; 8] =
    [56, 191, 79, 18, 26, 121, 80, 208];
impl From<UpdateSpotMarketMaxTokenDepositsIxArgs> for UpdateSpotMarketMaxTokenDepositsIxData {
    fn from(args: UpdateSpotMarketMaxTokenDepositsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketMaxTokenDepositsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketMaxTokenDepositsIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_max_token_deposits_ix<
    K: Into<UpdateSpotMarketMaxTokenDepositsKeys>,
    A: Into<UpdateSpotMarketMaxTokenDepositsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketMaxTokenDepositsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketMaxTokenDepositsIxArgs = args.into();
    let data: UpdateSpotMarketMaxTokenDepositsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_max_token_deposits_invoke<
    'info,
    A: Into<UpdateSpotMarketMaxTokenDepositsIxArgs>,
>(
    accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_max_token_deposits_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_max_token_deposits_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketMaxTokenDepositsIxArgs>,
>(
    accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_max_token_deposits_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_max_token_deposits_verify_account_keys(
    accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, '_>,
    keys: &UpdateSpotMarketMaxTokenDepositsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_max_token_deposits_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOracleAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketOracleKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
}
impl From<&UpdateSpotMarketOracleAccounts<'_, '_>> for UpdateSpotMarketOracleKeys {
    fn from(accounts: &UpdateSpotMarketOracleAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateSpotMarketOracleKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketOracleKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]> for UpdateSpotMarketOracleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
            oracle: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdateSpotMarketOracleAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketOracleAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
            oracle: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketOracleIxArgs {
    pub oracle: Pubkey,
    pub oracle_source: OracleSource,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketOracleIxData(pub UpdateSpotMarketOracleIxArgs);
pub const UPDATE_SPOT_MARKET_ORACLE_IX_DISCM: [u8; 8] = [114, 184, 102, 37, 246, 186, 180, 99];
impl From<UpdateSpotMarketOracleIxArgs> for UpdateSpotMarketOracleIxData {
    fn from(args: UpdateSpotMarketOracleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_ORACLE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_ORACLE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketOracleIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_ORACLE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_oracle_ix<
    K: Into<UpdateSpotMarketOracleKeys>,
    A: Into<UpdateSpotMarketOracleIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketOracleKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketOracleIxArgs = args.into();
    let data: UpdateSpotMarketOracleIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_oracle_invoke<'info, A: Into<UpdateSpotMarketOracleIxArgs>>(
    accounts: &UpdateSpotMarketOracleAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_oracle_invoke_signed<'info, A: Into<UpdateSpotMarketOracleIxArgs>>(
    accounts: &UpdateSpotMarketOracleAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_oracle_verify_account_keys(
    accounts: &UpdateSpotMarketOracleAccounts<'_, '_>,
    keys: &UpdateSpotMarketOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (accounts.oracle.key, &keys.oracle),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_oracle_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStepSizeAndTickSizeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketStepSizeAndTickSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, '_>>
    for UpdateSpotMarketStepSizeAndTickSizeKeys
{
    fn from(accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketStepSizeAndTickSizeKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketStepSizeAndTickSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketStepSizeAndTickSizeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketStepSizeAndTickSizeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketStepSizeAndTickSizeIxArgs {
    pub step_size: u64,
    pub tick_size: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketStepSizeAndTickSizeIxData(pub UpdateSpotMarketStepSizeAndTickSizeIxArgs);
pub const UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM: [u8; 8] =
    [238, 153, 137, 80, 206, 59, 250, 61];
impl From<UpdateSpotMarketStepSizeAndTickSizeIxArgs> for UpdateSpotMarketStepSizeAndTickSizeIxData {
    fn from(args: UpdateSpotMarketStepSizeAndTickSizeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketStepSizeAndTickSizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdateSpotMarketStepSizeAndTickSizeIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_step_size_and_tick_size_ix<
    K: Into<UpdateSpotMarketStepSizeAndTickSizeKeys>,
    A: Into<UpdateSpotMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketStepSizeAndTickSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketStepSizeAndTickSizeIxArgs = args.into();
    let data: UpdateSpotMarketStepSizeAndTickSizeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_step_size_and_tick_size_invoke<
    'info,
    A: Into<UpdateSpotMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_step_size_and_tick_size_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_step_size_and_tick_size_verify_account_keys(
    accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, '_>,
    keys: &UpdateSpotMarketStepSizeAndTickSizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_step_size_and_tick_size_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMinOrderSizeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMinOrderSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketMinOrderSizeAccounts<'_, '_>> for UpdateSpotMarketMinOrderSizeKeys {
    fn from(accounts: &UpdateSpotMarketMinOrderSizeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketMinOrderSizeKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketMinOrderSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketMinOrderSizeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketMinOrderSizeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketMinOrderSizeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketMinOrderSizeIxArgs {
    pub order_size: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketMinOrderSizeIxData(pub UpdateSpotMarketMinOrderSizeIxArgs);
pub const UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM: [u8; 8] = [93, 128, 11, 119, 26, 20, 181, 50];
impl From<UpdateSpotMarketMinOrderSizeIxArgs> for UpdateSpotMarketMinOrderSizeIxData {
    fn from(args: UpdateSpotMarketMinOrderSizeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketMinOrderSizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketMinOrderSizeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_min_order_size_ix<
    K: Into<UpdateSpotMarketMinOrderSizeKeys>,
    A: Into<UpdateSpotMarketMinOrderSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketMinOrderSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketMinOrderSizeIxArgs = args.into();
    let data: UpdateSpotMarketMinOrderSizeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_min_order_size_invoke<
    'info,
    A: Into<UpdateSpotMarketMinOrderSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_min_order_size_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketMinOrderSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_min_order_size_verify_account_keys(
    accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, '_>,
    keys: &UpdateSpotMarketMinOrderSizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_min_order_size_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketMinOrderSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOrdersEnabledAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketOrdersEnabledKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketOrdersEnabledAccounts<'_, '_>> for UpdateSpotMarketOrdersEnabledKeys {
    fn from(accounts: &UpdateSpotMarketOrdersEnabledAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketOrdersEnabledKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketOrdersEnabledKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketOrdersEnabledKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketOrdersEnabledAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketOrdersEnabledAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketOrdersEnabledIxArgs {
    pub orders_enabled: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketOrdersEnabledIxData(pub UpdateSpotMarketOrdersEnabledIxArgs);
pub const UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM: [u8; 8] =
    [190, 79, 206, 15, 26, 229, 229, 43];
impl From<UpdateSpotMarketOrdersEnabledIxArgs> for UpdateSpotMarketOrdersEnabledIxData {
    fn from(args: UpdateSpotMarketOrdersEnabledIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketOrdersEnabledIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketOrdersEnabledIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_orders_enabled_ix<
    K: Into<UpdateSpotMarketOrdersEnabledKeys>,
    A: Into<UpdateSpotMarketOrdersEnabledIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketOrdersEnabledKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketOrdersEnabledIxArgs = args.into();
    let data: UpdateSpotMarketOrdersEnabledIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_orders_enabled_invoke<
    'info,
    A: Into<UpdateSpotMarketOrdersEnabledIxArgs>,
>(
    accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_orders_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_orders_enabled_invoke_signed<
    'info,
    A: Into<UpdateSpotMarketOrdersEnabledIxArgs>,
>(
    accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_orders_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_orders_enabled_verify_account_keys(
    accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, '_>,
    keys: &UpdateSpotMarketOrdersEnabledKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_orders_enabled_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketOrdersEnabledAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketNameAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketNameKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl From<&UpdateSpotMarketNameAccounts<'_, '_>> for UpdateSpotMarketNameKeys {
    fn from(accounts: &UpdateSpotMarketNameAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketNameKeys> for [AccountMeta; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateSpotMarketNameKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN]> for UpdateSpotMarketNameKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateSpotMarketNameAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketNameAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN]>
    for UpdateSpotMarketNameAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotMarketNameIxArgs {
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotMarketNameIxData(pub UpdateSpotMarketNameIxArgs);
pub const UPDATE_SPOT_MARKET_NAME_IX_DISCM: [u8; 8] = [17, 208, 1, 1, 162, 211, 188, 224];
impl From<UpdateSpotMarketNameIxArgs> for UpdateSpotMarketNameIxData {
    fn from(args: UpdateSpotMarketNameIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotMarketNameIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_MARKET_NAME_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_MARKET_NAME_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotMarketNameIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_NAME_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_market_name_ix<
    K: Into<UpdateSpotMarketNameKeys>,
    A: Into<UpdateSpotMarketNameIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketNameKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketNameIxArgs = args.into();
    let data: UpdateSpotMarketNameIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_name_invoke<'info, A: Into<UpdateSpotMarketNameIxArgs>>(
    accounts: &UpdateSpotMarketNameAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_name_invoke_signed<'info, A: Into<UpdateSpotMarketNameIxArgs>>(
    accounts: &UpdateSpotMarketNameAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_market_name_verify_account_keys(
    accounts: &UpdateSpotMarketNameAccounts<'_, '_>,
    keys: &UpdateSpotMarketNameKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_market_name_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotMarketNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.spot_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStatusAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketStatusKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketStatusAccounts<'_, '_>> for UpdatePerpMarketStatusKeys {
    fn from(accounts: &UpdatePerpMarketStatusAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketStatusKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]> for UpdatePerpMarketStatusKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketStatusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketStatusIxArgs {
    pub status: MarketStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketStatusIxData(pub UpdatePerpMarketStatusIxArgs);
pub const UPDATE_PERP_MARKET_STATUS_IX_DISCM: [u8; 8] = [71, 201, 175, 122, 255, 207, 196, 207];
impl From<UpdatePerpMarketStatusIxArgs> for UpdatePerpMarketStatusIxData {
    fn from(args: UpdatePerpMarketStatusIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketStatusIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_STATUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_status_ix<
    K: Into<UpdatePerpMarketStatusKeys>,
    A: Into<UpdatePerpMarketStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketStatusIxArgs = args.into();
    let data: UpdatePerpMarketStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_status_invoke<'info, A: Into<UpdatePerpMarketStatusIxArgs>>(
    accounts: &UpdatePerpMarketStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_status_invoke_signed<'info, A: Into<UpdatePerpMarketStatusIxArgs>>(
    accounts: &UpdatePerpMarketStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_status_verify_account_keys(
    accounts: &UpdatePerpMarketStatusAccounts<'_, '_>,
    keys: &UpdatePerpMarketStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_status_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketContractTierAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketContractTierKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketContractTierAccounts<'_, '_>> for UpdatePerpMarketContractTierKeys {
    fn from(accounts: &UpdatePerpMarketContractTierAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketContractTierKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketContractTierKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketContractTierKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketContractTierAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketContractTierAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketContractTierAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketContractTierIxArgs {
    pub contract_tier: ContractTier,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketContractTierIxData(pub UpdatePerpMarketContractTierIxArgs);
pub const UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM: [u8; 8] =
    [236, 128, 15, 95, 203, 214, 68, 117];
impl From<UpdatePerpMarketContractTierIxArgs> for UpdatePerpMarketContractTierIxData {
    fn from(args: UpdatePerpMarketContractTierIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketContractTierIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketContractTierIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_contract_tier_ix<
    K: Into<UpdatePerpMarketContractTierKeys>,
    A: Into<UpdatePerpMarketContractTierIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketContractTierKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketContractTierIxArgs = args.into();
    let data: UpdatePerpMarketContractTierIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_contract_tier_invoke<
    'info,
    A: Into<UpdatePerpMarketContractTierIxArgs>,
>(
    accounts: &UpdatePerpMarketContractTierAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_contract_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_contract_tier_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketContractTierIxArgs>,
>(
    accounts: &UpdatePerpMarketContractTierAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_contract_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_contract_tier_verify_account_keys(
    accounts: &UpdatePerpMarketContractTierAccounts<'_, '_>,
    keys: &UpdatePerpMarketContractTierKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_contract_tier_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketContractTierAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketImfFactorAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketImfFactorKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketImfFactorAccounts<'_, '_>> for UpdatePerpMarketImfFactorKeys {
    fn from(accounts: &UpdatePerpMarketImfFactorAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketImfFactorKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketImfFactorKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketImfFactorKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketImfFactorAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketImfFactorAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketImfFactorAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketImfFactorIxArgs {
    pub imf_factor: u32,
    pub unrealized_pnl_imf_factor: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketImfFactorIxData(pub UpdatePerpMarketImfFactorIxArgs);
pub const UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM: [u8; 8] = [207, 194, 56, 132, 35, 67, 71, 244];
impl From<UpdatePerpMarketImfFactorIxArgs> for UpdatePerpMarketImfFactorIxData {
    fn from(args: UpdatePerpMarketImfFactorIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketImfFactorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketImfFactorIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_imf_factor_ix<
    K: Into<UpdatePerpMarketImfFactorKeys>,
    A: Into<UpdatePerpMarketImfFactorIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketImfFactorKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketImfFactorIxArgs = args.into();
    let data: UpdatePerpMarketImfFactorIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_imf_factor_invoke<'info, A: Into<UpdatePerpMarketImfFactorIxArgs>>(
    accounts: &UpdatePerpMarketImfFactorAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_imf_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_imf_factor_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketImfFactorIxArgs>,
>(
    accounts: &UpdatePerpMarketImfFactorAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_imf_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_imf_factor_verify_account_keys(
    accounts: &UpdatePerpMarketImfFactorAccounts<'_, '_>,
    keys: &UpdatePerpMarketImfFactorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_imf_factor_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketImfFactorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketUnrealizedAssetWeightAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketUnrealizedAssetWeightKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, '_>>
    for UpdatePerpMarketUnrealizedAssetWeightKeys
{
    fn from(accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketUnrealizedAssetWeightKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketUnrealizedAssetWeightKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketUnrealizedAssetWeightKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketUnrealizedAssetWeightAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketUnrealizedAssetWeightIxArgs {
    pub unrealized_initial_asset_weight: u32,
    pub unrealized_maintenance_asset_weight: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketUnrealizedAssetWeightIxData(
    pub UpdatePerpMarketUnrealizedAssetWeightIxArgs,
);
pub const UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM: [u8; 8] =
    [135, 132, 205, 165, 109, 150, 166, 106];
impl From<UpdatePerpMarketUnrealizedAssetWeightIxArgs>
    for UpdatePerpMarketUnrealizedAssetWeightIxData
{
    fn from(args: UpdatePerpMarketUnrealizedAssetWeightIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketUnrealizedAssetWeightIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdatePerpMarketUnrealizedAssetWeightIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_unrealized_asset_weight_ix<
    K: Into<UpdatePerpMarketUnrealizedAssetWeightKeys>,
    A: Into<UpdatePerpMarketUnrealizedAssetWeightIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketUnrealizedAssetWeightKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketUnrealizedAssetWeightIxArgs = args.into();
    let data: UpdatePerpMarketUnrealizedAssetWeightIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_unrealized_asset_weight_invoke<
    'info,
    A: Into<UpdatePerpMarketUnrealizedAssetWeightIxArgs>,
>(
    accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_unrealized_asset_weight_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_unrealized_asset_weight_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketUnrealizedAssetWeightIxArgs>,
>(
    accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_unrealized_asset_weight_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_unrealized_asset_weight_verify_account_keys(
    accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, '_>,
    keys: &UpdatePerpMarketUnrealizedAssetWeightKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_unrealized_asset_weight_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketConcentrationCoefAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketConcentrationCoefKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketConcentrationCoefAccounts<'_, '_>>
    for UpdatePerpMarketConcentrationCoefKeys
{
    fn from(accounts: &UpdatePerpMarketConcentrationCoefAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketConcentrationCoefKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketConcentrationCoefKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketConcentrationCoefKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketConcentrationCoefAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketConcentrationCoefAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketConcentrationCoefIxArgs {
    pub concentration_scale: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketConcentrationCoefIxData(pub UpdatePerpMarketConcentrationCoefIxArgs);
pub const UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM: [u8; 8] =
    [24, 78, 232, 126, 169, 176, 230, 16];
impl From<UpdatePerpMarketConcentrationCoefIxArgs> for UpdatePerpMarketConcentrationCoefIxData {
    fn from(args: UpdatePerpMarketConcentrationCoefIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketConcentrationCoefIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketConcentrationCoefIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_concentration_coef_ix<
    K: Into<UpdatePerpMarketConcentrationCoefKeys>,
    A: Into<UpdatePerpMarketConcentrationCoefIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketConcentrationCoefKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketConcentrationCoefIxArgs = args.into();
    let data: UpdatePerpMarketConcentrationCoefIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_concentration_coef_invoke<
    'info,
    A: Into<UpdatePerpMarketConcentrationCoefIxArgs>,
>(
    accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_concentration_coef_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_concentration_coef_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketConcentrationCoefIxArgs>,
>(
    accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_concentration_coef_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_concentration_coef_verify_account_keys(
    accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, '_>,
    keys: &UpdatePerpMarketConcentrationCoefKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_concentration_coef_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketConcentrationCoefAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketCurveUpdateIntensityAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketCurveUpdateIntensityKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketCurveUpdateIntensityAccounts<'_, '_>>
    for UpdatePerpMarketCurveUpdateIntensityKeys
{
    fn from(accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketCurveUpdateIntensityKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketCurveUpdateIntensityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketCurveUpdateIntensityKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketCurveUpdateIntensityAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketCurveUpdateIntensityIxArgs {
    pub curve_update_intensity: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketCurveUpdateIntensityIxData(
    pub UpdatePerpMarketCurveUpdateIntensityIxArgs,
);
pub const UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM: [u8; 8] =
    [50, 131, 6, 156, 226, 231, 189, 72];
impl From<UpdatePerpMarketCurveUpdateIntensityIxArgs>
    for UpdatePerpMarketCurveUpdateIntensityIxData
{
    fn from(args: UpdatePerpMarketCurveUpdateIntensityIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketCurveUpdateIntensityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdatePerpMarketCurveUpdateIntensityIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_curve_update_intensity_ix<
    K: Into<UpdatePerpMarketCurveUpdateIntensityKeys>,
    A: Into<UpdatePerpMarketCurveUpdateIntensityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketCurveUpdateIntensityKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketCurveUpdateIntensityIxArgs = args.into();
    let data: UpdatePerpMarketCurveUpdateIntensityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_curve_update_intensity_invoke<
    'info,
    A: Into<UpdatePerpMarketCurveUpdateIntensityIxArgs>,
>(
    accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_curve_update_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_curve_update_intensity_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketCurveUpdateIntensityIxArgs>,
>(
    accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_curve_update_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_curve_update_intensity_verify_account_keys(
    accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, '_>,
    keys: &UpdatePerpMarketCurveUpdateIntensityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_curve_update_intensity_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, '_>>
    for UpdatePerpMarketTargetBaseAssetAmountPerLpKeys
{
    fn from(accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketTargetBaseAssetAmountPerLpKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketTargetBaseAssetAmountPerLpKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketTargetBaseAssetAmountPerLpKeys
{
    fn from(
        pubkeys: [Pubkey; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<
        &'me [AccountInfo<'info>;
                 UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN],
    > for UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>;
                 UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs {
    pub target_base_asset_amount_per_lp: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpIxData(
    pub UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs,
);
pub const UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM: [u8; 8] =
    [62, 87, 68, 115, 29, 150, 150, 165];
impl From<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>
    for UpdatePerpMarketTargetBaseAssetAmountPerLpIxData
{
    fn from(args: UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketTargetBaseAssetAmountPerLpIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_target_base_asset_amount_per_lp_ix<
    K: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpKeys>,
    A: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketTargetBaseAssetAmountPerLpKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs = args.into();
    let data: UpdatePerpMarketTargetBaseAssetAmountPerLpIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_target_base_asset_amount_per_lp_invoke<
    'info,
    A: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>,
>(
    accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_target_base_asset_amount_per_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_target_base_asset_amount_per_lp_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>,
>(
    accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_target_base_asset_amount_per_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_target_base_asset_amount_per_lp_verify_account_keys(
    accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, '_>,
    keys: &UpdatePerpMarketTargetBaseAssetAmountPerLpKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_target_base_asset_amount_per_lp_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLpCooldownTimeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLpCooldownTimeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateLpCooldownTimeAccounts<'_, '_>> for UpdateLpCooldownTimeKeys {
    fn from(accounts: &UpdateLpCooldownTimeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateLpCooldownTimeKeys> for [AccountMeta; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateLpCooldownTimeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN]> for UpdateLpCooldownTimeKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateLpCooldownTimeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateLpCooldownTimeAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN]>
    for UpdateLpCooldownTimeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateLpCooldownTimeIxArgs {
    pub lp_cooldown_time: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLpCooldownTimeIxData(pub UpdateLpCooldownTimeIxArgs);
pub const UPDATE_LP_COOLDOWN_TIME_IX_DISCM: [u8; 8] = [198, 133, 88, 41, 241, 119, 61, 14];
impl From<UpdateLpCooldownTimeIxArgs> for UpdateLpCooldownTimeIxData {
    fn from(args: UpdateLpCooldownTimeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLpCooldownTimeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LP_COOLDOWN_TIME_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_LP_COOLDOWN_TIME_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateLpCooldownTimeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LP_COOLDOWN_TIME_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lp_cooldown_time_ix<
    K: Into<UpdateLpCooldownTimeKeys>,
    A: Into<UpdateLpCooldownTimeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateLpCooldownTimeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateLpCooldownTimeIxArgs = args.into();
    let data: UpdateLpCooldownTimeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lp_cooldown_time_invoke<'info, A: Into<UpdateLpCooldownTimeIxArgs>>(
    accounts: &UpdateLpCooldownTimeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_lp_cooldown_time_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_lp_cooldown_time_invoke_signed<'info, A: Into<UpdateLpCooldownTimeIxArgs>>(
    accounts: &UpdateLpCooldownTimeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_lp_cooldown_time_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_lp_cooldown_time_verify_account_keys(
    accounts: &UpdateLpCooldownTimeAccounts<'_, '_>,
    keys: &UpdateLpCooldownTimeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_lp_cooldown_time_verify_account_privileges<'me, 'info>(
    accounts: &UpdateLpCooldownTimeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpFeeStructureAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpFeeStructureKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdatePerpFeeStructureAccounts<'_, '_>> for UpdatePerpFeeStructureKeys {
    fn from(accounts: &UpdatePerpFeeStructureAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdatePerpFeeStructureKeys>
    for [AccountMeta; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpFeeStructureKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]> for UpdatePerpFeeStructureKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdatePerpFeeStructureAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpFeeStructureAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]>
    for UpdatePerpFeeStructureAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpFeeStructureIxArgs {
    pub fee_structure: FeeStructure,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpFeeStructureIxData(pub UpdatePerpFeeStructureIxArgs);
pub const UPDATE_PERP_FEE_STRUCTURE_IX_DISCM: [u8; 8] = [23, 178, 111, 203, 73, 22, 140, 75];
impl From<UpdatePerpFeeStructureIxArgs> for UpdatePerpFeeStructureIxData {
    fn from(args: UpdatePerpFeeStructureIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpFeeStructureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_FEE_STRUCTURE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_FEE_STRUCTURE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpFeeStructureIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_FEE_STRUCTURE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_fee_structure_ix<
    K: Into<UpdatePerpFeeStructureKeys>,
    A: Into<UpdatePerpFeeStructureIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpFeeStructureKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpFeeStructureIxArgs = args.into();
    let data: UpdatePerpFeeStructureIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_fee_structure_invoke<'info, A: Into<UpdatePerpFeeStructureIxArgs>>(
    accounts: &UpdatePerpFeeStructureAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_fee_structure_invoke_signed<'info, A: Into<UpdatePerpFeeStructureIxArgs>>(
    accounts: &UpdatePerpFeeStructureAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_fee_structure_verify_account_keys(
    accounts: &UpdatePerpFeeStructureAccounts<'_, '_>,
    keys: &UpdatePerpFeeStructureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_fee_structure_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpFeeStructureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotFeeStructureAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotFeeStructureKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateSpotFeeStructureAccounts<'_, '_>> for UpdateSpotFeeStructureKeys {
    fn from(accounts: &UpdateSpotFeeStructureAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateSpotFeeStructureKeys>
    for [AccountMeta; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotFeeStructureKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]> for UpdateSpotFeeStructureKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateSpotFeeStructureAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotFeeStructureAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]>
    for UpdateSpotFeeStructureAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotFeeStructureIxArgs {
    pub fee_structure: FeeStructure,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotFeeStructureIxData(pub UpdateSpotFeeStructureIxArgs);
pub const UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM: [u8; 8] = [97, 216, 105, 131, 113, 246, 142, 141];
impl From<UpdateSpotFeeStructureIxArgs> for UpdateSpotFeeStructureIxData {
    fn from(args: UpdateSpotFeeStructureIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotFeeStructureIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotFeeStructureIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_fee_structure_ix<
    K: Into<UpdateSpotFeeStructureKeys>,
    A: Into<UpdateSpotFeeStructureIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotFeeStructureKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotFeeStructureIxArgs = args.into();
    let data: UpdateSpotFeeStructureIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_fee_structure_invoke<'info, A: Into<UpdateSpotFeeStructureIxArgs>>(
    accounts: &UpdateSpotFeeStructureAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_fee_structure_invoke_signed<'info, A: Into<UpdateSpotFeeStructureIxArgs>>(
    accounts: &UpdateSpotFeeStructureAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_fee_structure_verify_account_keys(
    accounts: &UpdateSpotFeeStructureAccounts<'_, '_>,
    keys: &UpdateSpotFeeStructureKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_fee_structure_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotFeeStructureAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateInitialPctToLiquidateAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateInitialPctToLiquidateKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateInitialPctToLiquidateAccounts<'_, '_>> for UpdateInitialPctToLiquidateKeys {
    fn from(accounts: &UpdateInitialPctToLiquidateAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateInitialPctToLiquidateKeys>
    for [AccountMeta; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateInitialPctToLiquidateKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]>
    for UpdateInitialPctToLiquidateKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateInitialPctToLiquidateAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateInitialPctToLiquidateAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]>
    for UpdateInitialPctToLiquidateAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateInitialPctToLiquidateIxArgs {
    pub initial_pct_to_liquidate: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateInitialPctToLiquidateIxData(pub UpdateInitialPctToLiquidateIxArgs);
pub const UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM: [u8; 8] =
    [210, 133, 225, 128, 194, 50, 13, 109];
impl From<UpdateInitialPctToLiquidateIxArgs> for UpdateInitialPctToLiquidateIxData {
    fn from(args: UpdateInitialPctToLiquidateIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateInitialPctToLiquidateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateInitialPctToLiquidateIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_initial_pct_to_liquidate_ix<
    K: Into<UpdateInitialPctToLiquidateKeys>,
    A: Into<UpdateInitialPctToLiquidateIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateInitialPctToLiquidateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateInitialPctToLiquidateIxArgs = args.into();
    let data: UpdateInitialPctToLiquidateIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_initial_pct_to_liquidate_invoke<'info, A: Into<UpdateInitialPctToLiquidateIxArgs>>(
    accounts: &UpdateInitialPctToLiquidateAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_initial_pct_to_liquidate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_initial_pct_to_liquidate_invoke_signed<
    'info,
    A: Into<UpdateInitialPctToLiquidateIxArgs>,
>(
    accounts: &UpdateInitialPctToLiquidateAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_initial_pct_to_liquidate_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_initial_pct_to_liquidate_verify_account_keys(
    accounts: &UpdateInitialPctToLiquidateAccounts<'_, '_>,
    keys: &UpdateInitialPctToLiquidateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_initial_pct_to_liquidate_verify_account_privileges<'me, 'info>(
    accounts: &UpdateInitialPctToLiquidateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLiquidationDurationAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLiquidationDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateLiquidationDurationAccounts<'_, '_>> for UpdateLiquidationDurationKeys {
    fn from(accounts: &UpdateLiquidationDurationAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateLiquidationDurationKeys>
    for [AccountMeta; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateLiquidationDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]> for UpdateLiquidationDurationKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateLiquidationDurationAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateLiquidationDurationAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]>
    for UpdateLiquidationDurationAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateLiquidationDurationIxArgs {
    pub liquidation_duration: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLiquidationDurationIxData(pub UpdateLiquidationDurationIxArgs);
pub const UPDATE_LIQUIDATION_DURATION_IX_DISCM: [u8; 8] = [28, 154, 20, 249, 102, 192, 73, 71];
impl From<UpdateLiquidationDurationIxArgs> for UpdateLiquidationDurationIxData {
    fn from(args: UpdateLiquidationDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLiquidationDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LIQUIDATION_DURATION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_LIQUIDATION_DURATION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateLiquidationDurationIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LIQUIDATION_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_liquidation_duration_ix<
    K: Into<UpdateLiquidationDurationKeys>,
    A: Into<UpdateLiquidationDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateLiquidationDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateLiquidationDurationIxArgs = args.into();
    let data: UpdateLiquidationDurationIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_liquidation_duration_invoke<'info, A: Into<UpdateLiquidationDurationIxArgs>>(
    accounts: &UpdateLiquidationDurationAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_liquidation_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_liquidation_duration_invoke_signed<
    'info,
    A: Into<UpdateLiquidationDurationIxArgs>,
>(
    accounts: &UpdateLiquidationDurationAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_liquidation_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_liquidation_duration_verify_account_keys(
    accounts: &UpdateLiquidationDurationAccounts<'_, '_>,
    keys: &UpdateLiquidationDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_liquidation_duration_verify_account_privileges<'me, 'info>(
    accounts: &UpdateLiquidationDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleGuardRailsAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateOracleGuardRailsKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateOracleGuardRailsAccounts<'_, '_>> for UpdateOracleGuardRailsKeys {
    fn from(accounts: &UpdateOracleGuardRailsAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateOracleGuardRailsKeys>
    for [AccountMeta; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateOracleGuardRailsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]> for UpdateOracleGuardRailsKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateOracleGuardRailsAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateOracleGuardRailsAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]>
    for UpdateOracleGuardRailsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateOracleGuardRailsIxArgs {
    pub oracle_guard_rails: OracleGuardRails,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateOracleGuardRailsIxData(pub UpdateOracleGuardRailsIxArgs);
pub const UPDATE_ORACLE_GUARD_RAILS_IX_DISCM: [u8; 8] = [131, 112, 10, 59, 32, 54, 40, 164];
impl From<UpdateOracleGuardRailsIxArgs> for UpdateOracleGuardRailsIxData {
    fn from(args: UpdateOracleGuardRailsIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateOracleGuardRailsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ORACLE_GUARD_RAILS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_ORACLE_GUARD_RAILS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateOracleGuardRailsIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_GUARD_RAILS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_oracle_guard_rails_ix<
    K: Into<UpdateOracleGuardRailsKeys>,
    A: Into<UpdateOracleGuardRailsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateOracleGuardRailsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateOracleGuardRailsIxArgs = args.into();
    let data: UpdateOracleGuardRailsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracle_guard_rails_invoke<'info, A: Into<UpdateOracleGuardRailsIxArgs>>(
    accounts: &UpdateOracleGuardRailsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_oracle_guard_rails_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_oracle_guard_rails_invoke_signed<'info, A: Into<UpdateOracleGuardRailsIxArgs>>(
    accounts: &UpdateOracleGuardRailsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_oracle_guard_rails_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_oracle_guard_rails_verify_account_keys(
    accounts: &UpdateOracleGuardRailsAccounts<'_, '_>,
    keys: &UpdateOracleGuardRailsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_oracle_guard_rails_verify_account_privileges<'me, 'info>(
    accounts: &UpdateOracleGuardRailsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateStateSettlementDurationAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateStateSettlementDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateStateSettlementDurationAccounts<'_, '_>> for UpdateStateSettlementDurationKeys {
    fn from(accounts: &UpdateStateSettlementDurationAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateStateSettlementDurationKeys>
    for [AccountMeta; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateStateSettlementDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]>
    for UpdateStateSettlementDurationKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateStateSettlementDurationAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateStateSettlementDurationAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]>
    for UpdateStateSettlementDurationAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateStateSettlementDurationIxArgs {
    pub settlement_duration: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStateSettlementDurationIxData(pub UpdateStateSettlementDurationIxArgs);
pub const UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM: [u8; 8] = [97, 68, 199, 235, 131, 80, 61, 173];
impl From<UpdateStateSettlementDurationIxArgs> for UpdateStateSettlementDurationIxData {
    fn from(args: UpdateStateSettlementDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateStateSettlementDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateStateSettlementDurationIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_state_settlement_duration_ix<
    K: Into<UpdateStateSettlementDurationKeys>,
    A: Into<UpdateStateSettlementDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateStateSettlementDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateStateSettlementDurationIxArgs = args.into();
    let data: UpdateStateSettlementDurationIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_state_settlement_duration_invoke<
    'info,
    A: Into<UpdateStateSettlementDurationIxArgs>,
>(
    accounts: &UpdateStateSettlementDurationAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_state_settlement_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_state_settlement_duration_invoke_signed<
    'info,
    A: Into<UpdateStateSettlementDurationIxArgs>,
>(
    accounts: &UpdateStateSettlementDurationAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_state_settlement_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_state_settlement_duration_verify_account_keys(
    accounts: &UpdateStateSettlementDurationAccounts<'_, '_>,
    keys: &UpdateStateSettlementDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_state_settlement_duration_verify_account_privileges<'me, 'info>(
    accounts: &UpdateStateSettlementDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketOracleAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketOracleKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl From<&UpdatePerpMarketOracleAccounts<'_, '_>> for UpdatePerpMarketOracleKeys {
    fn from(accounts: &UpdatePerpMarketOracleAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&UpdatePerpMarketOracleKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketOracleKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]> for UpdatePerpMarketOracleKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            perp_market: pubkeys[1],
            oracle: pubkeys[2],
            admin: pubkeys[3],
        }
    }
}
impl<'info> From<&UpdatePerpMarketOracleAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketOracleAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketOracleAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            perp_market: &arr[1],
            oracle: &arr[2],
            admin: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketOracleIxArgs {
    pub oracle: Pubkey,
    pub oracle_source: OracleSource,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketOracleIxData(pub UpdatePerpMarketOracleIxArgs);
pub const UPDATE_PERP_MARKET_ORACLE_IX_DISCM: [u8; 8] = [182, 113, 111, 160, 67, 174, 89, 191];
impl From<UpdatePerpMarketOracleIxArgs> for UpdatePerpMarketOracleIxData {
    fn from(args: UpdatePerpMarketOracleIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketOracleIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_ORACLE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_ORACLE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketOracleIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_ORACLE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_oracle_ix<
    K: Into<UpdatePerpMarketOracleKeys>,
    A: Into<UpdatePerpMarketOracleIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketOracleKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketOracleIxArgs = args.into();
    let data: UpdatePerpMarketOracleIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_oracle_invoke<'info, A: Into<UpdatePerpMarketOracleIxArgs>>(
    accounts: &UpdatePerpMarketOracleAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_oracle_invoke_signed<'info, A: Into<UpdatePerpMarketOracleIxArgs>>(
    accounts: &UpdatePerpMarketOracleAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_oracle_verify_account_keys(
    accounts: &UpdatePerpMarketOracleAccounts<'_, '_>,
    keys: &UpdatePerpMarketOracleKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
        (accounts.oracle.key, &keys.oracle),
        (accounts.admin.key, &keys.admin),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_oracle_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketOracleAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketBaseSpreadAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketBaseSpreadKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketBaseSpreadAccounts<'_, '_>> for UpdatePerpMarketBaseSpreadKeys {
    fn from(accounts: &UpdatePerpMarketBaseSpreadAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketBaseSpreadKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketBaseSpreadKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketBaseSpreadKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketBaseSpreadAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketBaseSpreadAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketBaseSpreadIxArgs {
    pub base_spread: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketBaseSpreadIxData(pub UpdatePerpMarketBaseSpreadIxArgs);
pub const UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM: [u8; 8] = [71, 95, 84, 168, 9, 157, 198, 65];
impl From<UpdatePerpMarketBaseSpreadIxArgs> for UpdatePerpMarketBaseSpreadIxData {
    fn from(args: UpdatePerpMarketBaseSpreadIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketBaseSpreadIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketBaseSpreadIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_base_spread_ix<
    K: Into<UpdatePerpMarketBaseSpreadKeys>,
    A: Into<UpdatePerpMarketBaseSpreadIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketBaseSpreadKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketBaseSpreadIxArgs = args.into();
    let data: UpdatePerpMarketBaseSpreadIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_base_spread_invoke<'info, A: Into<UpdatePerpMarketBaseSpreadIxArgs>>(
    accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_base_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_base_spread_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketBaseSpreadIxArgs>,
>(
    accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_base_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_base_spread_verify_account_keys(
    accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, '_>,
    keys: &UpdatePerpMarketBaseSpreadKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_base_spread_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketBaseSpreadAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmJitIntensityAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAmmJitIntensityKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdateAmmJitIntensityAccounts<'_, '_>> for UpdateAmmJitIntensityKeys {
    fn from(accounts: &UpdateAmmJitIntensityAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdateAmmJitIntensityKeys> for [AccountMeta; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateAmmJitIntensityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN]> for UpdateAmmJitIntensityKeys {
    fn from(pubkeys: [Pubkey; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdateAmmJitIntensityAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateAmmJitIntensityAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN]>
    for UpdateAmmJitIntensityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateAmmJitIntensityIxArgs {
    pub amm_jit_intensity: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAmmJitIntensityIxData(pub UpdateAmmJitIntensityIxArgs);
pub const UPDATE_AMM_JIT_INTENSITY_IX_DISCM: [u8; 8] = [181, 191, 53, 109, 166, 249, 55, 142];
impl From<UpdateAmmJitIntensityIxArgs> for UpdateAmmJitIntensityIxData {
    fn from(args: UpdateAmmJitIntensityIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAmmJitIntensityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_AMM_JIT_INTENSITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_AMM_JIT_INTENSITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateAmmJitIntensityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AMM_JIT_INTENSITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_amm_jit_intensity_ix<
    K: Into<UpdateAmmJitIntensityKeys>,
    A: Into<UpdateAmmJitIntensityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateAmmJitIntensityKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateAmmJitIntensityIxArgs = args.into();
    let data: UpdateAmmJitIntensityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_amm_jit_intensity_invoke<'info, A: Into<UpdateAmmJitIntensityIxArgs>>(
    accounts: &UpdateAmmJitIntensityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_amm_jit_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_amm_jit_intensity_invoke_signed<'info, A: Into<UpdateAmmJitIntensityIxArgs>>(
    accounts: &UpdateAmmJitIntensityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_amm_jit_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_amm_jit_intensity_verify_account_keys(
    accounts: &UpdateAmmJitIntensityAccounts<'_, '_>,
    keys: &UpdateAmmJitIntensityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_amm_jit_intensity_verify_account_privileges<'me, 'info>(
    accounts: &UpdateAmmJitIntensityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSpreadAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxSpreadKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMaxSpreadAccounts<'_, '_>> for UpdatePerpMarketMaxSpreadKeys {
    fn from(accounts: &UpdatePerpMarketMaxSpreadAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxSpreadKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxSpreadKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxSpreadKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMaxSpreadAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxSpreadAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMaxSpreadIxArgs {
    pub max_spread: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxSpreadIxData(pub UpdatePerpMarketMaxSpreadIxArgs);
pub const UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM: [u8; 8] = [80, 252, 122, 62, 40, 218, 91, 100];
impl From<UpdatePerpMarketMaxSpreadIxArgs> for UpdatePerpMarketMaxSpreadIxData {
    fn from(args: UpdatePerpMarketMaxSpreadIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMaxSpreadIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketMaxSpreadIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_max_spread_ix<
    K: Into<UpdatePerpMarketMaxSpreadKeys>,
    A: Into<UpdatePerpMarketMaxSpreadIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxSpreadKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMaxSpreadIxArgs = args.into();
    let data: UpdatePerpMarketMaxSpreadIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_spread_invoke<'info, A: Into<UpdatePerpMarketMaxSpreadIxArgs>>(
    accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_spread_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMaxSpreadIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_max_spread_verify_account_keys(
    accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, '_>,
    keys: &UpdatePerpMarketMaxSpreadKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_max_spread_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMaxSpreadAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStepSizeAndTickSizeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketStepSizeAndTickSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, '_>>
    for UpdatePerpMarketStepSizeAndTickSizeKeys
{
    fn from(accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketStepSizeAndTickSizeKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketStepSizeAndTickSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketStepSizeAndTickSizeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketStepSizeAndTickSizeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketStepSizeAndTickSizeIxArgs {
    pub step_size: u64,
    pub tick_size: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketStepSizeAndTickSizeIxData(pub UpdatePerpMarketStepSizeAndTickSizeIxArgs);
pub const UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM: [u8; 8] =
    [231, 255, 97, 25, 146, 139, 174, 4];
impl From<UpdatePerpMarketStepSizeAndTickSizeIxArgs> for UpdatePerpMarketStepSizeAndTickSizeIxData {
    fn from(args: UpdatePerpMarketStepSizeAndTickSizeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketStepSizeAndTickSizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdatePerpMarketStepSizeAndTickSizeIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_step_size_and_tick_size_ix<
    K: Into<UpdatePerpMarketStepSizeAndTickSizeKeys>,
    A: Into<UpdatePerpMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketStepSizeAndTickSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketStepSizeAndTickSizeIxArgs = args.into();
    let data: UpdatePerpMarketStepSizeAndTickSizeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_step_size_and_tick_size_invoke<
    'info,
    A: Into<UpdatePerpMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_step_size_and_tick_size_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_step_size_and_tick_size_verify_account_keys(
    accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, '_>,
    keys: &UpdatePerpMarketStepSizeAndTickSizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_step_size_and_tick_size_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketNameAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketNameKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketNameAccounts<'_, '_>> for UpdatePerpMarketNameKeys {
    fn from(accounts: &UpdatePerpMarketNameAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketNameKeys> for [AccountMeta; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdatePerpMarketNameKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN]> for UpdatePerpMarketNameKeys {
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketNameAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketNameAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketNameAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketNameIxArgs {
    pub name: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketNameIxData(pub UpdatePerpMarketNameIxArgs);
pub const UPDATE_PERP_MARKET_NAME_IX_DISCM: [u8; 8] = [211, 31, 21, 210, 64, 108, 66, 201];
impl From<UpdatePerpMarketNameIxArgs> for UpdatePerpMarketNameIxData {
    fn from(args: UpdatePerpMarketNameIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketNameIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_NAME_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_NAME_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketNameIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_NAME_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_name_ix<
    K: Into<UpdatePerpMarketNameKeys>,
    A: Into<UpdatePerpMarketNameIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketNameKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketNameIxArgs = args.into();
    let data: UpdatePerpMarketNameIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_name_invoke<'info, A: Into<UpdatePerpMarketNameIxArgs>>(
    accounts: &UpdatePerpMarketNameAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_name_invoke_signed<'info, A: Into<UpdatePerpMarketNameIxArgs>>(
    accounts: &UpdatePerpMarketNameAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_name_verify_account_keys(
    accounts: &UpdatePerpMarketNameAccounts<'_, '_>,
    keys: &UpdatePerpMarketNameKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_name_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketNameAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMinOrderSizeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMinOrderSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMinOrderSizeAccounts<'_, '_>> for UpdatePerpMarketMinOrderSizeKeys {
    fn from(accounts: &UpdatePerpMarketMinOrderSizeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMinOrderSizeKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMinOrderSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMinOrderSizeKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMinOrderSizeAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMinOrderSizeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMinOrderSizeIxArgs {
    pub order_size: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMinOrderSizeIxData(pub UpdatePerpMarketMinOrderSizeIxArgs);
pub const UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM: [u8; 8] = [226, 74, 5, 89, 108, 223, 46, 141];
impl From<UpdatePerpMarketMinOrderSizeIxArgs> for UpdatePerpMarketMinOrderSizeIxData {
    fn from(args: UpdatePerpMarketMinOrderSizeIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMinOrderSizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketMinOrderSizeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_min_order_size_ix<
    K: Into<UpdatePerpMarketMinOrderSizeKeys>,
    A: Into<UpdatePerpMarketMinOrderSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMinOrderSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMinOrderSizeIxArgs = args.into();
    let data: UpdatePerpMarketMinOrderSizeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_min_order_size_invoke<
    'info,
    A: Into<UpdatePerpMarketMinOrderSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_min_order_size_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMinOrderSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_min_order_size_verify_account_keys(
    accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, '_>,
    keys: &UpdatePerpMarketMinOrderSizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_min_order_size_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMinOrderSizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSlippageRatioAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxSlippageRatioKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMaxSlippageRatioAccounts<'_, '_>>
    for UpdatePerpMarketMaxSlippageRatioKeys
{
    fn from(accounts: &UpdatePerpMarketMaxSlippageRatioAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxSlippageRatioKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxSlippageRatioKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxSlippageRatioKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxSlippageRatioAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMaxSlippageRatioIxArgs {
    pub max_slippage_ratio: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxSlippageRatioIxData(pub UpdatePerpMarketMaxSlippageRatioIxArgs);
pub const UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM: [u8; 8] =
    [235, 37, 40, 196, 70, 146, 54, 201];
impl From<UpdatePerpMarketMaxSlippageRatioIxArgs> for UpdatePerpMarketMaxSlippageRatioIxData {
    fn from(args: UpdatePerpMarketMaxSlippageRatioIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMaxSlippageRatioIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketMaxSlippageRatioIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_max_slippage_ratio_ix<
    K: Into<UpdatePerpMarketMaxSlippageRatioKeys>,
    A: Into<UpdatePerpMarketMaxSlippageRatioIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxSlippageRatioKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketMaxSlippageRatioIxArgs = args.into();
    let data: UpdatePerpMarketMaxSlippageRatioIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_slippage_ratio_invoke<
    'info,
    A: Into<UpdatePerpMarketMaxSlippageRatioIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_slippage_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_slippage_ratio_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMaxSlippageRatioIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_slippage_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_max_slippage_ratio_verify_account_keys(
    accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, '_>,
    keys: &UpdatePerpMarketMaxSlippageRatioKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_max_slippage_ratio_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxFillReserveFractionAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxFillReserveFractionKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMaxFillReserveFractionAccounts<'_, '_>>
    for UpdatePerpMarketMaxFillReserveFractionKeys
{
    fn from(accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxFillReserveFractionKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxFillReserveFractionKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxFillReserveFractionKeys
{
    fn from(
        pubkeys: [Pubkey; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxFillReserveFractionAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>;
                 UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMaxFillReserveFractionIxArgs {
    pub max_fill_reserve_fraction: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxFillReserveFractionIxData(
    pub UpdatePerpMarketMaxFillReserveFractionIxArgs,
);
pub const UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM: [u8; 8] =
    [19, 172, 114, 154, 42, 135, 161, 133];
impl From<UpdatePerpMarketMaxFillReserveFractionIxArgs>
    for UpdatePerpMarketMaxFillReserveFractionIxData
{
    fn from(args: UpdatePerpMarketMaxFillReserveFractionIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMaxFillReserveFractionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            UpdatePerpMarketMaxFillReserveFractionIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_max_fill_reserve_fraction_ix<
    K: Into<UpdatePerpMarketMaxFillReserveFractionKeys>,
    A: Into<UpdatePerpMarketMaxFillReserveFractionIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxFillReserveFractionKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketMaxFillReserveFractionIxArgs = args.into();
    let data: UpdatePerpMarketMaxFillReserveFractionIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_fill_reserve_fraction_invoke<
    'info,
    A: Into<UpdatePerpMarketMaxFillReserveFractionIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_fill_reserve_fraction_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_fill_reserve_fraction_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMaxFillReserveFractionIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_fill_reserve_fraction_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_max_fill_reserve_fraction_verify_account_keys(
    accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, '_>,
    keys: &UpdatePerpMarketMaxFillReserveFractionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_max_fill_reserve_fraction_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxOpenInterestAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub perp_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxOpenInterestKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl From<&UpdatePerpMarketMaxOpenInterestAccounts<'_, '_>>
    for UpdatePerpMarketMaxOpenInterestKeys
{
    fn from(accounts: &UpdatePerpMarketMaxOpenInterestAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxOpenInterestKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxOpenInterestKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxOpenInterestKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            perp_market: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePerpMarketMaxOpenInterestAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]>
    for UpdatePerpMarketMaxOpenInterestAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            perp_market: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpMarketMaxOpenInterestIxArgs {
    pub max_open_interest: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpMarketMaxOpenInterestIxData(pub UpdatePerpMarketMaxOpenInterestIxArgs);
pub const UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM: [u8; 8] =
    [194, 79, 149, 224, 246, 102, 186, 140];
impl From<UpdatePerpMarketMaxOpenInterestIxArgs> for UpdatePerpMarketMaxOpenInterestIxData {
    fn from(args: UpdatePerpMarketMaxOpenInterestIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpMarketMaxOpenInterestIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpMarketMaxOpenInterestIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_market_max_open_interest_ix<
    K: Into<UpdatePerpMarketMaxOpenInterestKeys>,
    A: Into<UpdatePerpMarketMaxOpenInterestIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxOpenInterestKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMaxOpenInterestIxArgs = args.into();
    let data: UpdatePerpMarketMaxOpenInterestIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_open_interest_invoke<
    'info,
    A: Into<UpdatePerpMarketMaxOpenInterestIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_open_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_open_interest_invoke_signed<
    'info,
    A: Into<UpdatePerpMarketMaxOpenInterestIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_open_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_market_max_open_interest_verify_account_keys(
    accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, '_>,
    keys: &UpdatePerpMarketMaxOpenInterestKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.perp_market.key, &keys.perp_market),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_market_max_open_interest_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.perp_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_ADMIN_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAdminAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateAdminKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateAdminAccounts<'_, '_>> for UpdateAdminKeys {
    fn from(accounts: &UpdateAdminAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateAdminKeys> for [AccountMeta; UPDATE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateAdminKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_ADMIN_IX_ACCOUNTS_LEN]> for UpdateAdminKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateAdminAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateAdminAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN]>
    for UpdateAdminAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateAdminIxArgs {
    pub admin: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateAdminIxData(pub UpdateAdminIxArgs);
pub const UPDATE_ADMIN_IX_DISCM: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
impl From<UpdateAdminIxArgs> for UpdateAdminIxData {
    fn from(args: UpdateAdminIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateAdminIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_ADMIN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_ADMIN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateAdminIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ADMIN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_admin_ix<K: Into<UpdateAdminKeys>, A: Into<UpdateAdminIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateAdminKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateAdminIxArgs = args.into();
    let data: UpdateAdminIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_admin_invoke<'info, A: Into<UpdateAdminIxArgs>>(
    accounts: &UpdateAdminAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_admin_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_admin_invoke_signed<'info, A: Into<UpdateAdminIxArgs>>(
    accounts: &UpdateAdminAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_admin_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_admin_verify_account_keys(
    accounts: &UpdateAdminAccounts<'_, '_>,
    keys: &UpdateAdminKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_admin_verify_account_privileges<'me, 'info>(
    accounts: &UpdateAdminAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateWhitelistMintAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateWhitelistMintKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateWhitelistMintAccounts<'_, '_>> for UpdateWhitelistMintKeys {
    fn from(accounts: &UpdateWhitelistMintAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateWhitelistMintKeys> for [AccountMeta; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateWhitelistMintKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN]> for UpdateWhitelistMintKeys {
    fn from(pubkeys: [Pubkey; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateWhitelistMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateWhitelistMintAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN]>
    for UpdateWhitelistMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateWhitelistMintIxArgs {
    pub whitelist_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateWhitelistMintIxData(pub UpdateWhitelistMintIxArgs);
pub const UPDATE_WHITELIST_MINT_IX_DISCM: [u8; 8] = [161, 15, 162, 19, 148, 120, 144, 151];
impl From<UpdateWhitelistMintIxArgs> for UpdateWhitelistMintIxData {
    fn from(args: UpdateWhitelistMintIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateWhitelistMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_WHITELIST_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_WHITELIST_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateWhitelistMintIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WHITELIST_MINT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_whitelist_mint_ix<
    K: Into<UpdateWhitelistMintKeys>,
    A: Into<UpdateWhitelistMintIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateWhitelistMintKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateWhitelistMintIxArgs = args.into();
    let data: UpdateWhitelistMintIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_whitelist_mint_invoke<'info, A: Into<UpdateWhitelistMintIxArgs>>(
    accounts: &UpdateWhitelistMintAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_whitelist_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_whitelist_mint_invoke_signed<'info, A: Into<UpdateWhitelistMintIxArgs>>(
    accounts: &UpdateWhitelistMintAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_whitelist_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_whitelist_mint_verify_account_keys(
    accounts: &UpdateWhitelistMintAccounts<'_, '_>,
    keys: &UpdateWhitelistMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_whitelist_mint_verify_account_privileges<'me, 'info>(
    accounts: &UpdateWhitelistMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDiscountMintAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDiscountMintKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateDiscountMintAccounts<'_, '_>> for UpdateDiscountMintKeys {
    fn from(accounts: &UpdateDiscountMintAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateDiscountMintKeys> for [AccountMeta; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateDiscountMintKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN]> for UpdateDiscountMintKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateDiscountMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateDiscountMintAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN]>
    for UpdateDiscountMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateDiscountMintIxArgs {
    pub discount_mint: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDiscountMintIxData(pub UpdateDiscountMintIxArgs);
pub const UPDATE_DISCOUNT_MINT_IX_DISCM: [u8; 8] = [32, 252, 122, 211, 66, 31, 47, 241];
impl From<UpdateDiscountMintIxArgs> for UpdateDiscountMintIxData {
    fn from(args: UpdateDiscountMintIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateDiscountMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_DISCOUNT_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_DISCOUNT_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateDiscountMintIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DISCOUNT_MINT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_discount_mint_ix<
    K: Into<UpdateDiscountMintKeys>,
    A: Into<UpdateDiscountMintIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateDiscountMintKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateDiscountMintIxArgs = args.into();
    let data: UpdateDiscountMintIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_discount_mint_invoke<'info, A: Into<UpdateDiscountMintIxArgs>>(
    accounts: &UpdateDiscountMintAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_discount_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_discount_mint_invoke_signed<'info, A: Into<UpdateDiscountMintIxArgs>>(
    accounts: &UpdateDiscountMintAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_discount_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_discount_mint_verify_account_keys(
    accounts: &UpdateDiscountMintAccounts<'_, '_>,
    keys: &UpdateDiscountMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_discount_mint_verify_account_privileges<'me, 'info>(
    accounts: &UpdateDiscountMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExchangeStatusAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateExchangeStatusKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateExchangeStatusAccounts<'_, '_>> for UpdateExchangeStatusKeys {
    fn from(accounts: &UpdateExchangeStatusAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateExchangeStatusKeys> for [AccountMeta; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateExchangeStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN]> for UpdateExchangeStatusKeys {
    fn from(pubkeys: [Pubkey; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateExchangeStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateExchangeStatusAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN]>
    for UpdateExchangeStatusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateExchangeStatusIxArgs {
    pub exchange_status: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateExchangeStatusIxData(pub UpdateExchangeStatusIxArgs);
pub const UPDATE_EXCHANGE_STATUS_IX_DISCM: [u8; 8] = [83, 160, 252, 250, 129, 116, 49, 223];
impl From<UpdateExchangeStatusIxArgs> for UpdateExchangeStatusIxData {
    fn from(args: UpdateExchangeStatusIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateExchangeStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_EXCHANGE_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_EXCHANGE_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateExchangeStatusIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXCHANGE_STATUS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_exchange_status_ix<
    K: Into<UpdateExchangeStatusKeys>,
    A: Into<UpdateExchangeStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateExchangeStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateExchangeStatusIxArgs = args.into();
    let data: UpdateExchangeStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exchange_status_invoke<'info, A: Into<UpdateExchangeStatusIxArgs>>(
    accounts: &UpdateExchangeStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_exchange_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_exchange_status_invoke_signed<'info, A: Into<UpdateExchangeStatusIxArgs>>(
    accounts: &UpdateExchangeStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_exchange_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_exchange_status_verify_account_keys(
    accounts: &UpdateExchangeStatusAccounts<'_, '_>,
    keys: &UpdateExchangeStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_exchange_status_verify_account_privileges<'me, 'info>(
    accounts: &UpdateExchangeStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpAuctionDurationAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdatePerpAuctionDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdatePerpAuctionDurationAccounts<'_, '_>> for UpdatePerpAuctionDurationKeys {
    fn from(accounts: &UpdatePerpAuctionDurationAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdatePerpAuctionDurationKeys>
    for [AccountMeta; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpAuctionDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]>
    for UpdatePerpAuctionDurationKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdatePerpAuctionDurationAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpAuctionDurationAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]>
    for UpdatePerpAuctionDurationAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePerpAuctionDurationIxArgs {
    pub min_perp_auction_duration: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePerpAuctionDurationIxData(pub UpdatePerpAuctionDurationIxArgs);
pub const UPDATE_PERP_AUCTION_DURATION_IX_DISCM: [u8; 8] = [126, 110, 52, 174, 30, 206, 215, 90];
impl From<UpdatePerpAuctionDurationIxArgs> for UpdatePerpAuctionDurationIxData {
    fn from(args: UpdatePerpAuctionDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePerpAuctionDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_PERP_AUCTION_DURATION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PERP_AUCTION_DURATION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePerpAuctionDurationIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_AUCTION_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_perp_auction_duration_ix<
    K: Into<UpdatePerpAuctionDurationKeys>,
    A: Into<UpdatePerpAuctionDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpAuctionDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpAuctionDurationIxArgs = args.into();
    let data: UpdatePerpAuctionDurationIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_auction_duration_invoke<'info, A: Into<UpdatePerpAuctionDurationIxArgs>>(
    accounts: &UpdatePerpAuctionDurationAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_auction_duration_invoke_signed<
    'info,
    A: Into<UpdatePerpAuctionDurationIxArgs>,
>(
    accounts: &UpdatePerpAuctionDurationAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_perp_auction_duration_verify_account_keys(
    accounts: &UpdatePerpAuctionDurationAccounts<'_, '_>,
    keys: &UpdatePerpAuctionDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_perp_auction_duration_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePerpAuctionDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotAuctionDurationAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSpotAuctionDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl From<&UpdateSpotAuctionDurationAccounts<'_, '_>> for UpdateSpotAuctionDurationKeys {
    fn from(accounts: &UpdateSpotAuctionDurationAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateSpotAuctionDurationKeys>
    for [AccountMeta; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotAuctionDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]>
    for UpdateSpotAuctionDurationKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateSpotAuctionDurationAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotAuctionDurationAccounts<'_, 'info>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]>
    for UpdateSpotAuctionDurationAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSpotAuctionDurationIxArgs {
    pub default_spot_auction_duration: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSpotAuctionDurationIxData(pub UpdateSpotAuctionDurationIxArgs);
pub const UPDATE_SPOT_AUCTION_DURATION_IX_DISCM: [u8; 8] = [182, 178, 203, 72, 187, 143, 157, 107];
impl From<UpdateSpotAuctionDurationIxArgs> for UpdateSpotAuctionDurationIxData {
    fn from(args: UpdateSpotAuctionDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSpotAuctionDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SPOT_AUCTION_DURATION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_SPOT_AUCTION_DURATION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateSpotAuctionDurationIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_AUCTION_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_spot_auction_duration_ix<
    K: Into<UpdateSpotAuctionDurationKeys>,
    A: Into<UpdateSpotAuctionDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotAuctionDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotAuctionDurationIxArgs = args.into();
    let data: UpdateSpotAuctionDurationIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_auction_duration_invoke<'info, A: Into<UpdateSpotAuctionDurationIxArgs>>(
    accounts: &UpdateSpotAuctionDurationAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_auction_duration_invoke_signed<
    'info,
    A: Into<UpdateSpotAuctionDurationIxArgs>,
>(
    accounts: &UpdateSpotAuctionDurationAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_spot_auction_duration_verify_account_keys(
    accounts: &UpdateSpotAuctionDurationAccounts<'_, '_>,
    keys: &UpdateSpotAuctionDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_spot_auction_duration_verify_account_privileges<'me, 'info>(
    accounts: &UpdateSpotAuctionDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AdminRemoveInsuranceFundStakeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub spot_market: &'me AccountInfo<'info>,
    pub insurance_fund_vault: &'me AccountInfo<'info>,
    pub drift_signer: &'me AccountInfo<'info>,
    pub admin_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdminRemoveInsuranceFundStakeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub admin_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<&AdminRemoveInsuranceFundStakeAccounts<'_, '_>> for AdminRemoveInsuranceFundStakeKeys {
    fn from(accounts: &AdminRemoveInsuranceFundStakeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            admin_token_account: *accounts.admin_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&AdminRemoveInsuranceFundStakeKeys>
    for [AccountMeta; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &AdminRemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.admin_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for AdminRemoveInsuranceFundStakeKeys
{
    fn from(pubkeys: [Pubkey; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            spot_market: pubkeys[2],
            insurance_fund_vault: pubkeys[3],
            drift_signer: pubkeys[4],
            admin_token_account: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<&AdminRemoveInsuranceFundStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.admin_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]>
    for AdminRemoveInsuranceFundStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            spot_market: &arr[2],
            insurance_fund_vault: &arr[3],
            drift_signer: &arr[4],
            admin_token_account: &arr[5],
            token_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminRemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdminRemoveInsuranceFundStakeIxData(pub AdminRemoveInsuranceFundStakeIxArgs);
pub const ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] =
    [35, 13, 111, 220, 103, 217, 174, 115];
impl From<AdminRemoveInsuranceFundStakeIxArgs> for AdminRemoveInsuranceFundStakeIxData {
    fn from(args: AdminRemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl AdminRemoveInsuranceFundStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AdminRemoveInsuranceFundStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn admin_remove_insurance_fund_stake_ix<
    K: Into<AdminRemoveInsuranceFundStakeKeys>,
    A: Into<AdminRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AdminRemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AdminRemoveInsuranceFundStakeIxArgs = args.into();
    let data: AdminRemoveInsuranceFundStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_remove_insurance_fund_stake_invoke<
    'info,
    A: Into<AdminRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = admin_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn admin_remove_insurance_fund_stake_invoke_signed<
    'info,
    A: Into<AdminRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = admin_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn admin_remove_insurance_fund_stake_verify_account_keys(
    accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, '_>,
    keys: &AdminRemoveInsuranceFundStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.admin.key, &keys.admin),
        (accounts.state.key, &keys.state),
        (accounts.spot_market.key, &keys.spot_market),
        (
            accounts.insurance_fund_vault.key,
            &keys.insurance_fund_vault,
        ),
        (accounts.drift_signer.key, &keys.drift_signer),
        (accounts.admin_token_account.key, &keys.admin_token_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn admin_remove_insurance_fund_stake_verify_account_privileges<'me, 'info>(
    accounts: &AdminRemoveInsuranceFundStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.insurance_fund_vault, accounts.admin_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
