use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum UnstakeError {
    #[error("The provided LP token account is invalid")]
    InvalidLpTokenAccount = 6000u32,
    #[error("Could not find PDA bump")]
    PdaBumpNotCached = 6001u32,
    #[error(
        "The provided fee authority does not have the authority over the provided pool account"
    )]
    InvalidFeeAuthority = 6002u32,
    #[error(
        "The Authorized of the given stake account is None (possibly an uninitialized stake account was given)"
    )]
    StakeAccountAuthorizedNotRetrievable = 6003u32,
    #[error(
        "The Lockup of the given stake account is None (possibly an uninitialized stake account was given)"
    )]
    StakeAccountLockupNotRetrievable = 6004u32,
    #[error("The provided stake account is locked up")]
    StakeAccountLockupInForce = 6005u32,
    #[error("The provided description of fee violates the invariants")]
    InvalidFee = 6006u32,
    #[error("Internal Error")]
    InternalError = 6007u32,
    #[error("Not enough liquidity to service this unstake")]
    NotEnoughLiquidity = 6008u32,
    #[error("Liquidity to add too little")]
    LiquidityToAddTooLittle = 6009u32,
    #[error("Destination token account is not a wrapped SOL account")]
    DestinationNotWSol = 6010u32,
    #[error("Wrong protocol fee destination account")]
    WrongProtocolFeeDestination = 6011u32,
    #[error(
        "The provided protocol fee authority does not have the authority over the protocol fee account"
    )]
    InvalidProtocolFeeAuthority = 6012u32,
}
impl From<UnstakeError> for ProgramError {
    fn from(e: UnstakeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for UnstakeError {
    fn type_of() -> &'static str {
        "UnstakeError"
    }
}
impl PrintProgramError for UnstakeError {
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
