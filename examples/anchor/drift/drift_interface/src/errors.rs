use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DriftError {
    #[error("Invalid Spot Market Authority")]
    InvalidSpotMarketAuthority = 6000u32,
    #[error("Clearing house not insurance fund authority")]
    InvalidInsuranceFundAuthority = 6001u32,
    #[error("Insufficient deposit")]
    InsufficientDeposit = 6002u32,
    #[error("Insufficient collateral")]
    InsufficientCollateral = 6003u32,
    #[error("Sufficient collateral")]
    SufficientCollateral = 6004u32,
    #[error("Max number of positions taken")]
    MaxNumberOfPositions = 6005u32,
    #[error("Admin Controls Prices Disabled")]
    AdminControlsPricesDisabled = 6006u32,
    #[error("Market Delisted")]
    MarketDelisted = 6007u32,
    #[error("Market Index Already Initialized")]
    MarketIndexAlreadyInitialized = 6008u32,
    #[error("User Account And User Positions Account Mismatch")]
    UserAccountAndUserPositionsAccountMismatch = 6009u32,
    #[error("User Has No Position In Market")]
    UserHasNoPositionInMarket = 6010u32,
    #[error("Invalid Initial Peg")]
    InvalidInitialPeg = 6011u32,
    #[error("AMM repeg already configured with amt given")]
    InvalidRepegRedundant = 6012u32,
    #[error("AMM repeg incorrect repeg direction")]
    InvalidRepegDirection = 6013u32,
    #[error("AMM repeg out of bounds pnl")]
    InvalidRepegProfitability = 6014u32,
    #[error("Slippage Outside Limit Price")]
    SlippageOutsideLimit = 6015u32,
    #[error("Order Size Too Small")]
    OrderSizeTooSmall = 6016u32,
    #[error("Price change too large when updating K")]
    InvalidUpdateK = 6017u32,
    #[error("Admin tried to withdraw amount larger than fees collected")]
    AdminWithdrawTooLarge = 6018u32,
    #[error("Math Error")]
    MathError = 6019u32,
    #[error("Conversion to u128/u64 failed with an overflow or underflow")]
    BnConversionError = 6020u32,
    #[error("Clock unavailable")]
    ClockUnavailable = 6021u32,
    #[error("Unable To Load Oracles")]
    UnableToLoadOracle = 6022u32,
    #[error("Price Bands Breached")]
    PriceBandsBreached = 6023u32,
    #[error("Exchange is paused")]
    ExchangePaused = 6024u32,
    #[error("Invalid whitelist token")]
    InvalidWhitelistToken = 6025u32,
    #[error("Whitelist token not found")]
    WhitelistTokenNotFound = 6026u32,
    #[error("Invalid discount token")]
    InvalidDiscountToken = 6027u32,
    #[error("Discount token not found")]
    DiscountTokenNotFound = 6028u32,
    #[error("Referrer not found")]
    ReferrerNotFound = 6029u32,
    #[error("ReferrerNotFound")]
    ReferrerStatsNotFound = 6030u32,
    #[error("ReferrerMustBeWritable")]
    ReferrerMustBeWritable = 6031u32,
    #[error("ReferrerMustBeWritable")]
    ReferrerStatsMustBeWritable = 6032u32,
    #[error("ReferrerAndReferrerStatsAuthorityUnequal")]
    ReferrerAndReferrerStatsAuthorityUnequal = 6033u32,
    #[error("InvalidReferrer")]
    InvalidReferrer = 6034u32,
    #[error("InvalidOracle")]
    InvalidOracle = 6035u32,
    #[error("OracleNotFound")]
    OracleNotFound = 6036u32,
    #[error("Liquidations Blocked By Oracle")]
    LiquidationsBlockedByOracle = 6037u32,
    #[error("Can not deposit more than max deposit")]
    MaxDeposit = 6038u32,
    #[error("Can not delete user that still has collateral")]
    CantDeleteUserWithCollateral = 6039u32,
    #[error("AMM funding out of bounds pnl")]
    InvalidFundingProfitability = 6040u32,
    #[error("Casting Failure")]
    CastingFailure = 6041u32,
    #[error("InvalidOrder")]
    InvalidOrder = 6042u32,
    #[error("InvalidOrderMaxTs")]
    InvalidOrderMaxTs = 6043u32,
    #[error("InvalidOrderMarketType")]
    InvalidOrderMarketType = 6044u32,
    #[error("InvalidOrderForInitialMarginReq")]
    InvalidOrderForInitialMarginReq = 6045u32,
    #[error("InvalidOrderNotRiskReducing")]
    InvalidOrderNotRiskReducing = 6046u32,
    #[error("InvalidOrderSizeTooSmall")]
    InvalidOrderSizeTooSmall = 6047u32,
    #[error("InvalidOrderNotStepSizeMultiple")]
    InvalidOrderNotStepSizeMultiple = 6048u32,
    #[error("InvalidOrderBaseQuoteAsset")]
    InvalidOrderBaseQuoteAsset = 6049u32,
    #[error("InvalidOrderIOC")]
    InvalidOrderIoc = 6050u32,
    #[error("InvalidOrderPostOnly")]
    InvalidOrderPostOnly = 6051u32,
    #[error("InvalidOrderIOCPostOnly")]
    InvalidOrderIocPostOnly = 6052u32,
    #[error("InvalidOrderTrigger")]
    InvalidOrderTrigger = 6053u32,
    #[error("InvalidOrderAuction")]
    InvalidOrderAuction = 6054u32,
    #[error("InvalidOrderOracleOffset")]
    InvalidOrderOracleOffset = 6055u32,
    #[error("InvalidOrderMinOrderSize")]
    InvalidOrderMinOrderSize = 6056u32,
    #[error("Failed to Place Post-Only Limit Order")]
    PlacePostOnlyLimitFailure = 6057u32,
    #[error("User has no order")]
    UserHasNoOrder = 6058u32,
    #[error("Order Amount Too Small")]
    OrderAmountTooSmall = 6059u32,
    #[error("Max number of orders taken")]
    MaxNumberOfOrders = 6060u32,
    #[error("Order does not exist")]
    OrderDoesNotExist = 6061u32,
    #[error("Order not open")]
    OrderNotOpen = 6062u32,
    #[error("FillOrderDidNotUpdateState")]
    FillOrderDidNotUpdateState = 6063u32,
    #[error("Reduce only order increased risk")]
    ReduceOnlyOrderIncreasedRisk = 6064u32,
    #[error("Unable to load AccountLoader")]
    UnableToLoadAccountLoader = 6065u32,
    #[error("Trade Size Too Large")]
    TradeSizeTooLarge = 6066u32,
    #[error("User cant refer themselves")]
    UserCantReferThemselves = 6067u32,
    #[error("Did not receive expected referrer")]
    DidNotReceiveExpectedReferrer = 6068u32,
    #[error("Could not deserialize referrer")]
    CouldNotDeserializeReferrer = 6069u32,
    #[error("Could not deserialize referrer stats")]
    CouldNotDeserializeReferrerStats = 6070u32,
    #[error("User Order Id Already In Use")]
    UserOrderIdAlreadyInUse = 6071u32,
    #[error("No positions liquidatable")]
    NoPositionsLiquidatable = 6072u32,
    #[error("Invalid Margin Ratio")]
    InvalidMarginRatio = 6073u32,
    #[error("Cant Cancel Post Only Order")]
    CantCancelPostOnlyOrder = 6074u32,
    #[error("InvalidOracleOffset")]
    InvalidOracleOffset = 6075u32,
    #[error("CantExpireOrders")]
    CantExpireOrders = 6076u32,
    #[error("CouldNotLoadMarketData")]
    CouldNotLoadMarketData = 6077u32,
    #[error("PerpMarketNotFound")]
    PerpMarketNotFound = 6078u32,
    #[error("InvalidMarketAccount")]
    InvalidMarketAccount = 6079u32,
    #[error("UnableToLoadMarketAccount")]
    UnableToLoadPerpMarketAccount = 6080u32,
    #[error("MarketWrongMutability")]
    MarketWrongMutability = 6081u32,
    #[error("UnableToCastUnixTime")]
    UnableToCastUnixTime = 6082u32,
    #[error("CouldNotFindSpotPosition")]
    CouldNotFindSpotPosition = 6083u32,
    #[error("NoSpotPositionAvailable")]
    NoSpotPositionAvailable = 6084u32,
    #[error("InvalidSpotMarketInitialization")]
    InvalidSpotMarketInitialization = 6085u32,
    #[error("CouldNotLoadSpotMarketData")]
    CouldNotLoadSpotMarketData = 6086u32,
    #[error("SpotMarketNotFound")]
    SpotMarketNotFound = 6087u32,
    #[error("InvalidSpotMarketAccount")]
    InvalidSpotMarketAccount = 6088u32,
    #[error("UnableToLoadSpotMarketAccount")]
    UnableToLoadSpotMarketAccount = 6089u32,
    #[error("SpotMarketWrongMutability")]
    SpotMarketWrongMutability = 6090u32,
    #[error("SpotInterestNotUpToDate")]
    SpotMarketInterestNotUpToDate = 6091u32,
    #[error("SpotMarketInsufficientDeposits")]
    SpotMarketInsufficientDeposits = 6092u32,
    #[error("UserMustSettleTheirOwnPositiveUnsettledPNL")]
    UserMustSettleTheirOwnPositiveUnsettledPnl = 6093u32,
    #[error("CantUpdatePoolBalanceType")]
    CantUpdatePoolBalanceType = 6094u32,
    #[error("InsufficientCollateralForSettlingPNL")]
    InsufficientCollateralForSettlingPnl = 6095u32,
    #[error("AMMNotUpdatedInSameSlot")]
    AmmNotUpdatedInSameSlot = 6096u32,
    #[error("AuctionNotComplete")]
    AuctionNotComplete = 6097u32,
    #[error("MakerNotFound")]
    MakerNotFound = 6098u32,
    #[error("MakerNotFound")]
    MakerStatsNotFound = 6099u32,
    #[error("MakerMustBeWritable")]
    MakerMustBeWritable = 6100u32,
    #[error("MakerMustBeWritable")]
    MakerStatsMustBeWritable = 6101u32,
    #[error("MakerOrderNotFound")]
    MakerOrderNotFound = 6102u32,
    #[error("CouldNotDeserializeMaker")]
    CouldNotDeserializeMaker = 6103u32,
    #[error("CouldNotDeserializeMaker")]
    CouldNotDeserializeMakerStats = 6104u32,
    #[error("AuctionPriceDoesNotSatisfyMaker")]
    AuctionPriceDoesNotSatisfyMaker = 6105u32,
    #[error("MakerCantFulfillOwnOrder")]
    MakerCantFulfillOwnOrder = 6106u32,
    #[error("MakerOrderMustBePostOnly")]
    MakerOrderMustBePostOnly = 6107u32,
    #[error("CantMatchTwoPostOnlys")]
    CantMatchTwoPostOnlys = 6108u32,
    #[error("OrderBreachesOraclePriceLimits")]
    OrderBreachesOraclePriceLimits = 6109u32,
    #[error("OrderMustBeTriggeredFirst")]
    OrderMustBeTriggeredFirst = 6110u32,
    #[error("OrderNotTriggerable")]
    OrderNotTriggerable = 6111u32,
    #[error("OrderDidNotSatisfyTriggerCondition")]
    OrderDidNotSatisfyTriggerCondition = 6112u32,
    #[error("PositionAlreadyBeingLiquidated")]
    PositionAlreadyBeingLiquidated = 6113u32,
    #[error("PositionDoesntHaveOpenPositionOrOrders")]
    PositionDoesntHaveOpenPositionOrOrders = 6114u32,
    #[error("AllOrdersAreAlreadyLiquidations")]
    AllOrdersAreAlreadyLiquidations = 6115u32,
    #[error("CantCancelLiquidationOrder")]
    CantCancelLiquidationOrder = 6116u32,
    #[error("UserIsBeingLiquidated")]
    UserIsBeingLiquidated = 6117u32,
    #[error("LiquidationsOngoing")]
    LiquidationsOngoing = 6118u32,
    #[error("WrongSpotBalanceType")]
    WrongSpotBalanceType = 6119u32,
    #[error("UserCantLiquidateThemself")]
    UserCantLiquidateThemself = 6120u32,
    #[error("InvalidPerpPositionToLiquidate")]
    InvalidPerpPositionToLiquidate = 6121u32,
    #[error("InvalidBaseAssetAmountForLiquidatePerp")]
    InvalidBaseAssetAmountForLiquidatePerp = 6122u32,
    #[error("InvalidPositionLastFundingRate")]
    InvalidPositionLastFundingRate = 6123u32,
    #[error("InvalidPositionDelta")]
    InvalidPositionDelta = 6124u32,
    #[error("UserBankrupt")]
    UserBankrupt = 6125u32,
    #[error("UserNotBankrupt")]
    UserNotBankrupt = 6126u32,
    #[error("UserHasInvalidBorrow")]
    UserHasInvalidBorrow = 6127u32,
    #[error("DailyWithdrawLimit")]
    DailyWithdrawLimit = 6128u32,
    #[error("DefaultError")]
    DefaultError = 6129u32,
    #[error("Insufficient LP tokens")]
    InsufficientLpTokens = 6130u32,
    #[error("Cant LP with a market position")]
    CantLpWithPerpPosition = 6131u32,
    #[error("Unable to burn LP tokens")]
    UnableToBurnLpTokens = 6132u32,
    #[error("Trying to remove liqudity too fast after adding it")]
    TryingToRemoveLiquidityTooFast = 6133u32,
    #[error("Invalid Spot Market Vault")]
    InvalidSpotMarketVault = 6134u32,
    #[error("Invalid Spot Market State")]
    InvalidSpotMarketState = 6135u32,
    #[error("InvalidSerumProgram")]
    InvalidSerumProgram = 6136u32,
    #[error("InvalidSerumMarket")]
    InvalidSerumMarket = 6137u32,
    #[error("InvalidSerumBids")]
    InvalidSerumBids = 6138u32,
    #[error("InvalidSerumAsks")]
    InvalidSerumAsks = 6139u32,
    #[error("InvalidSerumOpenOrders")]
    InvalidSerumOpenOrders = 6140u32,
    #[error("FailedSerumCPI")]
    FailedSerumCpi = 6141u32,
    #[error("FailedToFillOnExternalMarket")]
    FailedToFillOnExternalMarket = 6142u32,
    #[error("InvalidFulfillmentConfig")]
    InvalidFulfillmentConfig = 6143u32,
    #[error("InvalidFeeStructure")]
    InvalidFeeStructure = 6144u32,
    #[error("Insufficient IF shares")]
    InsufficientIfShares = 6145u32,
    #[error("the Market has paused this action")]
    MarketActionPaused = 6146u32,
    #[error("the Market status doesnt allow placing orders")]
    MarketPlaceOrderPaused = 6147u32,
    #[error("the Market status doesnt allow filling orders")]
    MarketFillOrderPaused = 6148u32,
    #[error("the Market status doesnt allow withdraws")]
    MarketWithdrawPaused = 6149u32,
    #[error("Action violates the Protected Asset Tier rules")]
    ProtectedAssetTierViolation = 6150u32,
    #[error("Action violates the Isolated Asset Tier rules")]
    IsolatedAssetTierViolation = 6151u32,
    #[error("User Cant Be Deleted")]
    UserCantBeDeleted = 6152u32,
    #[error("Reduce Only Withdraw Increased Risk")]
    ReduceOnlyWithdrawIncreasedRisk = 6153u32,
    #[error("Max Open Interest")]
    MaxOpenInterest = 6154u32,
    #[error("Cant Resolve Perp Bankruptcy")]
    CantResolvePerpBankruptcy = 6155u32,
    #[error("Liquidation Doesnt Satisfy Limit Price")]
    LiquidationDoesntSatisfyLimitPrice = 6156u32,
    #[error("Margin Trading Disabled")]
    MarginTradingDisabled = 6157u32,
    #[error("Invalid Market Status to Settle Perp Pnl")]
    InvalidMarketStatusToSettlePnl = 6158u32,
    #[error("PerpMarketNotInSettlement")]
    PerpMarketNotInSettlement = 6159u32,
    #[error("PerpMarketNotInReduceOnly")]
    PerpMarketNotInReduceOnly = 6160u32,
    #[error("PerpMarketSettlementBufferNotReached")]
    PerpMarketSettlementBufferNotReached = 6161u32,
    #[error("PerpMarketSettlementUserHasOpenOrders")]
    PerpMarketSettlementUserHasOpenOrders = 6162u32,
    #[error("PerpMarketSettlementUserHasActiveLP")]
    PerpMarketSettlementUserHasActiveLp = 6163u32,
    #[error("UnableToSettleExpiredUserPosition")]
    UnableToSettleExpiredUserPosition = 6164u32,
    #[error("UnequalMarketIndexForSpotTransfer")]
    UnequalMarketIndexForSpotTransfer = 6165u32,
    #[error("InvalidPerpPositionDetected")]
    InvalidPerpPositionDetected = 6166u32,
    #[error("InvalidSpotPositionDetected")]
    InvalidSpotPositionDetected = 6167u32,
    #[error("InvalidAmmDetected")]
    InvalidAmmDetected = 6168u32,
    #[error("InvalidAmmForFillDetected")]
    InvalidAmmForFillDetected = 6169u32,
    #[error("InvalidAmmLimitPriceOverride")]
    InvalidAmmLimitPriceOverride = 6170u32,
    #[error("InvalidOrderFillPrice")]
    InvalidOrderFillPrice = 6171u32,
    #[error("SpotMarketBalanceInvariantViolated")]
    SpotMarketBalanceInvariantViolated = 6172u32,
    #[error("SpotMarketVaultInvariantViolated")]
    SpotMarketVaultInvariantViolated = 6173u32,
    #[error("InvalidPDA")]
    InvalidPda = 6174u32,
    #[error("InvalidPDASigner")]
    InvalidPdaSigner = 6175u32,
    #[error("RevenueSettingsCannotSettleToIF")]
    RevenueSettingsCannotSettleToIf = 6176u32,
    #[error("NoRevenueToSettleToIF")]
    NoRevenueToSettleToIf = 6177u32,
    #[error("NoAmmPerpPnlDeficit")]
    NoAmmPerpPnlDeficit = 6178u32,
    #[error("SufficientPerpPnlPool")]
    SufficientPerpPnlPool = 6179u32,
    #[error("InsufficientPerpPnlPool")]
    InsufficientPerpPnlPool = 6180u32,
    #[error("PerpPnlDeficitBelowThreshold")]
    PerpPnlDeficitBelowThreshold = 6181u32,
    #[error("MaxRevenueWithdrawPerPeriodReached")]
    MaxRevenueWithdrawPerPeriodReached = 6182u32,
    #[error("InvalidSpotPositionDetected")]
    MaxIfWithdrawReached = 6183u32,
    #[error("NoIFWithdrawAvailable")]
    NoIfWithdrawAvailable = 6184u32,
    #[error("InvalidIFUnstake")]
    InvalidIfUnstake = 6185u32,
    #[error("InvalidIFUnstakeSize")]
    InvalidIfUnstakeSize = 6186u32,
    #[error("InvalidIFUnstakeCancel")]
    InvalidIfUnstakeCancel = 6187u32,
    #[error("InvalidIFForNewStakes")]
    InvalidIfForNewStakes = 6188u32,
    #[error("InvalidIFRebase")]
    InvalidIfRebase = 6189u32,
    #[error("InvalidInsuranceUnstakeSize")]
    InvalidInsuranceUnstakeSize = 6190u32,
    #[error("InvalidOrderLimitPrice")]
    InvalidOrderLimitPrice = 6191u32,
    #[error("InvalidIFDetected")]
    InvalidIfDetected = 6192u32,
    #[error("InvalidAmmMaxSpreadDetected")]
    InvalidAmmMaxSpreadDetected = 6193u32,
    #[error("InvalidConcentrationCoef")]
    InvalidConcentrationCoef = 6194u32,
    #[error("InvalidSrmVault")]
    InvalidSrmVault = 6195u32,
    #[error("InvalidVaultOwner")]
    InvalidVaultOwner = 6196u32,
    #[error("InvalidMarketStatusForFills")]
    InvalidMarketStatusForFills = 6197u32,
    #[error("IFWithdrawRequestInProgress")]
    IfWithdrawRequestInProgress = 6198u32,
    #[error("NoIFWithdrawRequestInProgress")]
    NoIfWithdrawRequestInProgress = 6199u32,
    #[error("IFWithdrawRequestTooSmall")]
    IfWithdrawRequestTooSmall = 6200u32,
    #[error("IncorrectSpotMarketAccountPassed")]
    IncorrectSpotMarketAccountPassed = 6201u32,
    #[error("BlockchainClockInconsistency")]
    BlockchainClockInconsistency = 6202u32,
    #[error("InvalidIFSharesDetected")]
    InvalidIfSharesDetected = 6203u32,
    #[error("NewLPSizeTooSmall")]
    NewLpSizeTooSmall = 6204u32,
    #[error("MarketStatusInvalidForNewLP")]
    MarketStatusInvalidForNewLp = 6205u32,
    #[error("InvalidMarkTwapUpdateDetected")]
    InvalidMarkTwapUpdateDetected = 6206u32,
    #[error("MarketSettlementAttemptOnActiveMarket")]
    MarketSettlementAttemptOnActiveMarket = 6207u32,
    #[error("MarketSettlementRequiresSettledLP")]
    MarketSettlementRequiresSettledLp = 6208u32,
    #[error("MarketSettlementAttemptTooEarly")]
    MarketSettlementAttemptTooEarly = 6209u32,
    #[error("MarketSettlementTargetPriceInvalid")]
    MarketSettlementTargetPriceInvalid = 6210u32,
    #[error("UnsupportedSpotMarket")]
    UnsupportedSpotMarket = 6211u32,
    #[error("SpotOrdersDisabled")]
    SpotOrdersDisabled = 6212u32,
    #[error("Market Being Initialized")]
    MarketBeingInitialized = 6213u32,
    #[error("Invalid Sub Account Id")]
    InvalidUserSubAccountId = 6214u32,
    #[error("Invalid Trigger Order Condition")]
    InvalidTriggerOrderCondition = 6215u32,
    #[error("Invalid Spot Position")]
    InvalidSpotPosition = 6216u32,
    #[error("Cant transfer between same user account")]
    CantTransferBetweenSameUserAccount = 6217u32,
    #[error("Invalid Perp Position")]
    InvalidPerpPosition = 6218u32,
    #[error("Unable To Get Limit Price")]
    UnableToGetLimitPrice = 6219u32,
    #[error("Invalid Liquidation")]
    InvalidLiquidation = 6220u32,
    #[error("Spot Fulfullment Config Disabled")]
    SpotFulfillmentConfigDisabled = 6221u32,
    #[error("Invalid Maker")]
    InvalidMaker = 6222u32,
    #[error("Failed Unwrap")]
    FailedUnwrap = 6223u32,
    #[error("Max Number Of Users")]
    MaxNumberOfUsers = 6224u32,
    #[error("InvalidOracleForSettlePnl")]
    InvalidOracleForSettlePnl = 6225u32,
    #[error("MarginOrdersOpen")]
    MarginOrdersOpen = 6226u32,
    #[error("TierViolationLiquidatingPerpPnl")]
    TierViolationLiquidatingPerpPnl = 6227u32,
    #[error("CouldNotLoadUserData")]
    CouldNotLoadUserData = 6228u32,
    #[error("UserWrongMutability")]
    UserWrongMutability = 6229u32,
    #[error("InvalidUserAccount")]
    InvalidUserAccount = 6230u32,
    #[error("CouldNotLoadUserData")]
    CouldNotLoadUserStatsData = 6231u32,
    #[error("UserWrongMutability")]
    UserStatsWrongMutability = 6232u32,
    #[error("InvalidUserAccount")]
    InvalidUserStatsAccount = 6233u32,
    #[error("UserNotFound")]
    UserNotFound = 6234u32,
    #[error("UnableToLoadUserAccount")]
    UnableToLoadUserAccount = 6235u32,
    #[error("UserStatsNotFound")]
    UserStatsNotFound = 6236u32,
    #[error("UnableToLoadUserStatsAccount")]
    UnableToLoadUserStatsAccount = 6237u32,
    #[error("User Not Inactive")]
    UserNotInactive = 6238u32,
    #[error("RevertFill")]
    RevertFill = 6239u32,
    #[error("Invalid MarketAccount for Deletion")]
    InvalidMarketAccountforDeletion = 6240u32,
    #[error("Invalid Spot Fulfillment Params")]
    InvalidSpotFulfillmentParams = 6241u32,
    #[error("Failed to Get Mint")]
    FailedToGetMint = 6242u32,
    #[error("FailedPhoenixCPI")]
    FailedPhoenixCpi = 6243u32,
    #[error("FailedToDeserializePhoenixMarket")]
    FailedToDeserializePhoenixMarket = 6244u32,
    #[error("InvalidPricePrecision")]
    InvalidPricePrecision = 6245u32,
    #[error("InvalidPhoenixProgram")]
    InvalidPhoenixProgram = 6246u32,
    #[error("InvalidPhoenixMarket")]
    InvalidPhoenixMarket = 6247u32,
    #[error("InvalidSwap")]
    InvalidSwap = 6248u32,
    #[error("SwapLimitPriceBreached")]
    SwapLimitPriceBreached = 6249u32,
    #[error("SpotMarketReduceOnly")]
    SpotMarketReduceOnly = 6250u32,
    #[error("FundingWasNotUpdated")]
    FundingWasNotUpdated = 6251u32,
}
impl From<DriftError> for ProgramError {
    fn from(e: DriftError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for DriftError {
    fn type_of() -> &'static str {
        "DriftError"
    }
}
impl PrintProgramError for DriftError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
