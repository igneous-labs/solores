use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum DriftError {
    #[error("Invalid Spot Market Authority")]
    InvalidSpotMarketAuthority = 6000,
    #[error("Clearing house not insurance fund authority")]
    InvalidInsuranceFundAuthority = 6001,
    #[error("Insufficient deposit")]
    InsufficientDeposit = 6002,
    #[error("Insufficient collateral")]
    InsufficientCollateral = 6003,
    #[error("Sufficient collateral")]
    SufficientCollateral = 6004,
    #[error("Max number of positions taken")]
    MaxNumberOfPositions = 6005,
    #[error("Admin Controls Prices Disabled")]
    AdminControlsPricesDisabled = 6006,
    #[error("Market Delisted")]
    MarketDelisted = 6007,
    #[error("Market Index Already Initialized")]
    MarketIndexAlreadyInitialized = 6008,
    #[error("User Account And User Positions Account Mismatch")]
    UserAccountAndUserPositionsAccountMismatch = 6009,
    #[error("User Has No Position In Market")]
    UserHasNoPositionInMarket = 6010,
    #[error("Invalid Initial Peg")]
    InvalidInitialPeg = 6011,
    #[error("AMM repeg already configured with amt given")]
    InvalidRepegRedundant = 6012,
    #[error("AMM repeg incorrect repeg direction")]
    InvalidRepegDirection = 6013,
    #[error("AMM repeg out of bounds pnl")]
    InvalidRepegProfitability = 6014,
    #[error("Slippage Outside Limit Price")]
    SlippageOutsideLimit = 6015,
    #[error("Order Size Too Small")]
    OrderSizeTooSmall = 6016,
    #[error("Price change too large when updating K")]
    InvalidUpdateK = 6017,
    #[error("Admin tried to withdraw amount larger than fees collected")]
    AdminWithdrawTooLarge = 6018,
    #[error("Math Error")]
    MathError = 6019,
    #[error("Conversion to u128/u64 failed with an overflow or underflow")]
    BnConversionError = 6020,
    #[error("Clock unavailable")]
    ClockUnavailable = 6021,
    #[error("Unable To Load Oracles")]
    UnableToLoadOracle = 6022,
    #[error("Price Bands Breached")]
    PriceBandsBreached = 6023,
    #[error("Exchange is paused")]
    ExchangePaused = 6024,
    #[error("Invalid whitelist token")]
    InvalidWhitelistToken = 6025,
    #[error("Whitelist token not found")]
    WhitelistTokenNotFound = 6026,
    #[error("Invalid discount token")]
    InvalidDiscountToken = 6027,
    #[error("Discount token not found")]
    DiscountTokenNotFound = 6028,
    #[error("Referrer not found")]
    ReferrerNotFound = 6029,
    #[error("ReferrerNotFound")]
    ReferrerStatsNotFound = 6030,
    #[error("ReferrerMustBeWritable")]
    ReferrerMustBeWritable = 6031,
    #[error("ReferrerMustBeWritable")]
    ReferrerStatsMustBeWritable = 6032,
    #[error("ReferrerAndReferrerStatsAuthorityUnequal")]
    ReferrerAndReferrerStatsAuthorityUnequal = 6033,
    #[error("InvalidReferrer")]
    InvalidReferrer = 6034,
    #[error("InvalidOracle")]
    InvalidOracle = 6035,
    #[error("OracleNotFound")]
    OracleNotFound = 6036,
    #[error("Liquidations Blocked By Oracle")]
    LiquidationsBlockedByOracle = 6037,
    #[error("Can not deposit more than max deposit")]
    MaxDeposit = 6038,
    #[error("Can not delete user that still has collateral")]
    CantDeleteUserWithCollateral = 6039,
    #[error("AMM funding out of bounds pnl")]
    InvalidFundingProfitability = 6040,
    #[error("Casting Failure")]
    CastingFailure = 6041,
    #[error("InvalidOrder")]
    InvalidOrder = 6042,
    #[error("InvalidOrderMaxTs")]
    InvalidOrderMaxTs = 6043,
    #[error("InvalidOrderMarketType")]
    InvalidOrderMarketType = 6044,
    #[error("InvalidOrderForInitialMarginReq")]
    InvalidOrderForInitialMarginReq = 6045,
    #[error("InvalidOrderNotRiskReducing")]
    InvalidOrderNotRiskReducing = 6046,
    #[error("InvalidOrderSizeTooSmall")]
    InvalidOrderSizeTooSmall = 6047,
    #[error("InvalidOrderNotStepSizeMultiple")]
    InvalidOrderNotStepSizeMultiple = 6048,
    #[error("InvalidOrderBaseQuoteAsset")]
    InvalidOrderBaseQuoteAsset = 6049,
    #[error("InvalidOrderIOC")]
    InvalidOrderIoc = 6050,
    #[error("InvalidOrderPostOnly")]
    InvalidOrderPostOnly = 6051,
    #[error("InvalidOrderIOCPostOnly")]
    InvalidOrderIocPostOnly = 6052,
    #[error("InvalidOrderTrigger")]
    InvalidOrderTrigger = 6053,
    #[error("InvalidOrderAuction")]
    InvalidOrderAuction = 6054,
    #[error("InvalidOrderOracleOffset")]
    InvalidOrderOracleOffset = 6055,
    #[error("InvalidOrderMinOrderSize")]
    InvalidOrderMinOrderSize = 6056,
    #[error("Failed to Place Post-Only Limit Order")]
    PlacePostOnlyLimitFailure = 6057,
    #[error("User has no order")]
    UserHasNoOrder = 6058,
    #[error("Order Amount Too Small")]
    OrderAmountTooSmall = 6059,
    #[error("Max number of orders taken")]
    MaxNumberOfOrders = 6060,
    #[error("Order does not exist")]
    OrderDoesNotExist = 6061,
    #[error("Order not open")]
    OrderNotOpen = 6062,
    #[error("FillOrderDidNotUpdateState")]
    FillOrderDidNotUpdateState = 6063,
    #[error("Reduce only order increased risk")]
    ReduceOnlyOrderIncreasedRisk = 6064,
    #[error("Unable to load AccountLoader")]
    UnableToLoadAccountLoader = 6065,
    #[error("Trade Size Too Large")]
    TradeSizeTooLarge = 6066,
    #[error("User cant refer themselves")]
    UserCantReferThemselves = 6067,
    #[error("Did not receive expected referrer")]
    DidNotReceiveExpectedReferrer = 6068,
    #[error("Could not deserialize referrer")]
    CouldNotDeserializeReferrer = 6069,
    #[error("Could not deserialize referrer stats")]
    CouldNotDeserializeReferrerStats = 6070,
    #[error("User Order Id Already In Use")]
    UserOrderIdAlreadyInUse = 6071,
    #[error("No positions liquidatable")]
    NoPositionsLiquidatable = 6072,
    #[error("Invalid Margin Ratio")]
    InvalidMarginRatio = 6073,
    #[error("Cant Cancel Post Only Order")]
    CantCancelPostOnlyOrder = 6074,
    #[error("InvalidOracleOffset")]
    InvalidOracleOffset = 6075,
    #[error("CantExpireOrders")]
    CantExpireOrders = 6076,
    #[error("CouldNotLoadMarketData")]
    CouldNotLoadMarketData = 6077,
    #[error("PerpMarketNotFound")]
    PerpMarketNotFound = 6078,
    #[error("InvalidMarketAccount")]
    InvalidMarketAccount = 6079,
    #[error("UnableToLoadMarketAccount")]
    UnableToLoadPerpMarketAccount = 6080,
    #[error("MarketWrongMutability")]
    MarketWrongMutability = 6081,
    #[error("UnableToCastUnixTime")]
    UnableToCastUnixTime = 6082,
    #[error("CouldNotFindSpotPosition")]
    CouldNotFindSpotPosition = 6083,
    #[error("NoSpotPositionAvailable")]
    NoSpotPositionAvailable = 6084,
    #[error("InvalidSpotMarketInitialization")]
    InvalidSpotMarketInitialization = 6085,
    #[error("CouldNotLoadSpotMarketData")]
    CouldNotLoadSpotMarketData = 6086,
    #[error("SpotMarketNotFound")]
    SpotMarketNotFound = 6087,
    #[error("InvalidSpotMarketAccount")]
    InvalidSpotMarketAccount = 6088,
    #[error("UnableToLoadSpotMarketAccount")]
    UnableToLoadSpotMarketAccount = 6089,
    #[error("SpotMarketWrongMutability")]
    SpotMarketWrongMutability = 6090,
    #[error("SpotInterestNotUpToDate")]
    SpotMarketInterestNotUpToDate = 6091,
    #[error("SpotMarketInsufficientDeposits")]
    SpotMarketInsufficientDeposits = 6092,
    #[error("UserMustSettleTheirOwnPositiveUnsettledPNL")]
    UserMustSettleTheirOwnPositiveUnsettledPnl = 6093,
    #[error("CantUpdatePoolBalanceType")]
    CantUpdatePoolBalanceType = 6094,
    #[error("InsufficientCollateralForSettlingPNL")]
    InsufficientCollateralForSettlingPnl = 6095,
    #[error("AMMNotUpdatedInSameSlot")]
    AmmNotUpdatedInSameSlot = 6096,
    #[error("AuctionNotComplete")]
    AuctionNotComplete = 6097,
    #[error("MakerNotFound")]
    MakerNotFound = 6098,
    #[error("MakerNotFound")]
    MakerStatsNotFound = 6099,
    #[error("MakerMustBeWritable")]
    MakerMustBeWritable = 6100,
    #[error("MakerMustBeWritable")]
    MakerStatsMustBeWritable = 6101,
    #[error("MakerOrderNotFound")]
    MakerOrderNotFound = 6102,
    #[error("CouldNotDeserializeMaker")]
    CouldNotDeserializeMaker = 6103,
    #[error("CouldNotDeserializeMaker")]
    CouldNotDeserializeMakerStats = 6104,
    #[error("AuctionPriceDoesNotSatisfyMaker")]
    AuctionPriceDoesNotSatisfyMaker = 6105,
    #[error("MakerCantFulfillOwnOrder")]
    MakerCantFulfillOwnOrder = 6106,
    #[error("MakerOrderMustBePostOnly")]
    MakerOrderMustBePostOnly = 6107,
    #[error("CantMatchTwoPostOnlys")]
    CantMatchTwoPostOnlys = 6108,
    #[error("OrderBreachesOraclePriceLimits")]
    OrderBreachesOraclePriceLimits = 6109,
    #[error("OrderMustBeTriggeredFirst")]
    OrderMustBeTriggeredFirst = 6110,
    #[error("OrderNotTriggerable")]
    OrderNotTriggerable = 6111,
    #[error("OrderDidNotSatisfyTriggerCondition")]
    OrderDidNotSatisfyTriggerCondition = 6112,
    #[error("PositionAlreadyBeingLiquidated")]
    PositionAlreadyBeingLiquidated = 6113,
    #[error("PositionDoesntHaveOpenPositionOrOrders")]
    PositionDoesntHaveOpenPositionOrOrders = 6114,
    #[error("AllOrdersAreAlreadyLiquidations")]
    AllOrdersAreAlreadyLiquidations = 6115,
    #[error("CantCancelLiquidationOrder")]
    CantCancelLiquidationOrder = 6116,
    #[error("UserIsBeingLiquidated")]
    UserIsBeingLiquidated = 6117,
    #[error("LiquidationsOngoing")]
    LiquidationsOngoing = 6118,
    #[error("WrongSpotBalanceType")]
    WrongSpotBalanceType = 6119,
    #[error("UserCantLiquidateThemself")]
    UserCantLiquidateThemself = 6120,
    #[error("InvalidPerpPositionToLiquidate")]
    InvalidPerpPositionToLiquidate = 6121,
    #[error("InvalidBaseAssetAmountForLiquidatePerp")]
    InvalidBaseAssetAmountForLiquidatePerp = 6122,
    #[error("InvalidPositionLastFundingRate")]
    InvalidPositionLastFundingRate = 6123,
    #[error("InvalidPositionDelta")]
    InvalidPositionDelta = 6124,
    #[error("UserBankrupt")]
    UserBankrupt = 6125,
    #[error("UserNotBankrupt")]
    UserNotBankrupt = 6126,
    #[error("UserHasInvalidBorrow")]
    UserHasInvalidBorrow = 6127,
    #[error("DailyWithdrawLimit")]
    DailyWithdrawLimit = 6128,
    #[error("DefaultError")]
    DefaultError = 6129,
    #[error("Insufficient LP tokens")]
    InsufficientLpTokens = 6130,
    #[error("Cant LP with a market position")]
    CantLpWithPerpPosition = 6131,
    #[error("Unable to burn LP tokens")]
    UnableToBurnLpTokens = 6132,
    #[error("Trying to remove liqudity too fast after adding it")]
    TryingToRemoveLiquidityTooFast = 6133,
    #[error("Invalid Spot Market Vault")]
    InvalidSpotMarketVault = 6134,
    #[error("Invalid Spot Market State")]
    InvalidSpotMarketState = 6135,
    #[error("InvalidSerumProgram")]
    InvalidSerumProgram = 6136,
    #[error("InvalidSerumMarket")]
    InvalidSerumMarket = 6137,
    #[error("InvalidSerumBids")]
    InvalidSerumBids = 6138,
    #[error("InvalidSerumAsks")]
    InvalidSerumAsks = 6139,
    #[error("InvalidSerumOpenOrders")]
    InvalidSerumOpenOrders = 6140,
    #[error("FailedSerumCPI")]
    FailedSerumCpi = 6141,
    #[error("FailedToFillOnExternalMarket")]
    FailedToFillOnExternalMarket = 6142,
    #[error("InvalidFulfillmentConfig")]
    InvalidFulfillmentConfig = 6143,
    #[error("InvalidFeeStructure")]
    InvalidFeeStructure = 6144,
    #[error("Insufficient IF shares")]
    InsufficientIfShares = 6145,
    #[error("the Market has paused this action")]
    MarketActionPaused = 6146,
    #[error("the Market status doesnt allow placing orders")]
    MarketPlaceOrderPaused = 6147,
    #[error("the Market status doesnt allow filling orders")]
    MarketFillOrderPaused = 6148,
    #[error("the Market status doesnt allow withdraws")]
    MarketWithdrawPaused = 6149,
    #[error("Action violates the Protected Asset Tier rules")]
    ProtectedAssetTierViolation = 6150,
    #[error("Action violates the Isolated Asset Tier rules")]
    IsolatedAssetTierViolation = 6151,
    #[error("User Cant Be Deleted")]
    UserCantBeDeleted = 6152,
    #[error("Reduce Only Withdraw Increased Risk")]
    ReduceOnlyWithdrawIncreasedRisk = 6153,
    #[error("Max Open Interest")]
    MaxOpenInterest = 6154,
    #[error("Cant Resolve Perp Bankruptcy")]
    CantResolvePerpBankruptcy = 6155,
    #[error("Liquidation Doesnt Satisfy Limit Price")]
    LiquidationDoesntSatisfyLimitPrice = 6156,
    #[error("Margin Trading Disabled")]
    MarginTradingDisabled = 6157,
    #[error("Invalid Market Status to Settle Perp Pnl")]
    InvalidMarketStatusToSettlePnl = 6158,
    #[error("PerpMarketNotInSettlement")]
    PerpMarketNotInSettlement = 6159,
    #[error("PerpMarketNotInReduceOnly")]
    PerpMarketNotInReduceOnly = 6160,
    #[error("PerpMarketSettlementBufferNotReached")]
    PerpMarketSettlementBufferNotReached = 6161,
    #[error("PerpMarketSettlementUserHasOpenOrders")]
    PerpMarketSettlementUserHasOpenOrders = 6162,
    #[error("PerpMarketSettlementUserHasActiveLP")]
    PerpMarketSettlementUserHasActiveLp = 6163,
    #[error("UnableToSettleExpiredUserPosition")]
    UnableToSettleExpiredUserPosition = 6164,
    #[error("UnequalMarketIndexForSpotTransfer")]
    UnequalMarketIndexForSpotTransfer = 6165,
    #[error("InvalidPerpPositionDetected")]
    InvalidPerpPositionDetected = 6166,
    #[error("InvalidSpotPositionDetected")]
    InvalidSpotPositionDetected = 6167,
    #[error("InvalidAmmDetected")]
    InvalidAmmDetected = 6168,
    #[error("InvalidAmmForFillDetected")]
    InvalidAmmForFillDetected = 6169,
    #[error("InvalidAmmLimitPriceOverride")]
    InvalidAmmLimitPriceOverride = 6170,
    #[error("InvalidOrderFillPrice")]
    InvalidOrderFillPrice = 6171,
    #[error("SpotMarketBalanceInvariantViolated")]
    SpotMarketBalanceInvariantViolated = 6172,
    #[error("SpotMarketVaultInvariantViolated")]
    SpotMarketVaultInvariantViolated = 6173,
    #[error("InvalidPDA")]
    InvalidPda = 6174,
    #[error("InvalidPDASigner")]
    InvalidPdaSigner = 6175,
    #[error("RevenueSettingsCannotSettleToIF")]
    RevenueSettingsCannotSettleToIf = 6176,
    #[error("NoRevenueToSettleToIF")]
    NoRevenueToSettleToIf = 6177,
    #[error("NoAmmPerpPnlDeficit")]
    NoAmmPerpPnlDeficit = 6178,
    #[error("SufficientPerpPnlPool")]
    SufficientPerpPnlPool = 6179,
    #[error("InsufficientPerpPnlPool")]
    InsufficientPerpPnlPool = 6180,
    #[error("PerpPnlDeficitBelowThreshold")]
    PerpPnlDeficitBelowThreshold = 6181,
    #[error("MaxRevenueWithdrawPerPeriodReached")]
    MaxRevenueWithdrawPerPeriodReached = 6182,
    #[error("InvalidSpotPositionDetected")]
    MaxIfWithdrawReached = 6183,
    #[error("NoIFWithdrawAvailable")]
    NoIfWithdrawAvailable = 6184,
    #[error("InvalidIFUnstake")]
    InvalidIfUnstake = 6185,
    #[error("InvalidIFUnstakeSize")]
    InvalidIfUnstakeSize = 6186,
    #[error("InvalidIFUnstakeCancel")]
    InvalidIfUnstakeCancel = 6187,
    #[error("InvalidIFForNewStakes")]
    InvalidIfForNewStakes = 6188,
    #[error("InvalidIFRebase")]
    InvalidIfRebase = 6189,
    #[error("InvalidInsuranceUnstakeSize")]
    InvalidInsuranceUnstakeSize = 6190,
    #[error("InvalidOrderLimitPrice")]
    InvalidOrderLimitPrice = 6191,
    #[error("InvalidIFDetected")]
    InvalidIfDetected = 6192,
    #[error("InvalidAmmMaxSpreadDetected")]
    InvalidAmmMaxSpreadDetected = 6193,
    #[error("InvalidConcentrationCoef")]
    InvalidConcentrationCoef = 6194,
    #[error("InvalidSrmVault")]
    InvalidSrmVault = 6195,
    #[error("InvalidVaultOwner")]
    InvalidVaultOwner = 6196,
    #[error("InvalidMarketStatusForFills")]
    InvalidMarketStatusForFills = 6197,
    #[error("IFWithdrawRequestInProgress")]
    IfWithdrawRequestInProgress = 6198,
    #[error("NoIFWithdrawRequestInProgress")]
    NoIfWithdrawRequestInProgress = 6199,
    #[error("IFWithdrawRequestTooSmall")]
    IfWithdrawRequestTooSmall = 6200,
    #[error("IncorrectSpotMarketAccountPassed")]
    IncorrectSpotMarketAccountPassed = 6201,
    #[error("BlockchainClockInconsistency")]
    BlockchainClockInconsistency = 6202,
    #[error("InvalidIFSharesDetected")]
    InvalidIfSharesDetected = 6203,
    #[error("NewLPSizeTooSmall")]
    NewLpSizeTooSmall = 6204,
    #[error("MarketStatusInvalidForNewLP")]
    MarketStatusInvalidForNewLp = 6205,
    #[error("InvalidMarkTwapUpdateDetected")]
    InvalidMarkTwapUpdateDetected = 6206,
    #[error("MarketSettlementAttemptOnActiveMarket")]
    MarketSettlementAttemptOnActiveMarket = 6207,
    #[error("MarketSettlementRequiresSettledLP")]
    MarketSettlementRequiresSettledLp = 6208,
    #[error("MarketSettlementAttemptTooEarly")]
    MarketSettlementAttemptTooEarly = 6209,
    #[error("MarketSettlementTargetPriceInvalid")]
    MarketSettlementTargetPriceInvalid = 6210,
    #[error("UnsupportedSpotMarket")]
    UnsupportedSpotMarket = 6211,
    #[error("SpotOrdersDisabled")]
    SpotOrdersDisabled = 6212,
    #[error("Market Being Initialized")]
    MarketBeingInitialized = 6213,
    #[error("Invalid Sub Account Id")]
    InvalidUserSubAccountId = 6214,
    #[error("Invalid Trigger Order Condition")]
    InvalidTriggerOrderCondition = 6215,
    #[error("Invalid Spot Position")]
    InvalidSpotPosition = 6216,
    #[error("Cant transfer between same user account")]
    CantTransferBetweenSameUserAccount = 6217,
    #[error("Invalid Perp Position")]
    InvalidPerpPosition = 6218,
    #[error("Unable To Get Limit Price")]
    UnableToGetLimitPrice = 6219,
    #[error("Invalid Liquidation")]
    InvalidLiquidation = 6220,
    #[error("Spot Fulfullment Config Disabled")]
    SpotFulfillmentConfigDisabled = 6221,
    #[error("Invalid Maker")]
    InvalidMaker = 6222,
    #[error("Failed Unwrap")]
    FailedUnwrap = 6223,
    #[error("Max Number Of Users")]
    MaxNumberOfUsers = 6224,
    #[error("InvalidOracleForSettlePnl")]
    InvalidOracleForSettlePnl = 6225,
    #[error("MarginOrdersOpen")]
    MarginOrdersOpen = 6226,
    #[error("TierViolationLiquidatingPerpPnl")]
    TierViolationLiquidatingPerpPnl = 6227,
    #[error("CouldNotLoadUserData")]
    CouldNotLoadUserData = 6228,
    #[error("UserWrongMutability")]
    UserWrongMutability = 6229,
    #[error("InvalidUserAccount")]
    InvalidUserAccount = 6230,
    #[error("CouldNotLoadUserData")]
    CouldNotLoadUserStatsData = 6231,
    #[error("UserWrongMutability")]
    UserStatsWrongMutability = 6232,
    #[error("InvalidUserAccount")]
    InvalidUserStatsAccount = 6233,
    #[error("UserNotFound")]
    UserNotFound = 6234,
    #[error("UnableToLoadUserAccount")]
    UnableToLoadUserAccount = 6235,
    #[error("UserStatsNotFound")]
    UserStatsNotFound = 6236,
    #[error("UnableToLoadUserStatsAccount")]
    UnableToLoadUserStatsAccount = 6237,
    #[error("User Not Inactive")]
    UserNotInactive = 6238,
    #[error("RevertFill")]
    RevertFill = 6239,
    #[error("Invalid MarketAccount for Deletion")]
    InvalidMarketAccountforDeletion = 6240,
    #[error("Invalid Spot Fulfillment Params")]
    InvalidSpotFulfillmentParams = 6241,
    #[error("Failed to Get Mint")]
    FailedToGetMint = 6242,
    #[error("FailedPhoenixCPI")]
    FailedPhoenixCpi = 6243,
    #[error("FailedToDeserializePhoenixMarket")]
    FailedToDeserializePhoenixMarket = 6244,
    #[error("InvalidPricePrecision")]
    InvalidPricePrecision = 6245,
    #[error("InvalidPhoenixProgram")]
    InvalidPhoenixProgram = 6246,
    #[error("InvalidPhoenixMarket")]
    InvalidPhoenixMarket = 6247,
    #[error("InvalidSwap")]
    InvalidSwap = 6248,
    #[error("SwapLimitPriceBreached")]
    SwapLimitPriceBreached = 6249,
    #[error("SpotMarketReduceOnly")]
    SpotMarketReduceOnly = 6250,
    #[error("FundingWasNotUpdated")]
    FundingWasNotUpdated = 6251,
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
