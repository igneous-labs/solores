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
pub enum PhoenixV1ProgramIx {
    Swap(SwapIxArgs),
    SwapWithFreeFunds(SwapWithFreeFundsIxArgs),
    PlaceLimitOrder(PlaceLimitOrderIxArgs),
    PlaceLimitOrderWithFreeFunds(PlaceLimitOrderWithFreeFundsIxArgs),
    ReduceOrder(ReduceOrderIxArgs),
    ReduceOrderWithFreeFunds(ReduceOrderWithFreeFundsIxArgs),
    CancelAllOrders(CancelAllOrdersIxArgs),
    CancelAllOrdersWithFreeFunds(CancelAllOrdersWithFreeFundsIxArgs),
    CancelUpTo(CancelUpToIxArgs),
    CancelUpToWithFreeFunds(CancelUpToWithFreeFundsIxArgs),
    CancelMultipleOrdersById(CancelMultipleOrdersByIdIxArgs),
    CancelMultipleOrdersByIdWithFreeFunds(CancelMultipleOrdersByIdWithFreeFundsIxArgs),
    WithdrawFunds(WithdrawFundsIxArgs),
    DepositFunds(DepositFundsIxArgs),
    RequestSeat(RequestSeatIxArgs),
    Log(LogIxArgs),
    PlaceMultiplePostOnlyOrders(PlaceMultiplePostOnlyOrdersIxArgs),
    PlaceMultiplePostOnlyOrdersWithFreeFunds(PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs),
    InitializeMarket(InitializeMarketIxArgs),
    ClaimAuthority(ClaimAuthorityIxArgs),
    NameSuccessor(NameSuccessorIxArgs),
    ChangeMarketStatus(ChangeMarketStatusIxArgs),
    ChangeSeatStatus(ChangeSeatStatusIxArgs),
    RequestSeatAuthorized(RequestSeatAuthorizedIxArgs),
    EvictSeat(EvictSeatIxArgs),
    ForceCancelOrders(ForceCancelOrdersIxArgs),
    CollectFees(CollectFeesIxArgs),
    ChangeFeeRecipient(ChangeFeeRecipientIxArgs),
}
impl BorshSerialize for PhoenixV1ProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::Swap(args) => {
                SWAP_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SwapWithFreeFunds(args) => {
                SWAP_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PlaceLimitOrder(args) => {
                PLACE_LIMIT_ORDER_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PlaceLimitOrderWithFreeFunds(args) => {
                PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ReduceOrder(args) => {
                REDUCE_ORDER_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ReduceOrderWithFreeFunds(args) => {
                REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CancelAllOrders(args) => {
                CANCEL_ALL_ORDERS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CancelAllOrdersWithFreeFunds(args) => {
                CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CancelUpTo(args) => {
                CANCEL_UP_TO_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CancelUpToWithFreeFunds(args) => {
                CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CancelMultipleOrdersById(args) => {
                CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CancelMultipleOrdersByIdWithFreeFunds(args) => {
                CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::WithdrawFunds(args) => {
                WITHDRAW_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::DepositFunds(args) => {
                DEPOSIT_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RequestSeat(args) => {
                REQUEST_SEAT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::Log(args) => {
                LOG_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PlaceMultiplePostOnlyOrders(args) => {
                PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::PlaceMultiplePostOnlyOrdersWithFreeFunds(args) => {
                PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::InitializeMarket(args) => {
                INITIALIZE_MARKET_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ClaimAuthority(args) => {
                CLAIM_AUTHORITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::NameSuccessor(args) => {
                NAME_SUCCESSOR_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ChangeMarketStatus(args) => {
                CHANGE_MARKET_STATUS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ChangeSeatStatus(args) => {
                CHANGE_SEAT_STATUS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RequestSeatAuthorized(args) => {
                REQUEST_SEAT_AUTHORIZED_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::EvictSeat(args) => {
                EVICT_SEAT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ForceCancelOrders(args) => {
                FORCE_CANCEL_ORDERS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::CollectFees(args) => {
                COLLECT_FEES_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ChangeFeeRecipient(args) => {
                CHANGE_FEE_RECIPIENT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl PhoenixV1ProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(buf)?)),
            SWAP_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::SwapWithFreeFunds(
                SwapWithFreeFundsIxArgs::deserialize(buf)?,
            )),
            PLACE_LIMIT_ORDER_IX_DISCM => Ok(Self::PlaceLimitOrder(
                PlaceLimitOrderIxArgs::deserialize(buf)?,
            )),
            PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::PlaceLimitOrderWithFreeFunds(
                PlaceLimitOrderWithFreeFundsIxArgs::deserialize(buf)?,
            )),
            REDUCE_ORDER_IX_DISCM => Ok(Self::ReduceOrder(ReduceOrderIxArgs::deserialize(buf)?)),
            REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::ReduceOrderWithFreeFunds(
                ReduceOrderWithFreeFundsIxArgs::deserialize(buf)?,
            )),
            CANCEL_ALL_ORDERS_IX_DISCM => Ok(Self::CancelAllOrders(
                CancelAllOrdersIxArgs::deserialize(buf)?,
            )),
            CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::CancelAllOrdersWithFreeFunds(
                CancelAllOrdersWithFreeFundsIxArgs::deserialize(buf)?,
            )),
            CANCEL_UP_TO_IX_DISCM => Ok(Self::CancelUpTo(CancelUpToIxArgs::deserialize(buf)?)),
            CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::CancelUpToWithFreeFunds(
                CancelUpToWithFreeFundsIxArgs::deserialize(buf)?,
            )),
            CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM => Ok(Self::CancelMultipleOrdersById(
                CancelMultipleOrdersByIdIxArgs::deserialize(buf)?,
            )),
            CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM => {
                Ok(Self::CancelMultipleOrdersByIdWithFreeFunds(
                    CancelMultipleOrdersByIdWithFreeFundsIxArgs::deserialize(buf)?,
                ))
            }
            WITHDRAW_FUNDS_IX_DISCM => {
                Ok(Self::WithdrawFunds(WithdrawFundsIxArgs::deserialize(buf)?))
            }
            DEPOSIT_FUNDS_IX_DISCM => Ok(Self::DepositFunds(DepositFundsIxArgs::deserialize(buf)?)),
            REQUEST_SEAT_IX_DISCM => Ok(Self::RequestSeat(RequestSeatIxArgs::deserialize(buf)?)),
            LOG_IX_DISCM => Ok(Self::Log(LogIxArgs::deserialize(buf)?)),
            PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM => Ok(Self::PlaceMultiplePostOnlyOrders(
                PlaceMultiplePostOnlyOrdersIxArgs::deserialize(buf)?,
            )),
            PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM => {
                Ok(Self::PlaceMultiplePostOnlyOrdersWithFreeFunds(
                    PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs::deserialize(buf)?,
                ))
            }
            INITIALIZE_MARKET_IX_DISCM => Ok(Self::InitializeMarket(
                InitializeMarketIxArgs::deserialize(buf)?,
            )),
            CLAIM_AUTHORITY_IX_DISCM => Ok(Self::ClaimAuthority(
                ClaimAuthorityIxArgs::deserialize(buf)?,
            )),
            NAME_SUCCESSOR_IX_DISCM => {
                Ok(Self::NameSuccessor(NameSuccessorIxArgs::deserialize(buf)?))
            }
            CHANGE_MARKET_STATUS_IX_DISCM => Ok(Self::ChangeMarketStatus(
                ChangeMarketStatusIxArgs::deserialize(buf)?,
            )),
            CHANGE_SEAT_STATUS_IX_DISCM => Ok(Self::ChangeSeatStatus(
                ChangeSeatStatusIxArgs::deserialize(buf)?,
            )),
            REQUEST_SEAT_AUTHORIZED_IX_DISCM => Ok(Self::RequestSeatAuthorized(
                RequestSeatAuthorizedIxArgs::deserialize(buf)?,
            )),
            EVICT_SEAT_IX_DISCM => Ok(Self::EvictSeat(EvictSeatIxArgs::deserialize(buf)?)),
            FORCE_CANCEL_ORDERS_IX_DISCM => Ok(Self::ForceCancelOrders(
                ForceCancelOrdersIxArgs::deserialize(buf)?,
            )),
            COLLECT_FEES_IX_DISCM => Ok(Self::CollectFees(CollectFeesIxArgs::deserialize(buf)?)),
            CHANGE_FEE_RECIPIENT_IX_DISCM => Ok(Self::ChangeFeeRecipient(
                ChangeFeeRecipientIxArgs::deserialize(buf)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const SWAP_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SwapAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: &SwapAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: &SwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; SWAP_IX_ACCOUNTS_LEN]> for SwapKeys {
    fn from(pubkeys: [Pubkey; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&SwapAccounts<'_, 'info>> for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: &SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]>
    for SwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
pub const SWAP_IX_DISCM: u8 = 0u8;
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapIxArgs::deserialize(buf)?))
    }
}
pub fn swap_ix<K: Into<SwapKeys>, A: Into<SwapIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapKeys = accounts.into();
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SwapIxArgs = args.into();
    let data: SwapIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_invoke<'info, A: Into<SwapIxArgs>>(
    accounts: &SwapAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_invoke_signed<'info, A: Into<SwapIxArgs>>(
    accounts: &SwapAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn swap_verify_account_keys(
    accounts: &SwapAccounts<'_, '_>,
    keys: &SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges(accounts: &SwapAccounts<'_, '_>) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SwapWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SwapWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
}
impl From<&SwapWithFreeFundsAccounts<'_, '_>> for SwapWithFreeFundsKeys {
    fn from(accounts: &SwapWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<&SwapWithFreeFundsKeys> for [AccountMeta; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: &SwapWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new_readonly(keys.seat, false),
        ]
    }
}
impl From<[Pubkey; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]> for SwapWithFreeFundsKeys {
    fn from(pubkeys: [Pubkey; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            seat: pubkeys[4],
        }
    }
}
impl<'info> From<&SwapWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SwapWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for SwapWithFreeFundsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            seat: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapWithFreeFundsIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithFreeFundsIxData(pub SwapWithFreeFundsIxArgs);
pub const SWAP_WITH_FREE_FUNDS_IX_DISCM: u8 = 1u8;
impl From<SwapWithFreeFundsIxArgs> for SwapWithFreeFundsIxData {
    fn from(args: SwapWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SwapWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl SwapWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SWAP_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapWithFreeFundsIxArgs::deserialize(buf)?))
    }
}
pub fn swap_with_free_funds_ix<K: Into<SwapWithFreeFundsKeys>, A: Into<SwapWithFreeFundsIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SwapWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SwapWithFreeFundsIxArgs = args.into();
    let data: SwapWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_free_funds_invoke<'info, A: Into<SwapWithFreeFundsIxArgs>>(
    accounts: &SwapWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = swap_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn swap_with_free_funds_invoke_signed<'info, A: Into<SwapWithFreeFundsIxArgs>>(
    accounts: &SwapWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = swap_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn swap_with_free_funds_verify_account_keys(
    accounts: &SwapWithFreeFundsAccounts<'_, '_>,
    keys: &SwapWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn swap_with_free_funds_verify_account_privileges(
    accounts: &SwapWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct PlaceLimitOrderAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceLimitOrderKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&PlaceLimitOrderAccounts<'_, '_>> for PlaceLimitOrderKeys {
    fn from(accounts: &PlaceLimitOrderAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&PlaceLimitOrderKeys> for [AccountMeta; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceLimitOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new_readonly(keys.seat, false),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]> for PlaceLimitOrderKeys {
    fn from(pubkeys: [Pubkey; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            seat: pubkeys[4],
            base_account: pubkeys[5],
            quote_account: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&PlaceLimitOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceLimitOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]>
    for PlaceLimitOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            seat: &arr[4],
            base_account: &arr[5],
            quote_account: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceLimitOrderIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderIxData(pub PlaceLimitOrderIxArgs);
pub const PLACE_LIMIT_ORDER_IX_DISCM: u8 = 2u8;
impl From<PlaceLimitOrderIxArgs> for PlaceLimitOrderIxData {
    fn from(args: PlaceLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceLimitOrderIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_LIMIT_ORDER_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PlaceLimitOrderIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PLACE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_LIMIT_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceLimitOrderIxArgs::deserialize(buf)?))
    }
}
pub fn place_limit_order_ix<K: Into<PlaceLimitOrderKeys>, A: Into<PlaceLimitOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceLimitOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceLimitOrderIxArgs = args.into();
    let data: PlaceLimitOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_limit_order_invoke<'info, A: Into<PlaceLimitOrderIxArgs>>(
    accounts: &PlaceLimitOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_limit_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_limit_order_invoke_signed<'info, A: Into<PlaceLimitOrderIxArgs>>(
    accounts: &PlaceLimitOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_limit_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_limit_order_verify_account_keys(
    accounts: &PlaceLimitOrderAccounts<'_, '_>,
    keys: &PlaceLimitOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_limit_order_verify_account_privileges(
    accounts: &PlaceLimitOrderAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PlaceLimitOrderWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceLimitOrderWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
}
impl From<&PlaceLimitOrderWithFreeFundsAccounts<'_, '_>> for PlaceLimitOrderWithFreeFundsKeys {
    fn from(accounts: &PlaceLimitOrderWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<&PlaceLimitOrderWithFreeFundsKeys>
    for [AccountMeta; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &PlaceLimitOrderWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new_readonly(keys.seat, false),
        ]
    }
}
impl From<[Pubkey; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for PlaceLimitOrderWithFreeFundsKeys
{
    fn from(pubkeys: [Pubkey; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            seat: pubkeys[4],
        }
    }
}
impl<'info> From<&PlaceLimitOrderWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceLimitOrderWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for PlaceLimitOrderWithFreeFundsAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            seat: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceLimitOrderWithFreeFundsIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderWithFreeFundsIxData(pub PlaceLimitOrderWithFreeFundsIxArgs);
pub const PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM: u8 = 3u8;
impl From<PlaceLimitOrderWithFreeFundsIxArgs> for PlaceLimitOrderWithFreeFundsIxData {
    fn from(args: PlaceLimitOrderWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceLimitOrderWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PlaceLimitOrderWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceLimitOrderWithFreeFundsIxArgs::deserialize(buf)?))
    }
}
pub fn place_limit_order_with_free_funds_ix<
    K: Into<PlaceLimitOrderWithFreeFundsKeys>,
    A: Into<PlaceLimitOrderWithFreeFundsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceLimitOrderWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceLimitOrderWithFreeFundsIxArgs = args.into();
    let data: PlaceLimitOrderWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_limit_order_with_free_funds_invoke<
    'info,
    A: Into<PlaceLimitOrderWithFreeFundsIxArgs>,
>(
    accounts: &PlaceLimitOrderWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_limit_order_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_limit_order_with_free_funds_invoke_signed<
    'info,
    A: Into<PlaceLimitOrderWithFreeFundsIxArgs>,
>(
    accounts: &PlaceLimitOrderWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_limit_order_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_limit_order_with_free_funds_verify_account_keys(
    accounts: &PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
    keys: &PlaceLimitOrderWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_limit_order_with_free_funds_verify_account_privileges(
    accounts: &PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const REDUCE_ORDER_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ReduceOrderAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ReduceOrderKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&ReduceOrderAccounts<'_, '_>> for ReduceOrderKeys {
    fn from(accounts: &ReduceOrderAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ReduceOrderKeys> for [AccountMeta; REDUCE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &ReduceOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; REDUCE_ORDER_IX_ACCOUNTS_LEN]> for ReduceOrderKeys {
    fn from(pubkeys: [Pubkey; REDUCE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&ReduceOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDUCE_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ReduceOrderAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDUCE_ORDER_IX_ACCOUNTS_LEN]>
    for ReduceOrderAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDUCE_ORDER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReduceOrderIxArgs {
    pub params: ReduceOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReduceOrderIxData(pub ReduceOrderIxArgs);
pub const REDUCE_ORDER_IX_DISCM: u8 = 4u8;
impl From<ReduceOrderIxArgs> for ReduceOrderIxData {
    fn from(args: ReduceOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ReduceOrderIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REDUCE_ORDER_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ReduceOrderIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REDUCE_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDUCE_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReduceOrderIxArgs::deserialize(buf)?))
    }
}
pub fn reduce_order_ix<K: Into<ReduceOrderKeys>, A: Into<ReduceOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ReduceOrderKeys = accounts.into();
    let metas: [AccountMeta; REDUCE_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ReduceOrderIxArgs = args.into();
    let data: ReduceOrderIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reduce_order_invoke<'info, A: Into<ReduceOrderIxArgs>>(
    accounts: &ReduceOrderAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = reduce_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REDUCE_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn reduce_order_invoke_signed<'info, A: Into<ReduceOrderIxArgs>>(
    accounts: &ReduceOrderAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = reduce_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REDUCE_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn reduce_order_verify_account_keys(
    accounts: &ReduceOrderAccounts<'_, '_>,
    keys: &ReduceOrderKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn reduce_order_verify_account_privileges(
    accounts: &ReduceOrderAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ReduceOrderWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ReduceOrderWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
}
impl From<&ReduceOrderWithFreeFundsAccounts<'_, '_>> for ReduceOrderWithFreeFundsKeys {
    fn from(accounts: &ReduceOrderWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<&ReduceOrderWithFreeFundsKeys>
    for [AccountMeta; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &ReduceOrderWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new(keys.trader, true),
        ]
    }
}
impl From<[Pubkey; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]> for ReduceOrderWithFreeFundsKeys {
    fn from(pubkeys: [Pubkey; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
        }
    }
}
impl<'info> From<&ReduceOrderWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ReduceOrderWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for ReduceOrderWithFreeFundsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReduceOrderWithFreeFundsIxArgs {
    pub params: ReduceOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReduceOrderWithFreeFundsIxData(pub ReduceOrderWithFreeFundsIxArgs);
pub const REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM: u8 = 5u8;
impl From<ReduceOrderWithFreeFundsIxArgs> for ReduceOrderWithFreeFundsIxData {
    fn from(args: ReduceOrderWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ReduceOrderWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ReduceOrderWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReduceOrderWithFreeFundsIxArgs::deserialize(buf)?))
    }
}
pub fn reduce_order_with_free_funds_ix<
    K: Into<ReduceOrderWithFreeFundsKeys>,
    A: Into<ReduceOrderWithFreeFundsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ReduceOrderWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ReduceOrderWithFreeFundsIxArgs = args.into();
    let data: ReduceOrderWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reduce_order_with_free_funds_invoke<'info, A: Into<ReduceOrderWithFreeFundsIxArgs>>(
    accounts: &ReduceOrderWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = reduce_order_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn reduce_order_with_free_funds_invoke_signed<
    'info,
    A: Into<ReduceOrderWithFreeFundsIxArgs>,
>(
    accounts: &ReduceOrderWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = reduce_order_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn reduce_order_with_free_funds_verify_account_keys(
    accounts: &ReduceOrderWithFreeFundsAccounts<'_, '_>,
    keys: &ReduceOrderWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn reduce_order_with_free_funds_verify_account_privileges(
    accounts: &ReduceOrderWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market, accounts.trader] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CancelAllOrdersAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelAllOrdersKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&CancelAllOrdersAccounts<'_, '_>> for CancelAllOrdersKeys {
    fn from(accounts: &CancelAllOrdersAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CancelAllOrdersKeys> for [AccountMeta; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelAllOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]> for CancelAllOrdersKeys {
    fn from(pubkeys: [Pubkey; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&CancelAllOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelAllOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]>
    for CancelAllOrdersAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelAllOrdersIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAllOrdersIxData(pub CancelAllOrdersIxArgs);
pub const CANCEL_ALL_ORDERS_IX_DISCM: u8 = 6u8;
impl From<CancelAllOrdersIxArgs> for CancelAllOrdersIxData {
    fn from(args: CancelAllOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelAllOrdersIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_ALL_ORDERS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CancelAllOrdersIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CANCEL_ALL_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ALL_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelAllOrdersIxArgs::deserialize(buf)?))
    }
}
pub fn cancel_all_orders_ix<K: Into<CancelAllOrdersKeys>, A: Into<CancelAllOrdersIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelAllOrdersKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelAllOrdersIxArgs = args.into();
    let data: CancelAllOrdersIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_all_orders_invoke<'info, A: Into<CancelAllOrdersIxArgs>>(
    accounts: &CancelAllOrdersAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_all_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_all_orders_invoke_signed<'info, A: Into<CancelAllOrdersIxArgs>>(
    accounts: &CancelAllOrdersAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_all_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_all_orders_verify_account_keys(
    accounts: &CancelAllOrdersAccounts<'_, '_>,
    keys: &CancelAllOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_verify_account_privileges(
    accounts: &CancelAllOrdersAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CancelAllOrdersWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelAllOrdersWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
}
impl From<&CancelAllOrdersWithFreeFundsAccounts<'_, '_>> for CancelAllOrdersWithFreeFundsKeys {
    fn from(accounts: &CancelAllOrdersWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<&CancelAllOrdersWithFreeFundsKeys>
    for [AccountMeta; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CancelAllOrdersWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
        ]
    }
}
impl From<[Pubkey; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for CancelAllOrdersWithFreeFundsKeys
{
    fn from(pubkeys: [Pubkey; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
        }
    }
}
impl<'info> From<&CancelAllOrdersWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelAllOrdersWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for CancelAllOrdersWithFreeFundsAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelAllOrdersWithFreeFundsIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAllOrdersWithFreeFundsIxData(pub CancelAllOrdersWithFreeFundsIxArgs);
pub const CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM: u8 = 7u8;
impl From<CancelAllOrdersWithFreeFundsIxArgs> for CancelAllOrdersWithFreeFundsIxData {
    fn from(args: CancelAllOrdersWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelAllOrdersWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CancelAllOrdersWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelAllOrdersWithFreeFundsIxArgs::deserialize(buf)?))
    }
}
pub fn cancel_all_orders_with_free_funds_ix<
    K: Into<CancelAllOrdersWithFreeFundsKeys>,
    A: Into<CancelAllOrdersWithFreeFundsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelAllOrdersWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelAllOrdersWithFreeFundsIxArgs = args.into();
    let data: CancelAllOrdersWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_all_orders_with_free_funds_invoke<
    'info,
    A: Into<CancelAllOrdersWithFreeFundsIxArgs>,
>(
    accounts: &CancelAllOrdersWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_all_orders_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_all_orders_with_free_funds_invoke_signed<
    'info,
    A: Into<CancelAllOrdersWithFreeFundsIxArgs>,
>(
    accounts: &CancelAllOrdersWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_all_orders_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_all_orders_with_free_funds_verify_account_keys(
    accounts: &CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
    keys: &CancelAllOrdersWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_with_free_funds_verify_account_privileges(
    accounts: &CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CANCEL_UP_TO_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CancelUpToAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelUpToKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&CancelUpToAccounts<'_, '_>> for CancelUpToKeys {
    fn from(accounts: &CancelUpToAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CancelUpToKeys> for [AccountMeta; CANCEL_UP_TO_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelUpToKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; CANCEL_UP_TO_IX_ACCOUNTS_LEN]> for CancelUpToKeys {
    fn from(pubkeys: [Pubkey; CANCEL_UP_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&CancelUpToAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_UP_TO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelUpToAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_UP_TO_IX_ACCOUNTS_LEN]>
    for CancelUpToAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_UP_TO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelUpToIxArgs {
    pub params: CancelUpToParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUpToIxData(pub CancelUpToIxArgs);
pub const CANCEL_UP_TO_IX_DISCM: u8 = 8u8;
impl From<CancelUpToIxArgs> for CancelUpToIxData {
    fn from(args: CancelUpToIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelUpToIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_UP_TO_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CancelUpToIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CANCEL_UP_TO_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_UP_TO_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelUpToIxArgs::deserialize(buf)?))
    }
}
pub fn cancel_up_to_ix<K: Into<CancelUpToKeys>, A: Into<CancelUpToIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelUpToKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_UP_TO_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelUpToIxArgs = args.into();
    let data: CancelUpToIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_up_to_invoke<'info, A: Into<CancelUpToIxArgs>>(
    accounts: &CancelUpToAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_up_to_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_UP_TO_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_up_to_invoke_signed<'info, A: Into<CancelUpToIxArgs>>(
    accounts: &CancelUpToAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_up_to_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_UP_TO_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_up_to_verify_account_keys(
    accounts: &CancelUpToAccounts<'_, '_>,
    keys: &CancelUpToKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_up_to_verify_account_privileges(
    accounts: &CancelUpToAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CancelUpToWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelUpToWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
}
impl From<&CancelUpToWithFreeFundsAccounts<'_, '_>> for CancelUpToWithFreeFundsKeys {
    fn from(accounts: &CancelUpToWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<&CancelUpToWithFreeFundsKeys>
    for [AccountMeta; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CancelUpToWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
        ]
    }
}
impl From<[Pubkey; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]> for CancelUpToWithFreeFundsKeys {
    fn from(pubkeys: [Pubkey; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
        }
    }
}
impl<'info> From<&CancelUpToWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelUpToWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for CancelUpToWithFreeFundsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelUpToWithFreeFundsIxArgs {
    pub params: CancelUpToParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUpToWithFreeFundsIxData(pub CancelUpToWithFreeFundsIxArgs);
pub const CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM: u8 = 9u8;
impl From<CancelUpToWithFreeFundsIxArgs> for CancelUpToWithFreeFundsIxData {
    fn from(args: CancelUpToWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelUpToWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CancelUpToWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelUpToWithFreeFundsIxArgs::deserialize(buf)?))
    }
}
pub fn cancel_up_to_with_free_funds_ix<
    K: Into<CancelUpToWithFreeFundsKeys>,
    A: Into<CancelUpToWithFreeFundsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelUpToWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelUpToWithFreeFundsIxArgs = args.into();
    let data: CancelUpToWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_up_to_with_free_funds_invoke<'info, A: Into<CancelUpToWithFreeFundsIxArgs>>(
    accounts: &CancelUpToWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_up_to_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_up_to_with_free_funds_invoke_signed<'info, A: Into<CancelUpToWithFreeFundsIxArgs>>(
    accounts: &CancelUpToWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_up_to_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_up_to_with_free_funds_verify_account_keys(
    accounts: &CancelUpToWithFreeFundsAccounts<'_, '_>,
    keys: &CancelUpToWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_up_to_with_free_funds_verify_account_privileges(
    accounts: &CancelUpToWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CancelMultipleOrdersByIdAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelMultipleOrdersByIdKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&CancelMultipleOrdersByIdAccounts<'_, '_>> for CancelMultipleOrdersByIdKeys {
    fn from(accounts: &CancelMultipleOrdersByIdAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CancelMultipleOrdersByIdKeys>
    for [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CancelMultipleOrdersByIdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]> for CancelMultipleOrdersByIdKeys {
    fn from(pubkeys: [Pubkey; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&CancelMultipleOrdersByIdAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelMultipleOrdersByIdAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]>
    for CancelMultipleOrdersByIdAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelMultipleOrdersByIdIxArgs {
    pub params: CancelMultipleOrdersByIdParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelMultipleOrdersByIdIxData(pub CancelMultipleOrdersByIdIxArgs);
pub const CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM: u8 = 10u8;
impl From<CancelMultipleOrdersByIdIxArgs> for CancelMultipleOrdersByIdIxData {
    fn from(args: CancelMultipleOrdersByIdIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelMultipleOrdersByIdIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CancelMultipleOrdersByIdIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelMultipleOrdersByIdIxArgs::deserialize(buf)?))
    }
}
pub fn cancel_multiple_orders_by_id_ix<
    K: Into<CancelMultipleOrdersByIdKeys>,
    A: Into<CancelMultipleOrdersByIdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelMultipleOrdersByIdKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelMultipleOrdersByIdIxArgs = args.into();
    let data: CancelMultipleOrdersByIdIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_multiple_orders_by_id_invoke<'info, A: Into<CancelMultipleOrdersByIdIxArgs>>(
    accounts: &CancelMultipleOrdersByIdAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_multiple_orders_by_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_multiple_orders_by_id_invoke_signed<
    'info,
    A: Into<CancelMultipleOrdersByIdIxArgs>,
>(
    accounts: &CancelMultipleOrdersByIdAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_multiple_orders_by_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_multiple_orders_by_id_verify_account_keys(
    accounts: &CancelMultipleOrdersByIdAccounts<'_, '_>,
    keys: &CancelMultipleOrdersByIdKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_multiple_orders_by_id_verify_account_privileges(
    accounts: &CancelMultipleOrdersByIdAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CancelMultipleOrdersByIdWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelMultipleOrdersByIdWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
}
impl From<&CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>>
    for CancelMultipleOrdersByIdWithFreeFundsKeys
{
    fn from(accounts: &CancelMultipleOrdersByIdWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<&CancelMultipleOrdersByIdWithFreeFundsKeys>
    for [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CancelMultipleOrdersByIdWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
        ]
    }
}
impl From<[Pubkey; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for CancelMultipleOrdersByIdWithFreeFundsKeys
{
    fn from(
        pubkeys: [Pubkey; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
        }
    }
}
impl<'info> From<&CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for CancelMultipleOrdersByIdWithFreeFundsAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>;
                 CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelMultipleOrdersByIdWithFreeFundsIxArgs {
    pub params: CancelMultipleOrdersByIdParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelMultipleOrdersByIdWithFreeFundsIxData(
    pub CancelMultipleOrdersByIdWithFreeFundsIxArgs,
);
pub const CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM: u8 = 11u8;
impl From<CancelMultipleOrdersByIdWithFreeFundsIxArgs>
    for CancelMultipleOrdersByIdWithFreeFundsIxData
{
    fn from(args: CancelMultipleOrdersByIdWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelMultipleOrdersByIdWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CancelMultipleOrdersByIdWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            CancelMultipleOrdersByIdWithFreeFundsIxArgs::deserialize(buf)?,
        ))
    }
}
pub fn cancel_multiple_orders_by_id_with_free_funds_ix<
    K: Into<CancelMultipleOrdersByIdWithFreeFundsKeys>,
    A: Into<CancelMultipleOrdersByIdWithFreeFundsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelMultipleOrdersByIdWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: CancelMultipleOrdersByIdWithFreeFundsIxArgs = args.into();
    let data: CancelMultipleOrdersByIdWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_multiple_orders_by_id_with_free_funds_invoke<
    'info,
    A: Into<CancelMultipleOrdersByIdWithFreeFundsIxArgs>,
>(
    accounts: &CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = cancel_multiple_orders_by_id_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_multiple_orders_by_id_with_free_funds_invoke_signed<
    'info,
    A: Into<CancelMultipleOrdersByIdWithFreeFundsIxArgs>,
>(
    accounts: &CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_multiple_orders_by_id_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn cancel_multiple_orders_by_id_with_free_funds_verify_account_keys(
    accounts: &CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
    keys: &CancelMultipleOrdersByIdWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cancel_multiple_orders_by_id_with_free_funds_verify_account_privileges(
    accounts: &CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const WITHDRAW_FUNDS_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&WithdrawFundsAccounts<'_, '_>> for WithdrawFundsKeys {
    fn from(accounts: &WithdrawFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&WithdrawFundsKeys> for [AccountMeta; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: &WithdrawFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN]> for WithdrawFundsKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&WithdrawFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &WithdrawFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN]>
    for WithdrawFundsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawFundsIxArgs {
    pub withdraw_funds_params: WithdrawParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFundsIxData(pub WithdrawFundsIxArgs);
pub const WITHDRAW_FUNDS_IX_DISCM: u8 = 12u8;
impl From<WithdrawFundsIxArgs> for WithdrawFundsIxData {
    fn from(args: WithdrawFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for WithdrawFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl WithdrawFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != WITHDRAW_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawFundsIxArgs::deserialize(buf)?))
    }
}
pub fn withdraw_funds_ix<K: Into<WithdrawFundsKeys>, A: Into<WithdrawFundsIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: WithdrawFundsKeys = accounts.into();
    let metas: [AccountMeta; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: WithdrawFundsIxArgs = args.into();
    let data: WithdrawFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_funds_invoke<'info, A: Into<WithdrawFundsIxArgs>>(
    accounts: &WithdrawFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = withdraw_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn withdraw_funds_invoke_signed<'info, A: Into<WithdrawFundsIxArgs>>(
    accounts: &WithdrawFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = withdraw_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn withdraw_funds_verify_account_keys(
    accounts: &WithdrawFundsAccounts<'_, '_>,
    keys: &WithdrawFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_funds_verify_account_privileges(
    accounts: &WithdrawFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const DEPOSIT_FUNDS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct DepositFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&DepositFundsAccounts<'_, '_>> for DepositFundsKeys {
    fn from(accounts: &DepositFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&DepositFundsKeys> for [AccountMeta; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: &DepositFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new_readonly(keys.seat, false),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN]> for DepositFundsKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            seat: pubkeys[4],
            base_account: pubkeys[5],
            quote_account: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&DepositFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DepositFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN]>
    for DepositFundsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            seat: &arr[4],
            base_account: &arr[5],
            quote_account: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositFundsIxArgs {
    pub deposit_funds_params: DepositParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositFundsIxData(pub DepositFundsIxArgs);
pub const DEPOSIT_FUNDS_IX_DISCM: u8 = 13u8;
impl From<DepositFundsIxArgs> for DepositFundsIxData {
    fn from(args: DepositFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DepositFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPOSIT_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl DepositFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPOSIT_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositFundsIxArgs::deserialize(buf)?))
    }
}
pub fn deposit_funds_ix<K: Into<DepositFundsKeys>, A: Into<DepositFundsIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositFundsKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositFundsIxArgs = args.into();
    let data: DepositFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_funds_invoke<'info, A: Into<DepositFundsIxArgs>>(
    accounts: &DepositFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deposit_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_funds_invoke_signed<'info, A: Into<DepositFundsIxArgs>>(
    accounts: &DepositFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deposit_funds_verify_account_keys(
    accounts: &DepositFundsAccounts<'_, '_>,
    keys: &DepositFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_funds_verify_account_privileges(
    accounts: &DepositFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const REQUEST_SEAT_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RequestSeatAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RequestSeatKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub payer: Pubkey,
    pub seat: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&RequestSeatAccounts<'_, '_>> for RequestSeatKeys {
    fn from(accounts: &RequestSeatAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            payer: *accounts.payer.key,
            seat: *accounts.seat.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&RequestSeatKeys> for [AccountMeta; REQUEST_SEAT_IX_ACCOUNTS_LEN] {
    fn from(keys: &RequestSeatKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new(keys.seat, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; REQUEST_SEAT_IX_ACCOUNTS_LEN]> for RequestSeatKeys {
    fn from(pubkeys: [Pubkey; REQUEST_SEAT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            payer: pubkeys[3],
            seat: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<&RequestSeatAccounts<'_, 'info>>
    for [AccountInfo<'info>; REQUEST_SEAT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RequestSeatAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.payer.clone(),
            accounts.seat.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REQUEST_SEAT_IX_ACCOUNTS_LEN]>
    for RequestSeatAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REQUEST_SEAT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            payer: &arr[3],
            seat: &arr[4],
            system_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestSeatIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestSeatIxData(pub RequestSeatIxArgs);
pub const REQUEST_SEAT_IX_DISCM: u8 = 14u8;
impl From<RequestSeatIxArgs> for RequestSeatIxData {
    fn from(args: RequestSeatIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RequestSeatIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REQUEST_SEAT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RequestSeatIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REQUEST_SEAT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REQUEST_SEAT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RequestSeatIxArgs::deserialize(buf)?))
    }
}
pub fn request_seat_ix<K: Into<RequestSeatKeys>, A: Into<RequestSeatIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RequestSeatKeys = accounts.into();
    let metas: [AccountMeta; REQUEST_SEAT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RequestSeatIxArgs = args.into();
    let data: RequestSeatIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_seat_invoke<'info, A: Into<RequestSeatIxArgs>>(
    accounts: &RequestSeatAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = request_seat_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REQUEST_SEAT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn request_seat_invoke_signed<'info, A: Into<RequestSeatIxArgs>>(
    accounts: &RequestSeatAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = request_seat_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REQUEST_SEAT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn request_seat_verify_account_keys(
    accounts: &RequestSeatAccounts<'_, '_>,
    keys: &RequestSeatKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.payer.key, &keys.payer),
        (accounts.seat.key, &keys.seat),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn request_seat_verify_account_privileges(
    accounts: &RequestSeatAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market, accounts.payer, accounts.seat] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const LOG_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct LogAccounts<'me, 'info> {
    ///Log authority
    pub log_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct LogKeys {
    ///Log authority
    pub log_authority: Pubkey,
}
impl From<&LogAccounts<'_, '_>> for LogKeys {
    fn from(accounts: &LogAccounts) -> Self {
        Self {
            log_authority: *accounts.log_authority.key,
        }
    }
}
impl From<&LogKeys> for [AccountMeta; LOG_IX_ACCOUNTS_LEN] {
    fn from(keys: &LogKeys) -> Self {
        [AccountMeta::new_readonly(keys.log_authority, true)]
    }
}
impl From<[Pubkey; LOG_IX_ACCOUNTS_LEN]> for LogKeys {
    fn from(pubkeys: [Pubkey; LOG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            log_authority: pubkeys[0],
        }
    }
}
impl<'info> From<&LogAccounts<'_, 'info>> for [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN] {
    fn from(accounts: &LogAccounts<'_, 'info>) -> Self {
        [accounts.log_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN]> for LogAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            log_authority: &arr[0],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct LogIxData(pub LogIxArgs);
pub const LOG_IX_DISCM: u8 = 15u8;
impl From<LogIxArgs> for LogIxData {
    fn from(args: LogIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LogIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[LOG_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl LogIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != LOG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LOG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LogIxArgs::deserialize(buf)?))
    }
}
pub fn log_ix<K: Into<LogKeys>, A: Into<LogIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LogKeys = accounts.into();
    let metas: [AccountMeta; LOG_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LogIxArgs = args.into();
    let data: LogIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn log_invoke<'info, A: Into<LogIxArgs>>(
    accounts: &LogAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = log_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn log_invoke_signed<'info, A: Into<LogIxArgs>>(
    accounts: &LogAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = log_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn log_verify_account_keys(
    accounts: &LogAccounts<'_, '_>,
    keys: &LogKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.log_authority.key, &keys.log_authority)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn log_verify_account_privileges(accounts: &LogAccounts<'_, '_>) -> Result<(), ProgramError> {
    for should_be_signer in [accounts.log_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct PlaceMultiplePostOnlyOrdersAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceMultiplePostOnlyOrdersKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&PlaceMultiplePostOnlyOrdersAccounts<'_, '_>> for PlaceMultiplePostOnlyOrdersKeys {
    fn from(accounts: &PlaceMultiplePostOnlyOrdersAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&PlaceMultiplePostOnlyOrdersKeys>
    for [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &PlaceMultiplePostOnlyOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new_readonly(keys.seat, false),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]>
    for PlaceMultiplePostOnlyOrdersKeys
{
    fn from(pubkeys: [Pubkey; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            seat: pubkeys[4],
            base_account: pubkeys[5],
            quote_account: pubkeys[6],
            base_vault: pubkeys[7],
            quote_vault: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&PlaceMultiplePostOnlyOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceMultiplePostOnlyOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]>
    for PlaceMultiplePostOnlyOrdersAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            seat: &arr[4],
            base_account: &arr[5],
            quote_account: &arr[6],
            base_vault: &arr[7],
            quote_vault: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceMultiplePostOnlyOrdersIxArgs {
    pub multiple_order_packet: MultipleOrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceMultiplePostOnlyOrdersIxData(pub PlaceMultiplePostOnlyOrdersIxArgs);
pub const PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM: u8 = 16u8;
impl From<PlaceMultiplePostOnlyOrdersIxArgs> for PlaceMultiplePostOnlyOrdersIxData {
    fn from(args: PlaceMultiplePostOnlyOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceMultiplePostOnlyOrdersIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PlaceMultiplePostOnlyOrdersIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceMultiplePostOnlyOrdersIxArgs::deserialize(buf)?))
    }
}
pub fn place_multiple_post_only_orders_ix<
    K: Into<PlaceMultiplePostOnlyOrdersKeys>,
    A: Into<PlaceMultiplePostOnlyOrdersIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceMultiplePostOnlyOrdersKeys = accounts.into();
    let metas: [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceMultiplePostOnlyOrdersIxArgs = args.into();
    let data: PlaceMultiplePostOnlyOrdersIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_multiple_post_only_orders_invoke<'info, A: Into<PlaceMultiplePostOnlyOrdersIxArgs>>(
    accounts: &PlaceMultiplePostOnlyOrdersAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_multiple_post_only_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_multiple_post_only_orders_invoke_signed<
    'info,
    A: Into<PlaceMultiplePostOnlyOrdersIxArgs>,
>(
    accounts: &PlaceMultiplePostOnlyOrdersAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_multiple_post_only_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_multiple_post_only_orders_verify_account_keys(
    accounts: &PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
    keys: &PlaceMultiplePostOnlyOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_multiple_post_only_orders_verify_account_privileges(
    accounts: &PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
}
impl From<&PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>>
    for PlaceMultiplePostOnlyOrdersWithFreeFundsKeys
{
    fn from(accounts: &PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<&PlaceMultiplePostOnlyOrdersWithFreeFundsKeys>
    for [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &PlaceMultiplePostOnlyOrdersWithFreeFundsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.trader, true),
            AccountMeta::new_readonly(keys.seat, false),
        ]
    }
}
impl From<[Pubkey; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for PlaceMultiplePostOnlyOrdersWithFreeFundsKeys
{
    fn from(
        pubkeys: [Pubkey; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            seat: pubkeys[4],
        }
    }
}
impl<'info> From<&PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]>
    for PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>;
                 PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            seat: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs {
    pub multiple_order_packet: MultipleOrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsIxData(
    pub PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
);
pub const PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM: u8 = 17u8;
impl From<PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs>
    for PlaceMultiplePostOnlyOrdersWithFreeFundsIxData
{
    fn from(args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceMultiplePostOnlyOrdersWithFreeFundsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl PlaceMultiplePostOnlyOrdersWithFreeFundsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs::deserialize(buf)?,
        ))
    }
}
pub fn place_multiple_post_only_orders_with_free_funds_ix<
    K: Into<PlaceMultiplePostOnlyOrdersWithFreeFundsKeys>,
    A: Into<PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys = accounts.into();
    let metas: [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs = args.into();
    let data: PlaceMultiplePostOnlyOrdersWithFreeFundsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_multiple_post_only_orders_with_free_funds_invoke<
    'info,
    A: Into<PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs>,
>(
    accounts: &PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = place_multiple_post_only_orders_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_multiple_post_only_orders_with_free_funds_invoke_signed<
    'info,
    A: Into<PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs>,
>(
    accounts: &PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_multiple_post_only_orders_with_free_funds_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn place_multiple_post_only_orders_with_free_funds_verify_account_keys(
    accounts: &PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
    keys: &PlaceMultiplePostOnlyOrdersWithFreeFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn place_multiple_post_only_orders_with_free_funds_verify_account_privileges(
    accounts: &PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const INITIALIZE_MARKET_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitializeMarketAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_creator account must sign for the creation of new vaults
    pub market_creator: &'me AccountInfo<'info>,
    ///Base mint account
    pub base_mint: &'me AccountInfo<'info>,
    ///Quote mint account
    pub quote_mint: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeMarketKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_creator account must sign for the creation of new vaults
    pub market_creator: Pubkey,
    ///Base mint account
    pub base_mint: Pubkey,
    ///Quote mint account
    pub quote_mint: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&InitializeMarketAccounts<'_, '_>> for InitializeMarketKeys {
    fn from(accounts: &InitializeMarketAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_creator: *accounts.market_creator.key,
            base_mint: *accounts.base_mint.key,
            quote_mint: *accounts.quote_mint.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&InitializeMarketKeys> for [AccountMeta; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeMarketKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new(keys.market_creator, true),
            AccountMeta::new_readonly(keys.base_mint, false),
            AccountMeta::new_readonly(keys.quote_mint, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]> for InitializeMarketKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_creator: pubkeys[3],
            base_mint: pubkeys[4],
            quote_mint: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&InitializeMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_creator.clone(),
            accounts.base_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]>
    for InitializeMarketAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_creator: &arr[3],
            base_mint: &arr[4],
            quote_mint: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMarketIxArgs {
    pub initialize_params: InitializeParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMarketIxData(pub InitializeMarketIxArgs);
pub const INITIALIZE_MARKET_IX_DISCM: u8 = 100u8;
impl From<InitializeMarketIxArgs> for InitializeMarketIxData {
    fn from(args: InitializeMarketIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeMarketIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_MARKET_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl InitializeMarketIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != INITIALIZE_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeMarketIxArgs::deserialize(buf)?))
    }
}
pub fn initialize_market_ix<K: Into<InitializeMarketKeys>, A: Into<InitializeMarketIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeMarketKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeMarketIxArgs = args.into();
    let data: InitializeMarketIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_market_invoke<'info, A: Into<InitializeMarketIxArgs>>(
    accounts: &InitializeMarketAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = initialize_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_market_invoke_signed<'info, A: Into<InitializeMarketIxArgs>>(
    accounts: &InitializeMarketAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn initialize_market_verify_account_keys(
    accounts: &InitializeMarketAccounts<'_, '_>,
    keys: &InitializeMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_creator.key, &keys.market_creator),
        (accounts.base_mint.key, &keys.base_mint),
        (accounts.quote_mint.key, &keys.quote_mint),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_market_verify_account_privileges(
    accounts: &InitializeMarketAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.market_creator,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_creator] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CLAIM_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ClaimAuthorityAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The successor account must sign to claim authority
    pub successor: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ClaimAuthorityKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The successor account must sign to claim authority
    pub successor: Pubkey,
}
impl From<&ClaimAuthorityAccounts<'_, '_>> for ClaimAuthorityKeys {
    fn from(accounts: &ClaimAuthorityAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            successor: *accounts.successor.key,
        }
    }
}
impl From<&ClaimAuthorityKeys> for [AccountMeta; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ClaimAuthorityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.successor, true),
        ]
    }
}
impl From<[Pubkey; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]> for ClaimAuthorityKeys {
    fn from(pubkeys: [Pubkey; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            successor: pubkeys[3],
        }
    }
}
impl<'info> From<&ClaimAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ClaimAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.successor.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]>
    for ClaimAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            successor: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimAuthorityIxData(pub ClaimAuthorityIxArgs);
pub const CLAIM_AUTHORITY_IX_DISCM: u8 = 101u8;
impl From<ClaimAuthorityIxArgs> for ClaimAuthorityIxData {
    fn from(args: ClaimAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ClaimAuthorityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CLAIM_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ClaimAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CLAIM_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimAuthorityIxArgs::deserialize(buf)?))
    }
}
pub fn claim_authority_ix<K: Into<ClaimAuthorityKeys>, A: Into<ClaimAuthorityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ClaimAuthorityKeys = accounts.into();
    let metas: [AccountMeta; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ClaimAuthorityIxArgs = args.into();
    let data: ClaimAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_authority_invoke<'info, A: Into<ClaimAuthorityIxArgs>>(
    accounts: &ClaimAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = claim_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn claim_authority_invoke_signed<'info, A: Into<ClaimAuthorityIxArgs>>(
    accounts: &ClaimAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = claim_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn claim_authority_verify_account_keys(
    accounts: &ClaimAuthorityAccounts<'_, '_>,
    keys: &ClaimAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.successor.key, &keys.successor),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn claim_authority_verify_account_privileges(
    accounts: &ClaimAuthorityAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.successor] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const NAME_SUCCESSOR_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct NameSuccessorAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to name successor
    pub market_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NameSuccessorKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to name successor
    pub market_authority: Pubkey,
}
impl From<&NameSuccessorAccounts<'_, '_>> for NameSuccessorKeys {
    fn from(accounts: &NameSuccessorAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
        }
    }
}
impl From<&NameSuccessorKeys> for [AccountMeta; NAME_SUCCESSOR_IX_ACCOUNTS_LEN] {
    fn from(keys: &NameSuccessorKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
        ]
    }
}
impl From<[Pubkey; NAME_SUCCESSOR_IX_ACCOUNTS_LEN]> for NameSuccessorKeys {
    fn from(pubkeys: [Pubkey; NAME_SUCCESSOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
        }
    }
}
impl<'info> From<&NameSuccessorAccounts<'_, 'info>>
    for [AccountInfo<'info>; NAME_SUCCESSOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &NameSuccessorAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NAME_SUCCESSOR_IX_ACCOUNTS_LEN]>
    for NameSuccessorAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; NAME_SUCCESSOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameSuccessorIxArgs {
    pub successor: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NameSuccessorIxData(pub NameSuccessorIxArgs);
pub const NAME_SUCCESSOR_IX_DISCM: u8 = 102u8;
impl From<NameSuccessorIxArgs> for NameSuccessorIxData {
    fn from(args: NameSuccessorIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for NameSuccessorIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[NAME_SUCCESSOR_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl NameSuccessorIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != NAME_SUCCESSOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NAME_SUCCESSOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(NameSuccessorIxArgs::deserialize(buf)?))
    }
}
pub fn name_successor_ix<K: Into<NameSuccessorKeys>, A: Into<NameSuccessorIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: NameSuccessorKeys = accounts.into();
    let metas: [AccountMeta; NAME_SUCCESSOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: NameSuccessorIxArgs = args.into();
    let data: NameSuccessorIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn name_successor_invoke<'info, A: Into<NameSuccessorIxArgs>>(
    accounts: &NameSuccessorAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = name_successor_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; NAME_SUCCESSOR_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn name_successor_invoke_signed<'info, A: Into<NameSuccessorIxArgs>>(
    accounts: &NameSuccessorAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = name_successor_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; NAME_SUCCESSOR_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn name_successor_verify_account_keys(
    accounts: &NameSuccessorAccounts<'_, '_>,
    keys: &NameSuccessorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn name_successor_verify_account_privileges(
    accounts: &NameSuccessorAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct ChangeMarketStatusAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to change market status
    pub market_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ChangeMarketStatusKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to change market status
    pub market_authority: Pubkey,
}
impl From<&ChangeMarketStatusAccounts<'_, '_>> for ChangeMarketStatusKeys {
    fn from(accounts: &ChangeMarketStatusAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
        }
    }
}
impl From<&ChangeMarketStatusKeys> for [AccountMeta; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: &ChangeMarketStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
        ]
    }
}
impl From<[Pubkey; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN]> for ChangeMarketStatusKeys {
    fn from(pubkeys: [Pubkey; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
        }
    }
}
impl<'info> From<&ChangeMarketStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ChangeMarketStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN]>
    for ChangeMarketStatusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeMarketStatusIxArgs {
    pub market_status: MarketStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeMarketStatusIxData(pub ChangeMarketStatusIxArgs);
pub const CHANGE_MARKET_STATUS_IX_DISCM: u8 = 103u8;
impl From<ChangeMarketStatusIxArgs> for ChangeMarketStatusIxData {
    fn from(args: ChangeMarketStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ChangeMarketStatusIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_MARKET_STATUS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ChangeMarketStatusIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CHANGE_MARKET_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_MARKET_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeMarketStatusIxArgs::deserialize(buf)?))
    }
}
pub fn change_market_status_ix<
    K: Into<ChangeMarketStatusKeys>,
    A: Into<ChangeMarketStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ChangeMarketStatusKeys = accounts.into();
    let metas: [AccountMeta; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ChangeMarketStatusIxArgs = args.into();
    let data: ChangeMarketStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_market_status_invoke<'info, A: Into<ChangeMarketStatusIxArgs>>(
    accounts: &ChangeMarketStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = change_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn change_market_status_invoke_signed<'info, A: Into<ChangeMarketStatusIxArgs>>(
    accounts: &ChangeMarketStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = change_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn change_market_status_verify_account_keys(
    accounts: &ChangeMarketStatusAccounts<'_, '_>,
    keys: &ChangeMarketStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn change_market_status_verify_account_privileges(
    accounts: &ChangeMarketStatusAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ChangeSeatStatusAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to change seat status
    pub market_authority: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ChangeSeatStatusKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to change seat status
    pub market_authority: Pubkey,
    pub seat: Pubkey,
}
impl From<&ChangeSeatStatusAccounts<'_, '_>> for ChangeSeatStatusKeys {
    fn from(accounts: &ChangeSeatStatusAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<&ChangeSeatStatusKeys> for [AccountMeta; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: &ChangeSeatStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
            AccountMeta::new(keys.seat, false),
        ]
    }
}
impl From<[Pubkey; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN]> for ChangeSeatStatusKeys {
    fn from(pubkeys: [Pubkey; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
            seat: pubkeys[4],
        }
    }
}
impl<'info> From<&ChangeSeatStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ChangeSeatStatusAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.seat.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN]>
    for ChangeSeatStatusAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
            seat: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeSeatStatusIxArgs {
    pub approval_status: SeatApprovalStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeSeatStatusIxData(pub ChangeSeatStatusIxArgs);
pub const CHANGE_SEAT_STATUS_IX_DISCM: u8 = 104u8;
impl From<ChangeSeatStatusIxArgs> for ChangeSeatStatusIxData {
    fn from(args: ChangeSeatStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ChangeSeatStatusIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_SEAT_STATUS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ChangeSeatStatusIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CHANGE_SEAT_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_SEAT_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeSeatStatusIxArgs::deserialize(buf)?))
    }
}
pub fn change_seat_status_ix<K: Into<ChangeSeatStatusKeys>, A: Into<ChangeSeatStatusIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ChangeSeatStatusKeys = accounts.into();
    let metas: [AccountMeta; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ChangeSeatStatusIxArgs = args.into();
    let data: ChangeSeatStatusIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_seat_status_invoke<'info, A: Into<ChangeSeatStatusIxArgs>>(
    accounts: &ChangeSeatStatusAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = change_seat_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn change_seat_status_invoke_signed<'info, A: Into<ChangeSeatStatusIxArgs>>(
    accounts: &ChangeSeatStatusAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = change_seat_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn change_seat_status_verify_account_keys(
    accounts: &ChangeSeatStatusAccounts<'_, '_>,
    keys: &ChangeSeatStatusKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
        (accounts.seat.key, &keys.seat),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn change_seat_status_verify_account_privileges(
    accounts: &ChangeSeatStatusAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market, accounts.seat] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RequestSeatAuthorizedAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to request a seat on behalf of a trader
    pub market_authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub seat: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RequestSeatAuthorizedKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to request a seat on behalf of a trader
    pub market_authority: Pubkey,
    pub payer: Pubkey,
    pub trader: Pubkey,
    pub seat: Pubkey,
    ///System program
    pub system_program: Pubkey,
}
impl From<&RequestSeatAuthorizedAccounts<'_, '_>> for RequestSeatAuthorizedKeys {
    fn from(accounts: &RequestSeatAuthorizedAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            payer: *accounts.payer.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&RequestSeatAuthorizedKeys> for [AccountMeta; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN] {
    fn from(keys: &RequestSeatAuthorizedKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.trader, false),
            AccountMeta::new(keys.seat, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN]> for RequestSeatAuthorizedKeys {
    fn from(pubkeys: [Pubkey; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
            payer: pubkeys[4],
            trader: pubkeys[5],
            seat: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<&RequestSeatAuthorizedAccounts<'_, 'info>>
    for [AccountInfo<'info>; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RequestSeatAuthorizedAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.payer.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN]>
    for RequestSeatAuthorizedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
            payer: &arr[4],
            trader: &arr[5],
            seat: &arr[6],
            system_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestSeatAuthorizedIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestSeatAuthorizedIxData(pub RequestSeatAuthorizedIxArgs);
pub const REQUEST_SEAT_AUTHORIZED_IX_DISCM: u8 = 105u8;
impl From<RequestSeatAuthorizedIxArgs> for RequestSeatAuthorizedIxData {
    fn from(args: RequestSeatAuthorizedIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RequestSeatAuthorizedIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REQUEST_SEAT_AUTHORIZED_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl RequestSeatAuthorizedIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REQUEST_SEAT_AUTHORIZED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REQUEST_SEAT_AUTHORIZED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RequestSeatAuthorizedIxArgs::deserialize(buf)?))
    }
}
pub fn request_seat_authorized_ix<
    K: Into<RequestSeatAuthorizedKeys>,
    A: Into<RequestSeatAuthorizedIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RequestSeatAuthorizedKeys = accounts.into();
    let metas: [AccountMeta; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RequestSeatAuthorizedIxArgs = args.into();
    let data: RequestSeatAuthorizedIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_seat_authorized_invoke<'info, A: Into<RequestSeatAuthorizedIxArgs>>(
    accounts: &RequestSeatAuthorizedAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = request_seat_authorized_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn request_seat_authorized_invoke_signed<'info, A: Into<RequestSeatAuthorizedIxArgs>>(
    accounts: &RequestSeatAuthorizedAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = request_seat_authorized_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn request_seat_authorized_verify_account_keys(
    accounts: &RequestSeatAuthorizedAccounts<'_, '_>,
    keys: &RequestSeatAuthorizedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn request_seat_authorized_verify_account_privileges(
    accounts: &RequestSeatAuthorizedAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market, accounts.payer, accounts.seat] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const EVICT_SEAT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct EvictSeatAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to evict a seat
    pub market_authority: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///The trader's PDA seat account, seeds are [b'seat', market_address, trader_address]
    pub seat: &'me AccountInfo<'info>,
    pub base_account: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct EvictSeatKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to evict a seat
    pub market_authority: Pubkey,
    pub trader: Pubkey,
    ///The trader's PDA seat account, seeds are [b'seat', market_address, trader_address]
    pub seat: Pubkey,
    pub base_account: Pubkey,
    pub quote_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&EvictSeatAccounts<'_, '_>> for EvictSeatKeys {
    fn from(accounts: &EvictSeatAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&EvictSeatKeys> for [AccountMeta; EVICT_SEAT_IX_ACCOUNTS_LEN] {
    fn from(keys: &EvictSeatKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
            AccountMeta::new_readonly(keys.trader, false),
            AccountMeta::new_readonly(keys.seat, false),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; EVICT_SEAT_IX_ACCOUNTS_LEN]> for EvictSeatKeys {
    fn from(pubkeys: [Pubkey; EVICT_SEAT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
            trader: pubkeys[4],
            seat: pubkeys[5],
            base_account: pubkeys[6],
            quote_account: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<&EvictSeatAccounts<'_, 'info>>
    for [AccountInfo<'info>; EVICT_SEAT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EvictSeatAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EVICT_SEAT_IX_ACCOUNTS_LEN]>
    for EvictSeatAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; EVICT_SEAT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
            trader: &arr[4],
            seat: &arr[5],
            base_account: &arr[6],
            quote_account: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            token_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvictSeatIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct EvictSeatIxData(pub EvictSeatIxArgs);
pub const EVICT_SEAT_IX_DISCM: u8 = 106u8;
impl From<EvictSeatIxArgs> for EvictSeatIxData {
    fn from(args: EvictSeatIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EvictSeatIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[EVICT_SEAT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl EvictSeatIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != EVICT_SEAT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVICT_SEAT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EvictSeatIxArgs::deserialize(buf)?))
    }
}
pub fn evict_seat_ix<K: Into<EvictSeatKeys>, A: Into<EvictSeatIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EvictSeatKeys = accounts.into();
    let metas: [AccountMeta; EVICT_SEAT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EvictSeatIxArgs = args.into();
    let data: EvictSeatIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn evict_seat_invoke<'info, A: Into<EvictSeatIxArgs>>(
    accounts: &EvictSeatAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = evict_seat_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; EVICT_SEAT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn evict_seat_invoke_signed<'info, A: Into<EvictSeatIxArgs>>(
    accounts: &EvictSeatAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = evict_seat_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; EVICT_SEAT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn evict_seat_verify_account_keys(
    accounts: &EvictSeatAccounts<'_, '_>,
    keys: &EvictSeatKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn evict_seat_verify_account_privileges(
    accounts: &EvictSeatAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ForceCancelOrdersAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to claim authority
    pub market_authority: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    ///The trader's PDA seat account, seeds are [b'seat', market_address, trader_address]
    pub seat: &'me AccountInfo<'info>,
    ///Trader base token account
    pub base_account: &'me AccountInfo<'info>,
    ///Trader quote token account
    pub quote_account: &'me AccountInfo<'info>,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ForceCancelOrdersKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to claim authority
    pub market_authority: Pubkey,
    pub trader: Pubkey,
    ///The trader's PDA seat account, seeds are [b'seat', market_address, trader_address]
    pub seat: Pubkey,
    ///Trader base token account
    pub base_account: Pubkey,
    ///Trader quote token account
    pub quote_account: Pubkey,
    ///Base vault PDA, seeds are [b'vault', market_address, base_mint_address]
    pub base_vault: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&ForceCancelOrdersAccounts<'_, '_>> for ForceCancelOrdersKeys {
    fn from(accounts: &ForceCancelOrdersAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ForceCancelOrdersKeys> for [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: &ForceCancelOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
            AccountMeta::new_readonly(keys.trader, false),
            AccountMeta::new_readonly(keys.seat, false),
            AccountMeta::new(keys.base_account, false),
            AccountMeta::new(keys.quote_account, false),
            AccountMeta::new(keys.base_vault, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]> for ForceCancelOrdersKeys {
    fn from(pubkeys: [Pubkey; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
            trader: pubkeys[4],
            seat: pubkeys[5],
            base_account: pubkeys[6],
            quote_account: pubkeys[7],
            base_vault: pubkeys[8],
            quote_vault: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<&ForceCancelOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ForceCancelOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.trader.clone(),
            accounts.seat.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]>
    for ForceCancelOrdersAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
            trader: &arr[4],
            seat: &arr[5],
            base_account: &arr[6],
            quote_account: &arr[7],
            base_vault: &arr[8],
            quote_vault: &arr[9],
            token_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForceCancelOrdersIxArgs {
    pub params: CancelUpToParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ForceCancelOrdersIxData(pub ForceCancelOrdersIxArgs);
pub const FORCE_CANCEL_ORDERS_IX_DISCM: u8 = 107u8;
impl From<ForceCancelOrdersIxArgs> for ForceCancelOrdersIxData {
    fn from(args: ForceCancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ForceCancelOrdersIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[FORCE_CANCEL_ORDERS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ForceCancelOrdersIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != FORCE_CANCEL_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FORCE_CANCEL_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ForceCancelOrdersIxArgs::deserialize(buf)?))
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
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
        (accounts.trader.key, &keys.trader),
        (accounts.seat.key, &keys.seat),
        (accounts.base_account.key, &keys.base_account),
        (accounts.quote_account.key, &keys.quote_account),
        (accounts.base_vault.key, &keys.base_vault),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn force_cancel_orders_verify_account_privileges(
    accounts: &ForceCancelOrdersAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const COLLECT_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CollectFeesAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///Signer of collect fees instruction
    pub sweeper: &'me AccountInfo<'info>,
    ///Fee collector quote token account
    pub fee_recipient: &'me AccountInfo<'info>,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CollectFeesKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///Signer of collect fees instruction
    pub sweeper: Pubkey,
    ///Fee collector quote token account
    pub fee_recipient: Pubkey,
    ///Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]
    pub quote_vault: Pubkey,
    ///Token program
    pub token_program: Pubkey,
}
impl From<&CollectFeesAccounts<'_, '_>> for CollectFeesKeys {
    fn from(accounts: &CollectFeesAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            sweeper: *accounts.sweeper.key,
            fee_recipient: *accounts.fee_recipient.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&CollectFeesKeys> for [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: &CollectFeesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.sweeper, true),
            AccountMeta::new(keys.fee_recipient, false),
            AccountMeta::new(keys.quote_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]> for CollectFeesKeys {
    fn from(pubkeys: [Pubkey; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            sweeper: pubkeys[3],
            fee_recipient: pubkeys[4],
            quote_vault: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<&CollectFeesAccounts<'_, 'info>>
    for [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CollectFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.sweeper.clone(),
            accounts.fee_recipient.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]>
    for CollectFeesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            sweeper: &arr[3],
            fee_recipient: &arr[4],
            quote_vault: &arr[5],
            token_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectFeesIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesIxData(pub CollectFeesIxArgs);
pub const COLLECT_FEES_IX_DISCM: u8 = 108u8;
impl From<CollectFeesIxArgs> for CollectFeesIxData {
    fn from(args: CollectFeesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CollectFeesIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[COLLECT_FEES_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl CollectFeesIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != COLLECT_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CollectFeesIxArgs::deserialize(buf)?))
    }
}
pub fn collect_fees_ix<K: Into<CollectFeesKeys>, A: Into<CollectFeesIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CollectFeesKeys = accounts.into();
    let metas: [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CollectFeesIxArgs = args.into();
    let data: CollectFeesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn collect_fees_invoke<'info, A: Into<CollectFeesIxArgs>>(
    accounts: &CollectFeesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = collect_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn collect_fees_invoke_signed<'info, A: Into<CollectFeesIxArgs>>(
    accounts: &CollectFeesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = collect_fees_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn collect_fees_verify_account_keys(
    accounts: &CollectFeesAccounts<'_, '_>,
    keys: &CollectFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.sweeper.key, &keys.sweeper),
        (accounts.fee_recipient.key, &keys.fee_recipient),
        (accounts.quote_vault.key, &keys.quote_vault),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_account_privileges(
    accounts: &CollectFeesAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [
        accounts.market,
        accounts.fee_recipient,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.sweeper] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
pub const CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ChangeFeeRecipientAccounts<'me, 'info> {
    ///Phoenix program
    pub phoenix_program: &'me AccountInfo<'info>,
    ///Phoenix log authority
    pub log_authority: &'me AccountInfo<'info>,
    ///This account holds the market state
    pub market: &'me AccountInfo<'info>,
    ///The market_authority account must sign to change the free recipient
    pub market_authority: &'me AccountInfo<'info>,
    ///New fee recipient
    pub new_fee_recipient: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ChangeFeeRecipientKeys {
    ///Phoenix program
    pub phoenix_program: Pubkey,
    ///Phoenix log authority
    pub log_authority: Pubkey,
    ///This account holds the market state
    pub market: Pubkey,
    ///The market_authority account must sign to change the free recipient
    pub market_authority: Pubkey,
    ///New fee recipient
    pub new_fee_recipient: Pubkey,
}
impl From<&ChangeFeeRecipientAccounts<'_, '_>> for ChangeFeeRecipientKeys {
    fn from(accounts: &ChangeFeeRecipientAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            new_fee_recipient: *accounts.new_fee_recipient.key,
        }
    }
}
impl From<&ChangeFeeRecipientKeys> for [AccountMeta; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN] {
    fn from(keys: &ChangeFeeRecipientKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.log_authority, false),
            AccountMeta::new(keys.market, false),
            AccountMeta::new_readonly(keys.market_authority, true),
            AccountMeta::new_readonly(keys.new_fee_recipient, false),
        ]
    }
}
impl From<[Pubkey; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN]> for ChangeFeeRecipientKeys {
    fn from(pubkeys: [Pubkey; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            market_authority: pubkeys[3],
            new_fee_recipient: pubkeys[4],
        }
    }
}
impl<'info> From<&ChangeFeeRecipientAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ChangeFeeRecipientAccounts<'_, 'info>) -> Self {
        [
            accounts.phoenix_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.new_fee_recipient.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN]>
    for ChangeFeeRecipientAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            phoenix_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            market_authority: &arr[3],
            new_fee_recipient: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeFeeRecipientIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeFeeRecipientIxData(pub ChangeFeeRecipientIxArgs);
pub const CHANGE_FEE_RECIPIENT_IX_DISCM: u8 = 109u8;
impl From<ChangeFeeRecipientIxArgs> for ChangeFeeRecipientIxData {
    fn from(args: ChangeFeeRecipientIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ChangeFeeRecipientIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_FEE_RECIPIENT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
impl ChangeFeeRecipientIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CHANGE_FEE_RECIPIENT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_FEE_RECIPIENT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeFeeRecipientIxArgs::deserialize(buf)?))
    }
}
pub fn change_fee_recipient_ix<
    K: Into<ChangeFeeRecipientKeys>,
    A: Into<ChangeFeeRecipientIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ChangeFeeRecipientKeys = accounts.into();
    let metas: [AccountMeta; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ChangeFeeRecipientIxArgs = args.into();
    let data: ChangeFeeRecipientIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_fee_recipient_invoke<'info, A: Into<ChangeFeeRecipientIxArgs>>(
    accounts: &ChangeFeeRecipientAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = change_fee_recipient_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn change_fee_recipient_invoke_signed<'info, A: Into<ChangeFeeRecipientIxArgs>>(
    accounts: &ChangeFeeRecipientAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = change_fee_recipient_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn change_fee_recipient_verify_account_keys(
    accounts: &ChangeFeeRecipientAccounts<'_, '_>,
    keys: &ChangeFeeRecipientKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.phoenix_program.key, &keys.phoenix_program),
        (accounts.log_authority.key, &keys.log_authority),
        (accounts.market.key, &keys.market),
        (accounts.market_authority.key, &keys.market_authority),
        (accounts.new_fee_recipient.key, &keys.new_fee_recipient),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn change_fee_recipient_verify_account_privileges(
    accounts: &ChangeFeeRecipientAccounts<'_, '_>,
) -> Result<(), ProgramError> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err(ProgramError::InvalidAccountData);
        }
    }
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    Ok(())
}
