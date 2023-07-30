use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum PhoenixV1Error {
    #[error("Invalid market parameters error")]
    InvalidMarketParameters = 0,
    #[error("Invalid market authority error")]
    InvalidMarketAuthority = 1,
    #[error("Market deserialization error")]
    FailedToLoadMarketFromAccount = 2,
    #[error("Market already initialized error")]
    MarketAlreadyInitialized = 3,
    #[error("Market is not initialized error")]
    MarketUninitialized = 4,
    #[error("Invalid state transition error")]
    InvalidStateTransition = 5,
    #[error("Invalid market signer error")]
    InvalidMarketSigner = 6,
    #[error("Invalid lot size error")]
    InvalidLotSize = 7,
    #[error("Invalid tick size error")]
    InvalidTickSize = 8,
    #[error("Invalid mint error")]
    InvalidMint = 9,
    #[error("Invalid base vault error")]
    InvalidBaseVault = 10,
    #[error("Invalid quote vault error")]
    InvalidQuoteVault = 11,
    #[error("Invalid base account error")]
    InvalidBaseAccount = 12,
    #[error("Invalid quote account error")]
    InvalidQuoteAccount = 13,
    #[error("Too many events error")]
    TooManyEvents = 14,
    #[error("New order error")]
    NewOrderError = 15,
    #[error("Reduce order error")]
    ReduceOrderError = 16,
    #[error("Cancel multiple orders error")]
    CancelMultipleOrdersError = 17,
    #[error("Withdraw funds error")]
    WithdrawFundsError = 18,
    #[error("Remove empty orders error")]
    RemoveEmptyOrdersError = 19,
    #[error("Trader not found error")]
    TraderNotFound = 20,
    #[error("Invalid seat status")]
    InvalidSeatStatus = 21,
    #[error("Failed to evict trader")]
    EvictionError = 22,
    #[error("Non empty scratch buffer")]
    NonEmptyScratchBuffer = 23,
    #[error("Failed to serialize event")]
    FailedToSerializeEvent = 24,
    #[error("Failed to flush buffer")]
    FailedToFlushBuffer = 25,
}
impl From<PhoenixV1Error> for ProgramError {
    fn from(e: PhoenixV1Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for PhoenixV1Error {
    fn type_of() -> &'static str {
        "PhoenixV1Error"
    }
}
impl PrintProgramError for PhoenixV1Error {
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
