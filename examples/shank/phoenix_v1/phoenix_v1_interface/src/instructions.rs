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
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum PhoenixV1ProgramIx {
    Swap(SwapIxArgs),
    SwapWithFreeFunds(SwapWithFreeFundsIxArgs),
    PlaceLimitOrder(PlaceLimitOrderIxArgs),
    PlaceLimitOrderWithFreeFunds(PlaceLimitOrderWithFreeFundsIxArgs),
    ReduceOrder(ReduceOrderIxArgs),
    ReduceOrderWithFreeFunds(ReduceOrderWithFreeFundsIxArgs),
    CancelAllOrders,
    CancelAllOrdersWithFreeFunds,
    CancelUpTo(CancelUpToIxArgs),
    CancelUpToWithFreeFunds(CancelUpToWithFreeFundsIxArgs),
    CancelMultipleOrdersById(CancelMultipleOrdersByIdIxArgs),
    CancelMultipleOrdersByIdWithFreeFunds(CancelMultipleOrdersByIdWithFreeFundsIxArgs),
    WithdrawFunds(WithdrawFundsIxArgs),
    DepositFunds(DepositFundsIxArgs),
    RequestSeat,
    Log,
    PlaceMultiplePostOnlyOrders(PlaceMultiplePostOnlyOrdersIxArgs),
    PlaceMultiplePostOnlyOrdersWithFreeFunds(PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs),
    InitializeMarket(InitializeMarketIxArgs),
    ClaimAuthority,
    NameSuccessor(NameSuccessorIxArgs),
    ChangeMarketStatus(ChangeMarketStatusIxArgs),
    ChangeSeatStatus(ChangeSeatStatusIxArgs),
    RequestSeatAuthorized,
    EvictSeat,
    ForceCancelOrders(ForceCancelOrdersIxArgs),
    CollectFees,
    ChangeFeeRecipient,
}
impl PhoenixV1ProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            SWAP_IX_DISCM => Ok(Self::Swap(SwapIxArgs::deserialize(&mut reader)?)),
            SWAP_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::SwapWithFreeFunds(
                SwapWithFreeFundsIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_LIMIT_ORDER_IX_DISCM => Ok(Self::PlaceLimitOrder(
                PlaceLimitOrderIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::PlaceLimitOrderWithFreeFunds(
                PlaceLimitOrderWithFreeFundsIxArgs::deserialize(&mut reader)?,
            )),
            REDUCE_ORDER_IX_DISCM => Ok(Self::ReduceOrder(ReduceOrderIxArgs::deserialize(
                &mut reader,
            )?)),
            REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::ReduceOrderWithFreeFunds(
                ReduceOrderWithFreeFundsIxArgs::deserialize(&mut reader)?,
            )),
            CANCEL_ALL_ORDERS_IX_DISCM => Ok(Self::CancelAllOrders),
            CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::CancelAllOrdersWithFreeFunds),
            CANCEL_UP_TO_IX_DISCM => Ok(Self::CancelUpTo(CancelUpToIxArgs::deserialize(
                &mut reader,
            )?)),
            CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM => Ok(Self::CancelUpToWithFreeFunds(
                CancelUpToWithFreeFundsIxArgs::deserialize(&mut reader)?,
            )),
            CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM => Ok(Self::CancelMultipleOrdersById(
                CancelMultipleOrdersByIdIxArgs::deserialize(&mut reader)?,
            )),
            CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM => {
                Ok(Self::CancelMultipleOrdersByIdWithFreeFunds(
                    CancelMultipleOrdersByIdWithFreeFundsIxArgs::deserialize(&mut reader)?,
                ))
            }
            WITHDRAW_FUNDS_IX_DISCM => Ok(Self::WithdrawFunds(WithdrawFundsIxArgs::deserialize(
                &mut reader,
            )?)),
            DEPOSIT_FUNDS_IX_DISCM => Ok(Self::DepositFunds(DepositFundsIxArgs::deserialize(
                &mut reader,
            )?)),
            REQUEST_SEAT_IX_DISCM => Ok(Self::RequestSeat),
            LOG_IX_DISCM => Ok(Self::Log),
            PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM => Ok(Self::PlaceMultiplePostOnlyOrders(
                PlaceMultiplePostOnlyOrdersIxArgs::deserialize(&mut reader)?,
            )),
            PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM => {
                Ok(Self::PlaceMultiplePostOnlyOrdersWithFreeFunds(
                    PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs::deserialize(&mut reader)?,
                ))
            }
            INITIALIZE_MARKET_IX_DISCM => Ok(Self::InitializeMarket(
                InitializeMarketIxArgs::deserialize(&mut reader)?,
            )),
            CLAIM_AUTHORITY_IX_DISCM => Ok(Self::ClaimAuthority),
            NAME_SUCCESSOR_IX_DISCM => Ok(Self::NameSuccessor(NameSuccessorIxArgs::deserialize(
                &mut reader,
            )?)),
            CHANGE_MARKET_STATUS_IX_DISCM => Ok(Self::ChangeMarketStatus(
                ChangeMarketStatusIxArgs::deserialize(&mut reader)?,
            )),
            CHANGE_SEAT_STATUS_IX_DISCM => Ok(Self::ChangeSeatStatus(
                ChangeSeatStatusIxArgs::deserialize(&mut reader)?,
            )),
            REQUEST_SEAT_AUTHORIZED_IX_DISCM => Ok(Self::RequestSeatAuthorized),
            EVICT_SEAT_IX_DISCM => Ok(Self::EvictSeat),
            FORCE_CANCEL_ORDERS_IX_DISCM => Ok(Self::ForceCancelOrders(
                ForceCancelOrdersIxArgs::deserialize(&mut reader)?,
            )),
            COLLECT_FEES_IX_DISCM => Ok(Self::CollectFees),
            CHANGE_FEE_RECIPIENT_IX_DISCM => Ok(Self::ChangeFeeRecipient),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Swap(args) => {
                writer.write_all(&[SWAP_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SwapWithFreeFunds(args) => {
                writer.write_all(&[SWAP_WITH_FREE_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PlaceLimitOrder(args) => {
                writer.write_all(&[PLACE_LIMIT_ORDER_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PlaceLimitOrderWithFreeFunds(args) => {
                writer.write_all(&[PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ReduceOrder(args) => {
                writer.write_all(&[REDUCE_ORDER_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ReduceOrderWithFreeFunds(args) => {
                writer.write_all(&[REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CancelAllOrders => writer.write_all(&[CANCEL_ALL_ORDERS_IX_DISCM]),
            Self::CancelAllOrdersWithFreeFunds => {
                writer.write_all(&[CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM])
            }
            Self::CancelUpTo(args) => {
                writer.write_all(&[CANCEL_UP_TO_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CancelUpToWithFreeFunds(args) => {
                writer.write_all(&[CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CancelMultipleOrdersById(args) => {
                writer.write_all(&[CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CancelMultipleOrdersByIdWithFreeFunds(args) => {
                writer.write_all(&[CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::WithdrawFunds(args) => {
                writer.write_all(&[WITHDRAW_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::DepositFunds(args) => {
                writer.write_all(&[DEPOSIT_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::RequestSeat => writer.write_all(&[REQUEST_SEAT_IX_DISCM]),
            Self::Log => writer.write_all(&[LOG_IX_DISCM]),
            Self::PlaceMultiplePostOnlyOrders(args) => {
                writer.write_all(&[PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::PlaceMultiplePostOnlyOrdersWithFreeFunds(args) => {
                writer.write_all(&[PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::InitializeMarket(args) => {
                writer.write_all(&[INITIALIZE_MARKET_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ClaimAuthority => writer.write_all(&[CLAIM_AUTHORITY_IX_DISCM]),
            Self::NameSuccessor(args) => {
                writer.write_all(&[NAME_SUCCESSOR_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ChangeMarketStatus(args) => {
                writer.write_all(&[CHANGE_MARKET_STATUS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::ChangeSeatStatus(args) => {
                writer.write_all(&[CHANGE_SEAT_STATUS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::RequestSeatAuthorized => writer.write_all(&[REQUEST_SEAT_AUTHORIZED_IX_DISCM]),
            Self::EvictSeat => writer.write_all(&[EVICT_SEAT_IX_DISCM]),
            Self::ForceCancelOrders(args) => {
                writer.write_all(&[FORCE_CANCEL_ORDERS_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::CollectFees => writer.write_all(&[COLLECT_FEES_IX_DISCM]),
            Self::ChangeFeeRecipient => writer.write_all(&[CHANGE_FEE_RECIPIENT_IX_DISCM]),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
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
impl From<SwapAccounts<'_, '_>> for SwapKeys {
    fn from(accounts: SwapAccounts) -> Self {
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
impl From<SwapKeys> for [AccountMeta; SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<SwapAccounts<'_, 'info>> for [AccountInfo<'info>; SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: SwapAccounts<'_, 'info>) -> Self {
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
pub const SWAP_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapIxData(pub SwapIxArgs);
impl From<SwapIxArgs> for SwapIxData {
    fn from(args: SwapIxArgs) -> Self {
        Self(args)
    }
}
impl SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapKeys,
    args: SwapIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_ix(keys: SwapKeys, args: SwapIxArgs) -> std::io::Result<Instruction> {
    swap_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_invoke(accounts: SwapAccounts<'_, '_>, args: SwapIxArgs) -> ProgramResult {
    swap_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapKeys = accounts.into();
    let ix = swap_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_invoke_signed(
    accounts: SwapAccounts<'_, '_>,
    args: SwapIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_verify_account_keys(
    accounts: SwapAccounts<'_, '_>,
    keys: SwapKeys,
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
pub fn swap_verify_writable_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_verify_signer_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_verify_account_privileges<'me, 'info>(
    accounts: SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_verify_writable_privileges(accounts)?;
    swap_verify_signer_privileges(accounts)?;
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
impl From<SwapWithFreeFundsAccounts<'_, '_>> for SwapWithFreeFundsKeys {
    fn from(accounts: SwapWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<SwapWithFreeFundsKeys> for [AccountMeta; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: SwapWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<SwapWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SwapWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const SWAP_WITH_FREE_FUNDS_IX_DISCM: u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapWithFreeFundsIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapWithFreeFundsIxData(pub SwapWithFreeFundsIxArgs);
impl From<SwapWithFreeFundsIxArgs> for SwapWithFreeFundsIxData {
    fn from(args: SwapWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl SwapWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SWAP_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SWAP_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SwapWithFreeFundsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SWAP_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn swap_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: SwapWithFreeFundsKeys,
    args: SwapWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SWAP_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SwapWithFreeFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn swap_with_free_funds_ix(
    keys: SwapWithFreeFundsKeys,
    args: SwapWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    swap_with_free_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn swap_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithFreeFundsAccounts<'_, '_>,
    args: SwapWithFreeFundsIxArgs,
) -> ProgramResult {
    let keys: SwapWithFreeFundsKeys = accounts.into();
    let ix = swap_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn swap_with_free_funds_invoke(
    accounts: SwapWithFreeFundsAccounts<'_, '_>,
    args: SwapWithFreeFundsIxArgs,
) -> ProgramResult {
    swap_with_free_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn swap_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SwapWithFreeFundsAccounts<'_, '_>,
    args: SwapWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SwapWithFreeFundsKeys = accounts.into();
    let ix = swap_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn swap_with_free_funds_invoke_signed(
    accounts: SwapWithFreeFundsAccounts<'_, '_>,
    args: SwapWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    swap_with_free_funds_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn swap_with_free_funds_verify_account_keys(
    accounts: SwapWithFreeFundsAccounts<'_, '_>,
    keys: SwapWithFreeFundsKeys,
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
pub fn swap_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: SwapWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn swap_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: SwapWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn swap_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: SwapWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    swap_with_free_funds_verify_writable_privileges(accounts)?;
    swap_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<PlaceLimitOrderAccounts<'_, '_>> for PlaceLimitOrderKeys {
    fn from(accounts: PlaceLimitOrderAccounts) -> Self {
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
impl From<PlaceLimitOrderKeys> for [AccountMeta; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: PlaceLimitOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<PlaceLimitOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PlaceLimitOrderAccounts<'_, 'info>) -> Self {
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
pub const PLACE_LIMIT_ORDER_IX_DISCM: u8 = 2u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceLimitOrderIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderIxData(pub PlaceLimitOrderIxArgs);
impl From<PlaceLimitOrderIxArgs> for PlaceLimitOrderIxData {
    fn from(args: PlaceLimitOrderIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceLimitOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PLACE_LIMIT_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_LIMIT_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceLimitOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_LIMIT_ORDER_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_limit_order_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceLimitOrderKeys,
    args: PlaceLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_LIMIT_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceLimitOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_limit_order_ix(
    keys: PlaceLimitOrderKeys,
    args: PlaceLimitOrderIxArgs,
) -> std::io::Result<Instruction> {
    place_limit_order_ix_with_program_id(crate::ID, keys, args)
}
pub fn place_limit_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
) -> ProgramResult {
    let keys: PlaceLimitOrderKeys = accounts.into();
    let ix = place_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_limit_order_invoke(
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
) -> ProgramResult {
    place_limit_order_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn place_limit_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceLimitOrderKeys = accounts.into();
    let ix = place_limit_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_limit_order_invoke_signed(
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    args: PlaceLimitOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_limit_order_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn place_limit_order_verify_account_keys(
    accounts: PlaceLimitOrderAccounts<'_, '_>,
    keys: PlaceLimitOrderKeys,
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
pub fn place_limit_order_verify_writable_privileges<'me, 'info>(
    accounts: PlaceLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_limit_order_verify_signer_privileges<'me, 'info>(
    accounts: PlaceLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_limit_order_verify_account_privileges<'me, 'info>(
    accounts: PlaceLimitOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_limit_order_verify_writable_privileges(accounts)?;
    place_limit_order_verify_signer_privileges(accounts)?;
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
impl From<PlaceLimitOrderWithFreeFundsAccounts<'_, '_>> for PlaceLimitOrderWithFreeFundsKeys {
    fn from(accounts: PlaceLimitOrderWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<PlaceLimitOrderWithFreeFundsKeys>
    for [AccountMeta; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: PlaceLimitOrderWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<PlaceLimitOrderWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PlaceLimitOrderWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM: u8 = 3u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceLimitOrderWithFreeFundsIxArgs {
    pub order_packet: OrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceLimitOrderWithFreeFundsIxData(pub PlaceLimitOrderWithFreeFundsIxArgs);
impl From<PlaceLimitOrderWithFreeFundsIxArgs> for PlaceLimitOrderWithFreeFundsIxData {
    fn from(args: PlaceLimitOrderWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceLimitOrderWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceLimitOrderWithFreeFundsIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_limit_order_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceLimitOrderWithFreeFundsKeys,
    args: PlaceLimitOrderWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_LIMIT_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceLimitOrderWithFreeFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_limit_order_with_free_funds_ix(
    keys: PlaceLimitOrderWithFreeFundsKeys,
    args: PlaceLimitOrderWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    place_limit_order_with_free_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn place_limit_order_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
    args: PlaceLimitOrderWithFreeFundsIxArgs,
) -> ProgramResult {
    let keys: PlaceLimitOrderWithFreeFundsKeys = accounts.into();
    let ix = place_limit_order_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_limit_order_with_free_funds_invoke(
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
    args: PlaceLimitOrderWithFreeFundsIxArgs,
) -> ProgramResult {
    place_limit_order_with_free_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn place_limit_order_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
    args: PlaceLimitOrderWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceLimitOrderWithFreeFundsKeys = accounts.into();
    let ix = place_limit_order_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_limit_order_with_free_funds_invoke_signed(
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
    args: PlaceLimitOrderWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_limit_order_with_free_funds_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_limit_order_with_free_funds_verify_account_keys(
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'_, '_>,
    keys: PlaceLimitOrderWithFreeFundsKeys,
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
pub fn place_limit_order_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_limit_order_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_limit_order_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: PlaceLimitOrderWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_limit_order_with_free_funds_verify_writable_privileges(accounts)?;
    place_limit_order_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<ReduceOrderAccounts<'_, '_>> for ReduceOrderKeys {
    fn from(accounts: ReduceOrderAccounts) -> Self {
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
impl From<ReduceOrderKeys> for [AccountMeta; REDUCE_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: ReduceOrderKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<ReduceOrderAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDUCE_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ReduceOrderAccounts<'_, 'info>) -> Self {
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
pub const REDUCE_ORDER_IX_DISCM: u8 = 4u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReduceOrderIxArgs {
    pub params: ReduceOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReduceOrderIxData(pub ReduceOrderIxArgs);
impl From<ReduceOrderIxArgs> for ReduceOrderIxData {
    fn from(args: ReduceOrderIxArgs) -> Self {
        Self(args)
    }
}
impl ReduceOrderIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REDUCE_ORDER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDUCE_ORDER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReduceOrderIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REDUCE_ORDER_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reduce_order_ix_with_program_id(
    program_id: Pubkey,
    keys: ReduceOrderKeys,
    args: ReduceOrderIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDUCE_ORDER_IX_ACCOUNTS_LEN] = keys.into();
    let data: ReduceOrderIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reduce_order_ix(
    keys: ReduceOrderKeys,
    args: ReduceOrderIxArgs,
) -> std::io::Result<Instruction> {
    reduce_order_ix_with_program_id(crate::ID, keys, args)
}
pub fn reduce_order_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ReduceOrderAccounts<'_, '_>,
    args: ReduceOrderIxArgs,
) -> ProgramResult {
    let keys: ReduceOrderKeys = accounts.into();
    let ix = reduce_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn reduce_order_invoke(
    accounts: ReduceOrderAccounts<'_, '_>,
    args: ReduceOrderIxArgs,
) -> ProgramResult {
    reduce_order_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn reduce_order_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ReduceOrderAccounts<'_, '_>,
    args: ReduceOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ReduceOrderKeys = accounts.into();
    let ix = reduce_order_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reduce_order_invoke_signed(
    accounts: ReduceOrderAccounts<'_, '_>,
    args: ReduceOrderIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reduce_order_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn reduce_order_verify_account_keys(
    accounts: ReduceOrderAccounts<'_, '_>,
    keys: ReduceOrderKeys,
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
pub fn reduce_order_verify_writable_privileges<'me, 'info>(
    accounts: ReduceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reduce_order_verify_signer_privileges<'me, 'info>(
    accounts: ReduceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reduce_order_verify_account_privileges<'me, 'info>(
    accounts: ReduceOrderAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reduce_order_verify_writable_privileges(accounts)?;
    reduce_order_verify_signer_privileges(accounts)?;
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
impl From<ReduceOrderWithFreeFundsAccounts<'_, '_>> for ReduceOrderWithFreeFundsKeys {
    fn from(accounts: ReduceOrderWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<ReduceOrderWithFreeFundsKeys>
    for [AccountMeta; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: ReduceOrderWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: true,
            },
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
impl<'info> From<ReduceOrderWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ReduceOrderWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM: u8 = 5u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReduceOrderWithFreeFundsIxArgs {
    pub params: ReduceOrderParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReduceOrderWithFreeFundsIxData(pub ReduceOrderWithFreeFundsIxArgs);
impl From<ReduceOrderWithFreeFundsIxArgs> for ReduceOrderWithFreeFundsIxData {
    fn from(args: ReduceOrderWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl ReduceOrderWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReduceOrderWithFreeFundsIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REDUCE_ORDER_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn reduce_order_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: ReduceOrderWithFreeFundsKeys,
    args: ReduceOrderWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDUCE_ORDER_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ReduceOrderWithFreeFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reduce_order_with_free_funds_ix(
    keys: ReduceOrderWithFreeFundsKeys,
    args: ReduceOrderWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    reduce_order_with_free_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn reduce_order_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ReduceOrderWithFreeFundsAccounts<'_, '_>,
    args: ReduceOrderWithFreeFundsIxArgs,
) -> ProgramResult {
    let keys: ReduceOrderWithFreeFundsKeys = accounts.into();
    let ix = reduce_order_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn reduce_order_with_free_funds_invoke(
    accounts: ReduceOrderWithFreeFundsAccounts<'_, '_>,
    args: ReduceOrderWithFreeFundsIxArgs,
) -> ProgramResult {
    reduce_order_with_free_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn reduce_order_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ReduceOrderWithFreeFundsAccounts<'_, '_>,
    args: ReduceOrderWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ReduceOrderWithFreeFundsKeys = accounts.into();
    let ix = reduce_order_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn reduce_order_with_free_funds_invoke_signed(
    accounts: ReduceOrderWithFreeFundsAccounts<'_, '_>,
    args: ReduceOrderWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    reduce_order_with_free_funds_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn reduce_order_with_free_funds_verify_account_keys(
    accounts: ReduceOrderWithFreeFundsAccounts<'_, '_>,
    keys: ReduceOrderWithFreeFundsKeys,
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
pub fn reduce_order_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: ReduceOrderWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.trader] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn reduce_order_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: ReduceOrderWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn reduce_order_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: ReduceOrderWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    reduce_order_with_free_funds_verify_writable_privileges(accounts)?;
    reduce_order_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<CancelAllOrdersAccounts<'_, '_>> for CancelAllOrdersKeys {
    fn from(accounts: CancelAllOrdersAccounts) -> Self {
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
impl From<CancelAllOrdersKeys> for [AccountMeta; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelAllOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<CancelAllOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CancelAllOrdersAccounts<'_, 'info>) -> Self {
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
pub const CANCEL_ALL_ORDERS_IX_DISCM: u8 = 6u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAllOrdersIxData;
impl CancelAllOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CANCEL_ALL_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ALL_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_ALL_ORDERS_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_all_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelAllOrdersKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ALL_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelAllOrdersIxData.try_to_vec()?,
    })
}
pub fn cancel_all_orders_ix(keys: CancelAllOrdersKeys) -> std::io::Result<Instruction> {
    cancel_all_orders_ix_with_program_id(crate::ID, keys)
}
pub fn cancel_all_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllOrdersAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelAllOrdersKeys = accounts.into();
    let ix = cancel_all_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_all_orders_invoke(accounts: CancelAllOrdersAccounts<'_, '_>) -> ProgramResult {
    cancel_all_orders_invoke_with_program_id(crate::ID, accounts)
}
pub fn cancel_all_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelAllOrdersKeys = accounts.into();
    let ix = cancel_all_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_all_orders_invoke_signed(
    accounts: CancelAllOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_all_orders_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn cancel_all_orders_verify_account_keys(
    accounts: CancelAllOrdersAccounts<'_, '_>,
    keys: CancelAllOrdersKeys,
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
pub fn cancel_all_orders_verify_writable_privileges<'me, 'info>(
    accounts: CancelAllOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_verify_signer_privileges<'me, 'info>(
    accounts: CancelAllOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_verify_account_privileges<'me, 'info>(
    accounts: CancelAllOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_all_orders_verify_writable_privileges(accounts)?;
    cancel_all_orders_verify_signer_privileges(accounts)?;
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
impl From<CancelAllOrdersWithFreeFundsAccounts<'_, '_>> for CancelAllOrdersWithFreeFundsKeys {
    fn from(accounts: CancelAllOrdersWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<CancelAllOrdersWithFreeFundsKeys>
    for [AccountMeta; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: CancelAllOrdersWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
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
impl<'info> From<CancelAllOrdersWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CancelAllOrdersWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CancelAllOrdersWithFreeFundsIxData;
impl CancelAllOrdersWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_all_orders_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelAllOrdersWithFreeFundsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_ALL_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelAllOrdersWithFreeFundsIxData.try_to_vec()?,
    })
}
pub fn cancel_all_orders_with_free_funds_ix(
    keys: CancelAllOrdersWithFreeFundsKeys,
) -> std::io::Result<Instruction> {
    cancel_all_orders_with_free_funds_ix_with_program_id(crate::ID, keys)
}
pub fn cancel_all_orders_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelAllOrdersWithFreeFundsKeys = accounts.into();
    let ix = cancel_all_orders_with_free_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_all_orders_with_free_funds_invoke(
    accounts: CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
) -> ProgramResult {
    cancel_all_orders_with_free_funds_invoke_with_program_id(crate::ID, accounts)
}
pub fn cancel_all_orders_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelAllOrdersWithFreeFundsKeys = accounts.into();
    let ix = cancel_all_orders_with_free_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_all_orders_with_free_funds_invoke_signed(
    accounts: CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_all_orders_with_free_funds_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn cancel_all_orders_with_free_funds_verify_account_keys(
    accounts: CancelAllOrdersWithFreeFundsAccounts<'_, '_>,
    keys: CancelAllOrdersWithFreeFundsKeys,
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
pub fn cancel_all_orders_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: CancelAllOrdersWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: CancelAllOrdersWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_all_orders_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: CancelAllOrdersWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_all_orders_with_free_funds_verify_writable_privileges(accounts)?;
    cancel_all_orders_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<CancelUpToAccounts<'_, '_>> for CancelUpToKeys {
    fn from(accounts: CancelUpToAccounts) -> Self {
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
impl From<CancelUpToKeys> for [AccountMeta; CANCEL_UP_TO_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelUpToKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<CancelUpToAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_UP_TO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CancelUpToAccounts<'_, 'info>) -> Self {
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
pub const CANCEL_UP_TO_IX_DISCM: u8 = 8u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelUpToIxArgs {
    pub params: CancelUpToParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUpToIxData(pub CancelUpToIxArgs);
impl From<CancelUpToIxArgs> for CancelUpToIxData {
    fn from(args: CancelUpToIxArgs) -> Self {
        Self(args)
    }
}
impl CancelUpToIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CANCEL_UP_TO_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_UP_TO_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelUpToIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_UP_TO_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_up_to_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelUpToKeys,
    args: CancelUpToIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_UP_TO_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelUpToIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_up_to_ix(
    keys: CancelUpToKeys,
    args: CancelUpToIxArgs,
) -> std::io::Result<Instruction> {
    cancel_up_to_ix_with_program_id(crate::ID, keys, args)
}
pub fn cancel_up_to_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelUpToAccounts<'_, '_>,
    args: CancelUpToIxArgs,
) -> ProgramResult {
    let keys: CancelUpToKeys = accounts.into();
    let ix = cancel_up_to_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_up_to_invoke(
    accounts: CancelUpToAccounts<'_, '_>,
    args: CancelUpToIxArgs,
) -> ProgramResult {
    cancel_up_to_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn cancel_up_to_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelUpToAccounts<'_, '_>,
    args: CancelUpToIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelUpToKeys = accounts.into();
    let ix = cancel_up_to_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_up_to_invoke_signed(
    accounts: CancelUpToAccounts<'_, '_>,
    args: CancelUpToIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_up_to_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn cancel_up_to_verify_account_keys(
    accounts: CancelUpToAccounts<'_, '_>,
    keys: CancelUpToKeys,
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
pub fn cancel_up_to_verify_writable_privileges<'me, 'info>(
    accounts: CancelUpToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_up_to_verify_signer_privileges<'me, 'info>(
    accounts: CancelUpToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_up_to_verify_account_privileges<'me, 'info>(
    accounts: CancelUpToAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_up_to_verify_writable_privileges(accounts)?;
    cancel_up_to_verify_signer_privileges(accounts)?;
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
impl From<CancelUpToWithFreeFundsAccounts<'_, '_>> for CancelUpToWithFreeFundsKeys {
    fn from(accounts: CancelUpToWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<CancelUpToWithFreeFundsKeys>
    for [AccountMeta; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: CancelUpToWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
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
impl<'info> From<CancelUpToWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CancelUpToWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM: u8 = 9u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelUpToWithFreeFundsIxArgs {
    pub params: CancelUpToParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUpToWithFreeFundsIxData(pub CancelUpToWithFreeFundsIxArgs);
impl From<CancelUpToWithFreeFundsIxArgs> for CancelUpToWithFreeFundsIxData {
    fn from(args: CancelUpToWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl CancelUpToWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelUpToWithFreeFundsIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_UP_TO_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_up_to_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelUpToWithFreeFundsKeys,
    args: CancelUpToWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_UP_TO_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelUpToWithFreeFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_up_to_with_free_funds_ix(
    keys: CancelUpToWithFreeFundsKeys,
    args: CancelUpToWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    cancel_up_to_with_free_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn cancel_up_to_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelUpToWithFreeFundsAccounts<'_, '_>,
    args: CancelUpToWithFreeFundsIxArgs,
) -> ProgramResult {
    let keys: CancelUpToWithFreeFundsKeys = accounts.into();
    let ix = cancel_up_to_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_up_to_with_free_funds_invoke(
    accounts: CancelUpToWithFreeFundsAccounts<'_, '_>,
    args: CancelUpToWithFreeFundsIxArgs,
) -> ProgramResult {
    cancel_up_to_with_free_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn cancel_up_to_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelUpToWithFreeFundsAccounts<'_, '_>,
    args: CancelUpToWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelUpToWithFreeFundsKeys = accounts.into();
    let ix = cancel_up_to_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_up_to_with_free_funds_invoke_signed(
    accounts: CancelUpToWithFreeFundsAccounts<'_, '_>,
    args: CancelUpToWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_up_to_with_free_funds_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn cancel_up_to_with_free_funds_verify_account_keys(
    accounts: CancelUpToWithFreeFundsAccounts<'_, '_>,
    keys: CancelUpToWithFreeFundsKeys,
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
pub fn cancel_up_to_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: CancelUpToWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_up_to_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: CancelUpToWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_up_to_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: CancelUpToWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_up_to_with_free_funds_verify_writable_privileges(accounts)?;
    cancel_up_to_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<CancelMultipleOrdersByIdAccounts<'_, '_>> for CancelMultipleOrdersByIdKeys {
    fn from(accounts: CancelMultipleOrdersByIdAccounts) -> Self {
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
impl From<CancelMultipleOrdersByIdKeys>
    for [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]
{
    fn from(keys: CancelMultipleOrdersByIdKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<CancelMultipleOrdersByIdAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CancelMultipleOrdersByIdAccounts<'_, 'info>) -> Self {
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
pub const CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM: u8 = 10u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelMultipleOrdersByIdIxArgs {
    pub params: CancelMultipleOrdersByIdParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelMultipleOrdersByIdIxData(pub CancelMultipleOrdersByIdIxArgs);
impl From<CancelMultipleOrdersByIdIxArgs> for CancelMultipleOrdersByIdIxData {
    fn from(args: CancelMultipleOrdersByIdIxArgs) -> Self {
        Self(args)
    }
}
impl CancelMultipleOrdersByIdIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CancelMultipleOrdersByIdIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_MULTIPLE_ORDERS_BY_ID_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_multiple_orders_by_id_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelMultipleOrdersByIdKeys,
    args: CancelMultipleOrdersByIdIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_IX_ACCOUNTS_LEN] = keys.into();
    let data: CancelMultipleOrdersByIdIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_multiple_orders_by_id_ix(
    keys: CancelMultipleOrdersByIdKeys,
    args: CancelMultipleOrdersByIdIxArgs,
) -> std::io::Result<Instruction> {
    cancel_multiple_orders_by_id_ix_with_program_id(crate::ID, keys, args)
}
pub fn cancel_multiple_orders_by_id_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelMultipleOrdersByIdAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdIxArgs,
) -> ProgramResult {
    let keys: CancelMultipleOrdersByIdKeys = accounts.into();
    let ix = cancel_multiple_orders_by_id_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_multiple_orders_by_id_invoke(
    accounts: CancelMultipleOrdersByIdAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdIxArgs,
) -> ProgramResult {
    cancel_multiple_orders_by_id_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn cancel_multiple_orders_by_id_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelMultipleOrdersByIdAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelMultipleOrdersByIdKeys = accounts.into();
    let ix = cancel_multiple_orders_by_id_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_multiple_orders_by_id_invoke_signed(
    accounts: CancelMultipleOrdersByIdAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_multiple_orders_by_id_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn cancel_multiple_orders_by_id_verify_account_keys(
    accounts: CancelMultipleOrdersByIdAccounts<'_, '_>,
    keys: CancelMultipleOrdersByIdKeys,
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
pub fn cancel_multiple_orders_by_id_verify_writable_privileges<'me, 'info>(
    accounts: CancelMultipleOrdersByIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_multiple_orders_by_id_verify_signer_privileges<'me, 'info>(
    accounts: CancelMultipleOrdersByIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_multiple_orders_by_id_verify_account_privileges<'me, 'info>(
    accounts: CancelMultipleOrdersByIdAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_multiple_orders_by_id_verify_writable_privileges(accounts)?;
    cancel_multiple_orders_by_id_verify_signer_privileges(accounts)?;
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
impl From<CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>>
    for CancelMultipleOrdersByIdWithFreeFundsKeys
{
    fn from(accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
        }
    }
}
impl From<CancelMultipleOrdersByIdWithFreeFundsKeys>
    for [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: CancelMultipleOrdersByIdWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
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
impl<'info> From<CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM: u8 = 11u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelMultipleOrdersByIdWithFreeFundsIxArgs {
    pub params: CancelMultipleOrdersByIdParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelMultipleOrdersByIdWithFreeFundsIxData(
    pub CancelMultipleOrdersByIdWithFreeFundsIxArgs,
);
impl From<CancelMultipleOrdersByIdWithFreeFundsIxArgs>
    for CancelMultipleOrdersByIdWithFreeFundsIxData
{
    fn from(args: CancelMultipleOrdersByIdWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl CancelMultipleOrdersByIdWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
            CancelMultipleOrdersByIdWithFreeFundsIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_multiple_orders_by_id_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelMultipleOrdersByIdWithFreeFundsKeys,
    args: CancelMultipleOrdersByIdWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_MULTIPLE_ORDERS_BY_ID_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        keys.into();
    let data: CancelMultipleOrdersByIdWithFreeFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_multiple_orders_by_id_with_free_funds_ix(
    keys: CancelMultipleOrdersByIdWithFreeFundsKeys,
    args: CancelMultipleOrdersByIdWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    cancel_multiple_orders_by_id_with_free_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn cancel_multiple_orders_by_id_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdWithFreeFundsIxArgs,
) -> ProgramResult {
    let keys: CancelMultipleOrdersByIdWithFreeFundsKeys = accounts.into();
    let ix =
        cancel_multiple_orders_by_id_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_multiple_orders_by_id_with_free_funds_invoke(
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdWithFreeFundsIxArgs,
) -> ProgramResult {
    cancel_multiple_orders_by_id_with_free_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn cancel_multiple_orders_by_id_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelMultipleOrdersByIdWithFreeFundsKeys = accounts.into();
    let ix =
        cancel_multiple_orders_by_id_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_multiple_orders_by_id_with_free_funds_invoke_signed(
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
    args: CancelMultipleOrdersByIdWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_multiple_orders_by_id_with_free_funds_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn cancel_multiple_orders_by_id_with_free_funds_verify_account_keys(
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'_, '_>,
    keys: CancelMultipleOrdersByIdWithFreeFundsKeys,
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
pub fn cancel_multiple_orders_by_id_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_multiple_orders_by_id_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_multiple_orders_by_id_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: CancelMultipleOrdersByIdWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_multiple_orders_by_id_with_free_funds_verify_writable_privileges(accounts)?;
    cancel_multiple_orders_by_id_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<WithdrawFundsAccounts<'_, '_>> for WithdrawFundsKeys {
    fn from(accounts: WithdrawFundsAccounts) -> Self {
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
impl From<WithdrawFundsKeys> for [AccountMeta; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<WithdrawFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: WithdrawFundsAccounts<'_, 'info>) -> Self {
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
pub const WITHDRAW_FUNDS_IX_DISCM: u8 = 12u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawFundsIxArgs {
    pub withdraw_funds_params: WithdrawParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawFundsIxData(pub WithdrawFundsIxArgs);
impl From<WithdrawFundsIxArgs> for WithdrawFundsIxData {
    fn from(args: WithdrawFundsIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != WITHDRAW_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawFundsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[WITHDRAW_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawFundsKeys,
    args: WithdrawFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_funds_ix(
    keys: WithdrawFundsKeys,
    args: WithdrawFundsIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFundsAccounts<'_, '_>,
    args: WithdrawFundsIxArgs,
) -> ProgramResult {
    let keys: WithdrawFundsKeys = accounts.into();
    let ix = withdraw_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_funds_invoke(
    accounts: WithdrawFundsAccounts<'_, '_>,
    args: WithdrawFundsIxArgs,
) -> ProgramResult {
    withdraw_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawFundsAccounts<'_, '_>,
    args: WithdrawFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawFundsKeys = accounts.into();
    let ix = withdraw_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_funds_invoke_signed(
    accounts: WithdrawFundsAccounts<'_, '_>,
    args: WithdrawFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_funds_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_funds_verify_account_keys(
    accounts: WithdrawFundsAccounts<'_, '_>,
    keys: WithdrawFundsKeys,
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
pub fn withdraw_funds_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_funds_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_funds_verify_account_privileges<'me, 'info>(
    accounts: WithdrawFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_funds_verify_writable_privileges(accounts)?;
    withdraw_funds_verify_signer_privileges(accounts)?;
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
impl From<DepositFundsAccounts<'_, '_>> for DepositFundsKeys {
    fn from(accounts: DepositFundsAccounts) -> Self {
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
impl From<DepositFundsKeys> for [AccountMeta; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<DepositFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DepositFundsAccounts<'_, 'info>) -> Self {
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
pub const DEPOSIT_FUNDS_IX_DISCM: u8 = 13u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositFundsIxArgs {
    pub deposit_funds_params: DepositParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositFundsIxData(pub DepositFundsIxArgs);
impl From<DepositFundsIxArgs> for DepositFundsIxData {
    fn from(args: DepositFundsIxArgs) -> Self {
        Self(args)
    }
}
impl DepositFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DEPOSIT_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositFundsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPOSIT_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositFundsKeys,
    args: DepositFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_funds_ix(
    keys: DepositFundsKeys,
    args: DepositFundsIxArgs,
) -> std::io::Result<Instruction> {
    deposit_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn deposit_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositFundsAccounts<'_, '_>,
    args: DepositFundsIxArgs,
) -> ProgramResult {
    let keys: DepositFundsKeys = accounts.into();
    let ix = deposit_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_funds_invoke(
    accounts: DepositFundsAccounts<'_, '_>,
    args: DepositFundsIxArgs,
) -> ProgramResult {
    deposit_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn deposit_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositFundsAccounts<'_, '_>,
    args: DepositFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositFundsKeys = accounts.into();
    let ix = deposit_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_funds_invoke_signed(
    accounts: DepositFundsAccounts<'_, '_>,
    args: DepositFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_funds_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn deposit_funds_verify_account_keys(
    accounts: DepositFundsAccounts<'_, '_>,
    keys: DepositFundsKeys,
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
pub fn deposit_funds_verify_writable_privileges<'me, 'info>(
    accounts: DepositFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_funds_verify_signer_privileges<'me, 'info>(
    accounts: DepositFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_funds_verify_account_privileges<'me, 'info>(
    accounts: DepositFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_funds_verify_writable_privileges(accounts)?;
    deposit_funds_verify_signer_privileges(accounts)?;
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
impl From<RequestSeatAccounts<'_, '_>> for RequestSeatKeys {
    fn from(accounts: RequestSeatAccounts) -> Self {
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
impl From<RequestSeatKeys> for [AccountMeta; REQUEST_SEAT_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestSeatKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<RequestSeatAccounts<'_, 'info>>
    for [AccountInfo<'info>; REQUEST_SEAT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RequestSeatAccounts<'_, 'info>) -> Self {
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
pub const REQUEST_SEAT_IX_DISCM: u8 = 14u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RequestSeatIxData;
impl RequestSeatIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REQUEST_SEAT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REQUEST_SEAT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REQUEST_SEAT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_seat_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestSeatKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_SEAT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RequestSeatIxData.try_to_vec()?,
    })
}
pub fn request_seat_ix(keys: RequestSeatKeys) -> std::io::Result<Instruction> {
    request_seat_ix_with_program_id(crate::ID, keys)
}
pub fn request_seat_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestSeatAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RequestSeatKeys = accounts.into();
    let ix = request_seat_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_seat_invoke(accounts: RequestSeatAccounts<'_, '_>) -> ProgramResult {
    request_seat_invoke_with_program_id(crate::ID, accounts)
}
pub fn request_seat_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestSeatAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestSeatKeys = accounts.into();
    let ix = request_seat_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_seat_invoke_signed(
    accounts: RequestSeatAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_seat_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn request_seat_verify_account_keys(
    accounts: RequestSeatAccounts<'_, '_>,
    keys: RequestSeatKeys,
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
pub fn request_seat_verify_writable_privileges<'me, 'info>(
    accounts: RequestSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.payer, accounts.seat] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_seat_verify_signer_privileges<'me, 'info>(
    accounts: RequestSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_seat_verify_account_privileges<'me, 'info>(
    accounts: RequestSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_seat_verify_writable_privileges(accounts)?;
    request_seat_verify_signer_privileges(accounts)?;
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
impl From<LogAccounts<'_, '_>> for LogKeys {
    fn from(accounts: LogAccounts) -> Self {
        Self {
            log_authority: *accounts.log_authority.key,
        }
    }
}
impl From<LogKeys> for [AccountMeta; LOG_IX_ACCOUNTS_LEN] {
    fn from(keys: LogKeys) -> Self {
        [AccountMeta {
            pubkey: keys.log_authority,
            is_signer: true,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; LOG_IX_ACCOUNTS_LEN]> for LogKeys {
    fn from(pubkeys: [Pubkey; LOG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            log_authority: pubkeys[0],
        }
    }
}
impl<'info> From<LogAccounts<'_, 'info>> for [AccountInfo<'info>; LOG_IX_ACCOUNTS_LEN] {
    fn from(accounts: LogAccounts<'_, 'info>) -> Self {
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
pub const LOG_IX_DISCM: u8 = 15u8;
#[derive(Clone, Debug, PartialEq)]
pub struct LogIxData;
impl LogIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != LOG_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LOG_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[LOG_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn log_ix_with_program_id(program_id: Pubkey, keys: LogKeys) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LOG_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LogIxData.try_to_vec()?,
    })
}
pub fn log_ix(keys: LogKeys) -> std::io::Result<Instruction> {
    log_ix_with_program_id(crate::ID, keys)
}
pub fn log_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LogAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LogKeys = accounts.into();
    let ix = log_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn log_invoke(accounts: LogAccounts<'_, '_>) -> ProgramResult {
    log_invoke_with_program_id(crate::ID, accounts)
}
pub fn log_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LogAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LogKeys = accounts.into();
    let ix = log_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn log_invoke_signed(accounts: LogAccounts<'_, '_>, seeds: &[&[&[u8]]]) -> ProgramResult {
    log_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn log_verify_account_keys(
    accounts: LogAccounts<'_, '_>,
    keys: LogKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.log_authority.key, &keys.log_authority)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn log_verify_signer_privileges<'me, 'info>(
    accounts: LogAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.log_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn log_verify_account_privileges<'me, 'info>(
    accounts: LogAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    log_verify_signer_privileges(accounts)?;
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
impl From<PlaceMultiplePostOnlyOrdersAccounts<'_, '_>> for PlaceMultiplePostOnlyOrdersKeys {
    fn from(accounts: PlaceMultiplePostOnlyOrdersAccounts) -> Self {
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
impl From<PlaceMultiplePostOnlyOrdersKeys>
    for [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(keys: PlaceMultiplePostOnlyOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<PlaceMultiplePostOnlyOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PlaceMultiplePostOnlyOrdersAccounts<'_, 'info>) -> Self {
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
pub const PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM: u8 = 16u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceMultiplePostOnlyOrdersIxArgs {
    pub multiple_order_packet: MultipleOrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceMultiplePostOnlyOrdersIxData(pub PlaceMultiplePostOnlyOrdersIxArgs);
impl From<PlaceMultiplePostOnlyOrdersIxArgs> for PlaceMultiplePostOnlyOrdersIxData {
    fn from(args: PlaceMultiplePostOnlyOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceMultiplePostOnlyOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PlaceMultiplePostOnlyOrdersIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_multiple_post_only_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceMultiplePostOnlyOrdersKeys,
    args: PlaceMultiplePostOnlyOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: PlaceMultiplePostOnlyOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_multiple_post_only_orders_ix(
    keys: PlaceMultiplePostOnlyOrdersKeys,
    args: PlaceMultiplePostOnlyOrdersIxArgs,
) -> std::io::Result<Instruction> {
    place_multiple_post_only_orders_ix_with_program_id(crate::ID, keys, args)
}
pub fn place_multiple_post_only_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersIxArgs,
) -> ProgramResult {
    let keys: PlaceMultiplePostOnlyOrdersKeys = accounts.into();
    let ix = place_multiple_post_only_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_multiple_post_only_orders_invoke(
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersIxArgs,
) -> ProgramResult {
    place_multiple_post_only_orders_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn place_multiple_post_only_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceMultiplePostOnlyOrdersKeys = accounts.into();
    let ix = place_multiple_post_only_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_multiple_post_only_orders_invoke_signed(
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_multiple_post_only_orders_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn place_multiple_post_only_orders_verify_account_keys(
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'_, '_>,
    keys: PlaceMultiplePostOnlyOrdersKeys,
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
pub fn place_multiple_post_only_orders_verify_writable_privileges<'me, 'info>(
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_multiple_post_only_orders_verify_signer_privileges<'me, 'info>(
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_multiple_post_only_orders_verify_account_privileges<'me, 'info>(
    accounts: PlaceMultiplePostOnlyOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_multiple_post_only_orders_verify_writable_privileges(accounts)?;
    place_multiple_post_only_orders_verify_signer_privileges(accounts)?;
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
impl From<PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>>
    for PlaceMultiplePostOnlyOrdersWithFreeFundsKeys
{
    fn from(accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<PlaceMultiplePostOnlyOrdersWithFreeFundsKeys>
    for [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, 'info>) -> Self {
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
pub const PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM: u8 = 17u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs {
    pub multiple_order_packet: MultipleOrderPacket,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsIxData(
    pub PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
);
impl From<PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs>
    for PlaceMultiplePostOnlyOrdersWithFreeFundsIxData
{
    fn from(args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs) -> Self {
        Self(args)
    }
}
impl PlaceMultiplePostOnlyOrdersWithFreeFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
            PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs::deserialize(&mut reader)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn place_multiple_post_only_orders_with_free_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys,
    args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PLACE_MULTIPLE_POST_ONLY_ORDERS_WITH_FREE_FUNDS_IX_ACCOUNTS_LEN] =
        keys.into();
    let data: PlaceMultiplePostOnlyOrdersWithFreeFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_multiple_post_only_orders_with_free_funds_ix(
    keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys,
    args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
) -> std::io::Result<Instruction> {
    place_multiple_post_only_orders_with_free_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn place_multiple_post_only_orders_with_free_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
) -> ProgramResult {
    let keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys = accounts.into();
    let ix =
        place_multiple_post_only_orders_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn place_multiple_post_only_orders_with_free_funds_invoke(
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
) -> ProgramResult {
    place_multiple_post_only_orders_with_free_funds_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn place_multiple_post_only_orders_with_free_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys = accounts.into();
    let ix =
        place_multiple_post_only_orders_with_free_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn place_multiple_post_only_orders_with_free_funds_invoke_signed(
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
    args: PlaceMultiplePostOnlyOrdersWithFreeFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    place_multiple_post_only_orders_with_free_funds_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn place_multiple_post_only_orders_with_free_funds_verify_account_keys(
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'_, '_>,
    keys: PlaceMultiplePostOnlyOrdersWithFreeFundsKeys,
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
pub fn place_multiple_post_only_orders_with_free_funds_verify_writable_privileges<'me, 'info>(
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn place_multiple_post_only_orders_with_free_funds_verify_signer_privileges<'me, 'info>(
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.trader] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn place_multiple_post_only_orders_with_free_funds_verify_account_privileges<'me, 'info>(
    accounts: PlaceMultiplePostOnlyOrdersWithFreeFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    place_multiple_post_only_orders_with_free_funds_verify_writable_privileges(accounts)?;
    place_multiple_post_only_orders_with_free_funds_verify_signer_privileges(accounts)?;
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
impl From<InitializeMarketAccounts<'_, '_>> for InitializeMarketKeys {
    fn from(accounts: InitializeMarketAccounts) -> Self {
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
impl From<InitializeMarketKeys> for [AccountMeta; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_creator,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<InitializeMarketAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeMarketAccounts<'_, 'info>) -> Self {
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
pub const INITIALIZE_MARKET_IX_DISCM: u8 = 100u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeMarketIxArgs {
    pub initialize_params: InitializeParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeMarketIxData(pub InitializeMarketIxArgs);
impl From<InitializeMarketIxArgs> for InitializeMarketIxData {
    fn from(args: InitializeMarketIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_MARKET_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_MARKET_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeMarketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_MARKET_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_market_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeMarketKeys,
    args: InitializeMarketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeMarketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_market_ix(
    keys: InitializeMarketKeys,
    args: InitializeMarketIxArgs,
) -> std::io::Result<Instruction> {
    initialize_market_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
) -> ProgramResult {
    let keys: InitializeMarketKeys = accounts.into();
    let ix = initialize_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_market_invoke(
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
) -> ProgramResult {
    initialize_market_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeMarketKeys = accounts.into();
    let ix = initialize_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_market_invoke_signed(
    accounts: InitializeMarketAccounts<'_, '_>,
    args: InitializeMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_market_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_market_verify_account_keys(
    accounts: InitializeMarketAccounts<'_, '_>,
    keys: InitializeMarketKeys,
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
pub fn initialize_market_verify_writable_privileges<'me, 'info>(
    accounts: InitializeMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.market_creator,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_market_verify_signer_privileges<'me, 'info>(
    accounts: InitializeMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_market_verify_account_privileges<'me, 'info>(
    accounts: InitializeMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_market_verify_writable_privileges(accounts)?;
    initialize_market_verify_signer_privileges(accounts)?;
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
impl From<ClaimAuthorityAccounts<'_, '_>> for ClaimAuthorityKeys {
    fn from(accounts: ClaimAuthorityAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            successor: *accounts.successor.key,
        }
    }
}
impl From<ClaimAuthorityKeys> for [AccountMeta; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.successor,
                is_signer: true,
                is_writable: false,
            },
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
impl<'info> From<ClaimAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ClaimAuthorityAccounts<'_, 'info>) -> Self {
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
pub const CLAIM_AUTHORITY_IX_DISCM: u8 = 101u8;
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimAuthorityIxData;
impl ClaimAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLAIM_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLAIM_AUTHORITY_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimAuthorityIxData.try_to_vec()?,
    })
}
pub fn claim_authority_ix(keys: ClaimAuthorityKeys) -> std::io::Result<Instruction> {
    claim_authority_ix_with_program_id(crate::ID, keys)
}
pub fn claim_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimAuthorityKeys = accounts.into();
    let ix = claim_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_authority_invoke(accounts: ClaimAuthorityAccounts<'_, '_>) -> ProgramResult {
    claim_authority_invoke_with_program_id(crate::ID, accounts)
}
pub fn claim_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimAuthorityKeys = accounts.into();
    let ix = claim_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_authority_invoke_signed(
    accounts: ClaimAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_authority_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn claim_authority_verify_account_keys(
    accounts: ClaimAuthorityAccounts<'_, '_>,
    keys: ClaimAuthorityKeys,
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
pub fn claim_authority_verify_writable_privileges<'me, 'info>(
    accounts: ClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_authority_verify_signer_privileges<'me, 'info>(
    accounts: ClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.successor] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_authority_verify_account_privileges<'me, 'info>(
    accounts: ClaimAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_authority_verify_writable_privileges(accounts)?;
    claim_authority_verify_signer_privileges(accounts)?;
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
impl From<NameSuccessorAccounts<'_, '_>> for NameSuccessorKeys {
    fn from(accounts: NameSuccessorAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
        }
    }
}
impl From<NameSuccessorKeys> for [AccountMeta; NAME_SUCCESSOR_IX_ACCOUNTS_LEN] {
    fn from(keys: NameSuccessorKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
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
impl<'info> From<NameSuccessorAccounts<'_, 'info>>
    for [AccountInfo<'info>; NAME_SUCCESSOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: NameSuccessorAccounts<'_, 'info>) -> Self {
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
pub const NAME_SUCCESSOR_IX_DISCM: u8 = 102u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameSuccessorIxArgs {
    pub successor: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NameSuccessorIxData(pub NameSuccessorIxArgs);
impl From<NameSuccessorIxArgs> for NameSuccessorIxData {
    fn from(args: NameSuccessorIxArgs) -> Self {
        Self(args)
    }
}
impl NameSuccessorIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NAME_SUCCESSOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NAME_SUCCESSOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(NameSuccessorIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NAME_SUCCESSOR_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn name_successor_ix_with_program_id(
    program_id: Pubkey,
    keys: NameSuccessorKeys,
    args: NameSuccessorIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NAME_SUCCESSOR_IX_ACCOUNTS_LEN] = keys.into();
    let data: NameSuccessorIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn name_successor_ix(
    keys: NameSuccessorKeys,
    args: NameSuccessorIxArgs,
) -> std::io::Result<Instruction> {
    name_successor_ix_with_program_id(crate::ID, keys, args)
}
pub fn name_successor_invoke_with_program_id(
    program_id: Pubkey,
    accounts: NameSuccessorAccounts<'_, '_>,
    args: NameSuccessorIxArgs,
) -> ProgramResult {
    let keys: NameSuccessorKeys = accounts.into();
    let ix = name_successor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn name_successor_invoke(
    accounts: NameSuccessorAccounts<'_, '_>,
    args: NameSuccessorIxArgs,
) -> ProgramResult {
    name_successor_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn name_successor_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: NameSuccessorAccounts<'_, '_>,
    args: NameSuccessorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NameSuccessorKeys = accounts.into();
    let ix = name_successor_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn name_successor_invoke_signed(
    accounts: NameSuccessorAccounts<'_, '_>,
    args: NameSuccessorIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    name_successor_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn name_successor_verify_account_keys(
    accounts: NameSuccessorAccounts<'_, '_>,
    keys: NameSuccessorKeys,
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
pub fn name_successor_verify_writable_privileges<'me, 'info>(
    accounts: NameSuccessorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn name_successor_verify_signer_privileges<'me, 'info>(
    accounts: NameSuccessorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn name_successor_verify_account_privileges<'me, 'info>(
    accounts: NameSuccessorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    name_successor_verify_writable_privileges(accounts)?;
    name_successor_verify_signer_privileges(accounts)?;
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
impl From<ChangeMarketStatusAccounts<'_, '_>> for ChangeMarketStatusKeys {
    fn from(accounts: ChangeMarketStatusAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
        }
    }
}
impl From<ChangeMarketStatusKeys> for [AccountMeta; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeMarketStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
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
impl<'info> From<ChangeMarketStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ChangeMarketStatusAccounts<'_, 'info>) -> Self {
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
pub const CHANGE_MARKET_STATUS_IX_DISCM: u8 = 103u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeMarketStatusIxArgs {
    pub market_status: MarketStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeMarketStatusIxData(pub ChangeMarketStatusIxArgs);
impl From<ChangeMarketStatusIxArgs> for ChangeMarketStatusIxData {
    fn from(args: ChangeMarketStatusIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeMarketStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CHANGE_MARKET_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_MARKET_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeMarketStatusIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_MARKET_STATUS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_market_status_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeMarketStatusKeys,
    args: ChangeMarketStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_MARKET_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeMarketStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_market_status_ix(
    keys: ChangeMarketStatusKeys,
    args: ChangeMarketStatusIxArgs,
) -> std::io::Result<Instruction> {
    change_market_status_ix_with_program_id(crate::ID, keys, args)
}
pub fn change_market_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeMarketStatusAccounts<'_, '_>,
    args: ChangeMarketStatusIxArgs,
) -> ProgramResult {
    let keys: ChangeMarketStatusKeys = accounts.into();
    let ix = change_market_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_market_status_invoke(
    accounts: ChangeMarketStatusAccounts<'_, '_>,
    args: ChangeMarketStatusIxArgs,
) -> ProgramResult {
    change_market_status_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn change_market_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeMarketStatusAccounts<'_, '_>,
    args: ChangeMarketStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeMarketStatusKeys = accounts.into();
    let ix = change_market_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_market_status_invoke_signed(
    accounts: ChangeMarketStatusAccounts<'_, '_>,
    args: ChangeMarketStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_market_status_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn change_market_status_verify_account_keys(
    accounts: ChangeMarketStatusAccounts<'_, '_>,
    keys: ChangeMarketStatusKeys,
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
pub fn change_market_status_verify_writable_privileges<'me, 'info>(
    accounts: ChangeMarketStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_market_status_verify_signer_privileges<'me, 'info>(
    accounts: ChangeMarketStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_market_status_verify_account_privileges<'me, 'info>(
    accounts: ChangeMarketStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_market_status_verify_writable_privileges(accounts)?;
    change_market_status_verify_signer_privileges(accounts)?;
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
impl From<ChangeSeatStatusAccounts<'_, '_>> for ChangeSeatStatusKeys {
    fn from(accounts: ChangeSeatStatusAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            seat: *accounts.seat.key,
        }
    }
}
impl From<ChangeSeatStatusKeys> for [AccountMeta; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeSeatStatusKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: true,
            },
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
impl<'info> From<ChangeSeatStatusAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ChangeSeatStatusAccounts<'_, 'info>) -> Self {
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
pub const CHANGE_SEAT_STATUS_IX_DISCM: u8 = 104u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeSeatStatusIxArgs {
    pub approval_status: SeatApprovalStatus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeSeatStatusIxData(pub ChangeSeatStatusIxArgs);
impl From<ChangeSeatStatusIxArgs> for ChangeSeatStatusIxData {
    fn from(args: ChangeSeatStatusIxArgs) -> Self {
        Self(args)
    }
}
impl ChangeSeatStatusIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CHANGE_SEAT_STATUS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_SEAT_STATUS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeSeatStatusIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_SEAT_STATUS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_seat_status_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeSeatStatusKeys,
    args: ChangeSeatStatusIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_SEAT_STATUS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ChangeSeatStatusIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_seat_status_ix(
    keys: ChangeSeatStatusKeys,
    args: ChangeSeatStatusIxArgs,
) -> std::io::Result<Instruction> {
    change_seat_status_ix_with_program_id(crate::ID, keys, args)
}
pub fn change_seat_status_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeSeatStatusAccounts<'_, '_>,
    args: ChangeSeatStatusIxArgs,
) -> ProgramResult {
    let keys: ChangeSeatStatusKeys = accounts.into();
    let ix = change_seat_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_seat_status_invoke(
    accounts: ChangeSeatStatusAccounts<'_, '_>,
    args: ChangeSeatStatusIxArgs,
) -> ProgramResult {
    change_seat_status_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn change_seat_status_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeSeatStatusAccounts<'_, '_>,
    args: ChangeSeatStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeSeatStatusKeys = accounts.into();
    let ix = change_seat_status_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_seat_status_invoke_signed(
    accounts: ChangeSeatStatusAccounts<'_, '_>,
    args: ChangeSeatStatusIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_seat_status_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn change_seat_status_verify_account_keys(
    accounts: ChangeSeatStatusAccounts<'_, '_>,
    keys: ChangeSeatStatusKeys,
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
pub fn change_seat_status_verify_writable_privileges<'me, 'info>(
    accounts: ChangeSeatStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.seat] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_seat_status_verify_signer_privileges<'me, 'info>(
    accounts: ChangeSeatStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_seat_status_verify_account_privileges<'me, 'info>(
    accounts: ChangeSeatStatusAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_seat_status_verify_writable_privileges(accounts)?;
    change_seat_status_verify_signer_privileges(accounts)?;
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
impl From<RequestSeatAuthorizedAccounts<'_, '_>> for RequestSeatAuthorizedKeys {
    fn from(accounts: RequestSeatAuthorizedAccounts) -> Self {
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
impl From<RequestSeatAuthorizedKeys> for [AccountMeta; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestSeatAuthorizedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<RequestSeatAuthorizedAccounts<'_, 'info>>
    for [AccountInfo<'info>; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RequestSeatAuthorizedAccounts<'_, 'info>) -> Self {
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
pub const REQUEST_SEAT_AUTHORIZED_IX_DISCM: u8 = 105u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RequestSeatAuthorizedIxData;
impl RequestSeatAuthorizedIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REQUEST_SEAT_AUTHORIZED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REQUEST_SEAT_AUTHORIZED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REQUEST_SEAT_AUTHORIZED_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_seat_authorized_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestSeatAuthorizedKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_SEAT_AUTHORIZED_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RequestSeatAuthorizedIxData.try_to_vec()?,
    })
}
pub fn request_seat_authorized_ix(keys: RequestSeatAuthorizedKeys) -> std::io::Result<Instruction> {
    request_seat_authorized_ix_with_program_id(crate::ID, keys)
}
pub fn request_seat_authorized_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestSeatAuthorizedAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RequestSeatAuthorizedKeys = accounts.into();
    let ix = request_seat_authorized_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_seat_authorized_invoke(
    accounts: RequestSeatAuthorizedAccounts<'_, '_>,
) -> ProgramResult {
    request_seat_authorized_invoke_with_program_id(crate::ID, accounts)
}
pub fn request_seat_authorized_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestSeatAuthorizedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestSeatAuthorizedKeys = accounts.into();
    let ix = request_seat_authorized_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_seat_authorized_invoke_signed(
    accounts: RequestSeatAuthorizedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_seat_authorized_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn request_seat_authorized_verify_account_keys(
    accounts: RequestSeatAuthorizedAccounts<'_, '_>,
    keys: RequestSeatAuthorizedKeys,
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
pub fn request_seat_authorized_verify_writable_privileges<'me, 'info>(
    accounts: RequestSeatAuthorizedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market, accounts.payer, accounts.seat] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_seat_authorized_verify_signer_privileges<'me, 'info>(
    accounts: RequestSeatAuthorizedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_seat_authorized_verify_account_privileges<'me, 'info>(
    accounts: RequestSeatAuthorizedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_seat_authorized_verify_writable_privileges(accounts)?;
    request_seat_authorized_verify_signer_privileges(accounts)?;
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
impl From<EvictSeatAccounts<'_, '_>> for EvictSeatKeys {
    fn from(accounts: EvictSeatAccounts) -> Self {
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
impl From<EvictSeatKeys> for [AccountMeta; EVICT_SEAT_IX_ACCOUNTS_LEN] {
    fn from(keys: EvictSeatKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<EvictSeatAccounts<'_, 'info>>
    for [AccountInfo<'info>; EVICT_SEAT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: EvictSeatAccounts<'_, 'info>) -> Self {
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
pub const EVICT_SEAT_IX_DISCM: u8 = 106u8;
#[derive(Clone, Debug, PartialEq)]
pub struct EvictSeatIxData;
impl EvictSeatIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != EVICT_SEAT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EVICT_SEAT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[EVICT_SEAT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn evict_seat_ix_with_program_id(
    program_id: Pubkey,
    keys: EvictSeatKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EVICT_SEAT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: EvictSeatIxData.try_to_vec()?,
    })
}
pub fn evict_seat_ix(keys: EvictSeatKeys) -> std::io::Result<Instruction> {
    evict_seat_ix_with_program_id(crate::ID, keys)
}
pub fn evict_seat_invoke_with_program_id(
    program_id: Pubkey,
    accounts: EvictSeatAccounts<'_, '_>,
) -> ProgramResult {
    let keys: EvictSeatKeys = accounts.into();
    let ix = evict_seat_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn evict_seat_invoke(accounts: EvictSeatAccounts<'_, '_>) -> ProgramResult {
    evict_seat_invoke_with_program_id(crate::ID, accounts)
}
pub fn evict_seat_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: EvictSeatAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: EvictSeatKeys = accounts.into();
    let ix = evict_seat_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn evict_seat_invoke_signed(
    accounts: EvictSeatAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    evict_seat_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn evict_seat_verify_account_keys(
    accounts: EvictSeatAccounts<'_, '_>,
    keys: EvictSeatKeys,
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
pub fn evict_seat_verify_writable_privileges<'me, 'info>(
    accounts: EvictSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn evict_seat_verify_signer_privileges<'me, 'info>(
    accounts: EvictSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn evict_seat_verify_account_privileges<'me, 'info>(
    accounts: EvictSeatAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    evict_seat_verify_writable_privileges(accounts)?;
    evict_seat_verify_signer_privileges(accounts)?;
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
impl From<ForceCancelOrdersAccounts<'_, '_>> for ForceCancelOrdersKeys {
    fn from(accounts: ForceCancelOrdersAccounts) -> Self {
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
impl From<ForceCancelOrdersKeys> for [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: ForceCancelOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seat,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<ForceCancelOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ForceCancelOrdersAccounts<'_, 'info>) -> Self {
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
pub const FORCE_CANCEL_ORDERS_IX_DISCM: u8 = 107u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForceCancelOrdersIxArgs {
    pub params: CancelUpToParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ForceCancelOrdersIxData(pub ForceCancelOrdersIxArgs);
impl From<ForceCancelOrdersIxArgs> for ForceCancelOrdersIxData {
    fn from(args: ForceCancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl ForceCancelOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
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
        writer.write_all(&[FORCE_CANCEL_ORDERS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn force_cancel_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: ForceCancelOrdersKeys,
    args: ForceCancelOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: ForceCancelOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn force_cancel_orders_ix(
    keys: ForceCancelOrdersKeys,
    args: ForceCancelOrdersIxArgs,
) -> std::io::Result<Instruction> {
    force_cancel_orders_ix_with_program_id(crate::ID, keys, args)
}
pub fn force_cancel_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ForceCancelOrdersAccounts<'_, '_>,
    args: ForceCancelOrdersIxArgs,
) -> ProgramResult {
    let keys: ForceCancelOrdersKeys = accounts.into();
    let ix = force_cancel_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn force_cancel_orders_invoke(
    accounts: ForceCancelOrdersAccounts<'_, '_>,
    args: ForceCancelOrdersIxArgs,
) -> ProgramResult {
    force_cancel_orders_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn force_cancel_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ForceCancelOrdersAccounts<'_, '_>,
    args: ForceCancelOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ForceCancelOrdersKeys = accounts.into();
    let ix = force_cancel_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn force_cancel_orders_invoke_signed(
    accounts: ForceCancelOrdersAccounts<'_, '_>,
    args: ForceCancelOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    force_cancel_orders_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn force_cancel_orders_verify_account_keys(
    accounts: ForceCancelOrdersAccounts<'_, '_>,
    keys: ForceCancelOrdersKeys,
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
pub fn force_cancel_orders_verify_writable_privileges<'me, 'info>(
    accounts: ForceCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn force_cancel_orders_verify_signer_privileges<'me, 'info>(
    accounts: ForceCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn force_cancel_orders_verify_account_privileges<'me, 'info>(
    accounts: ForceCancelOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    force_cancel_orders_verify_writable_privileges(accounts)?;
    force_cancel_orders_verify_signer_privileges(accounts)?;
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
impl From<CollectFeesAccounts<'_, '_>> for CollectFeesKeys {
    fn from(accounts: CollectFeesAccounts) -> Self {
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
impl From<CollectFeesKeys> for [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: CollectFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sweeper,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<CollectFeesAccounts<'_, 'info>>
    for [AccountInfo<'info>; COLLECT_FEES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CollectFeesAccounts<'_, 'info>) -> Self {
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
pub const COLLECT_FEES_IX_DISCM: u8 = 108u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CollectFeesIxData;
impl CollectFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != COLLECT_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    COLLECT_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[COLLECT_FEES_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn collect_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: CollectFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COLLECT_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CollectFeesIxData.try_to_vec()?,
    })
}
pub fn collect_fees_ix(keys: CollectFeesKeys) -> std::io::Result<Instruction> {
    collect_fees_ix_with_program_id(crate::ID, keys)
}
pub fn collect_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn collect_fees_invoke(accounts: CollectFeesAccounts<'_, '_>) -> ProgramResult {
    collect_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn collect_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CollectFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CollectFeesKeys = accounts.into();
    let ix = collect_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn collect_fees_invoke_signed(
    accounts: CollectFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    collect_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn collect_fees_verify_account_keys(
    accounts: CollectFeesAccounts<'_, '_>,
    keys: CollectFeesKeys,
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
pub fn collect_fees_verify_writable_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.fee_recipient,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_signer_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sweeper] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn collect_fees_verify_account_privileges<'me, 'info>(
    accounts: CollectFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    collect_fees_verify_writable_privileges(accounts)?;
    collect_fees_verify_signer_privileges(accounts)?;
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
impl From<ChangeFeeRecipientAccounts<'_, '_>> for ChangeFeeRecipientKeys {
    fn from(accounts: ChangeFeeRecipientAccounts) -> Self {
        Self {
            phoenix_program: *accounts.phoenix_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            new_fee_recipient: *accounts.new_fee_recipient.key,
        }
    }
}
impl From<ChangeFeeRecipientKeys> for [AccountMeta; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN] {
    fn from(keys: ChangeFeeRecipientKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.phoenix_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_fee_recipient,
                is_signer: false,
                is_writable: false,
            },
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
impl<'info> From<ChangeFeeRecipientAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ChangeFeeRecipientAccounts<'_, 'info>) -> Self {
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
pub const CHANGE_FEE_RECIPIENT_IX_DISCM: u8 = 109u8;
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeFeeRecipientIxData;
impl ChangeFeeRecipientIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CHANGE_FEE_RECIPIENT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_FEE_RECIPIENT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CHANGE_FEE_RECIPIENT_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn change_fee_recipient_ix_with_program_id(
    program_id: Pubkey,
    keys: ChangeFeeRecipientKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CHANGE_FEE_RECIPIENT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ChangeFeeRecipientIxData.try_to_vec()?,
    })
}
pub fn change_fee_recipient_ix(keys: ChangeFeeRecipientKeys) -> std::io::Result<Instruction> {
    change_fee_recipient_ix_with_program_id(crate::ID, keys)
}
pub fn change_fee_recipient_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ChangeFeeRecipientAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ChangeFeeRecipientKeys = accounts.into();
    let ix = change_fee_recipient_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn change_fee_recipient_invoke(accounts: ChangeFeeRecipientAccounts<'_, '_>) -> ProgramResult {
    change_fee_recipient_invoke_with_program_id(crate::ID, accounts)
}
pub fn change_fee_recipient_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ChangeFeeRecipientAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ChangeFeeRecipientKeys = accounts.into();
    let ix = change_fee_recipient_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn change_fee_recipient_invoke_signed(
    accounts: ChangeFeeRecipientAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    change_fee_recipient_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn change_fee_recipient_verify_account_keys(
    accounts: ChangeFeeRecipientAccounts<'_, '_>,
    keys: ChangeFeeRecipientKeys,
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
pub fn change_fee_recipient_verify_writable_privileges<'me, 'info>(
    accounts: ChangeFeeRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn change_fee_recipient_verify_signer_privileges<'me, 'info>(
    accounts: ChangeFeeRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.market_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn change_fee_recipient_verify_account_privileges<'me, 'info>(
    accounts: ChangeFeeRecipientAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    change_fee_recipient_verify_writable_privileges(accounts)?;
    change_fee_recipient_verify_signer_privileges(accounts)?;
    Ok(())
}
