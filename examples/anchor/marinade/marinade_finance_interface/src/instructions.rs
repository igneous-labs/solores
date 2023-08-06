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
pub enum MarinadeFinanceProgramIx {
    Initialize(InitializeIxArgs),
    ChangeAuthority(ChangeAuthorityIxArgs),
    AddValidator(AddValidatorIxArgs),
    RemoveValidator(RemoveValidatorIxArgs),
    SetValidatorScore(SetValidatorScoreIxArgs),
    ConfigValidatorSystem(ConfigValidatorSystemIxArgs),
    Deposit(DepositIxArgs),
    DepositStakeAccount(DepositStakeAccountIxArgs),
    LiquidUnstake(LiquidUnstakeIxArgs),
    AddLiquidity(AddLiquidityIxArgs),
    RemoveLiquidity(RemoveLiquidityIxArgs),
    SetLpParams(SetLpParamsIxArgs),
    ConfigMarinade(ConfigMarinadeIxArgs),
    OrderUnstake(OrderUnstakeIxArgs),
    Claim(ClaimIxArgs),
    StakeReserve(StakeReserveIxArgs),
    UpdateActive(UpdateActiveIxArgs),
    UpdateDeactivated(UpdateDeactivatedIxArgs),
    DeactivateStake(DeactivateStakeIxArgs),
    EmergencyUnstake(EmergencyUnstakeIxArgs),
    MergeStakes(MergeStakesIxArgs),
}
impl BorshSerialize for MarinadeFinanceProgramIx {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Self::Initialize(args) => {
                INITIALIZE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ChangeAuthority(args) => {
                CHANGE_AUTHORITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::AddValidator(args) => {
                ADD_VALIDATOR_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RemoveValidator(args) => {
                REMOVE_VALIDATOR_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetValidatorScore(args) => {
                SET_VALIDATOR_SCORE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ConfigValidatorSystem(args) => {
                CONFIG_VALIDATOR_SYSTEM_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::Deposit(args) => {
                DEPOSIT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::DepositStakeAccount(args) => {
                DEPOSIT_STAKE_ACCOUNT_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::LiquidUnstake(args) => {
                LIQUID_UNSTAKE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::AddLiquidity(args) => {
                ADD_LIQUIDITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::RemoveLiquidity(args) => {
                REMOVE_LIQUIDITY_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::SetLpParams(args) => {
                SET_LP_PARAMS_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::ConfigMarinade(args) => {
                CONFIG_MARINADE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::OrderUnstake(args) => {
                ORDER_UNSTAKE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::Claim(args) => {
                CLAIM_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::StakeReserve(args) => {
                STAKE_RESERVE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::UpdateActive(args) => {
                UPDATE_ACTIVE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::UpdateDeactivated(args) => {
                UPDATE_DEACTIVATED_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::DeactivateStake(args) => {
                DEACTIVATE_STAKE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::EmergencyUnstake(args) => {
                EMERGENCY_UNSTAKE_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
            Self::MergeStakes(args) => {
                MERGE_STAKES_IX_DISCM.serialize(writer)?;
                args.serialize(writer)
            }
        }
    }
}
impl MarinadeFinanceProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        match maybe_discm {
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(buf)?)),
            CHANGE_AUTHORITY_IX_DISCM => Ok(Self::ChangeAuthority(
                ChangeAuthorityIxArgs::deserialize(buf)?,
            )),
            ADD_VALIDATOR_IX_DISCM => Ok(Self::AddValidator(AddValidatorIxArgs::deserialize(buf)?)),
            REMOVE_VALIDATOR_IX_DISCM => Ok(Self::RemoveValidator(
                RemoveValidatorIxArgs::deserialize(buf)?,
            )),
            SET_VALIDATOR_SCORE_IX_DISCM => Ok(Self::SetValidatorScore(
                SetValidatorScoreIxArgs::deserialize(buf)?,
            )),
            CONFIG_VALIDATOR_SYSTEM_IX_DISCM => Ok(Self::ConfigValidatorSystem(
                ConfigValidatorSystemIxArgs::deserialize(buf)?,
            )),
            DEPOSIT_IX_DISCM => Ok(Self::Deposit(DepositIxArgs::deserialize(buf)?)),
            DEPOSIT_STAKE_ACCOUNT_IX_DISCM => Ok(Self::DepositStakeAccount(
                DepositStakeAccountIxArgs::deserialize(buf)?,
            )),
            LIQUID_UNSTAKE_IX_DISCM => {
                Ok(Self::LiquidUnstake(LiquidUnstakeIxArgs::deserialize(buf)?))
            }
            ADD_LIQUIDITY_IX_DISCM => Ok(Self::AddLiquidity(AddLiquidityIxArgs::deserialize(buf)?)),
            REMOVE_LIQUIDITY_IX_DISCM => Ok(Self::RemoveLiquidity(
                RemoveLiquidityIxArgs::deserialize(buf)?,
            )),
            SET_LP_PARAMS_IX_DISCM => Ok(Self::SetLpParams(SetLpParamsIxArgs::deserialize(buf)?)),
            CONFIG_MARINADE_IX_DISCM => Ok(Self::ConfigMarinade(
                ConfigMarinadeIxArgs::deserialize(buf)?,
            )),
            ORDER_UNSTAKE_IX_DISCM => Ok(Self::OrderUnstake(OrderUnstakeIxArgs::deserialize(buf)?)),
            CLAIM_IX_DISCM => Ok(Self::Claim(ClaimIxArgs::deserialize(buf)?)),
            STAKE_RESERVE_IX_DISCM => Ok(Self::StakeReserve(StakeReserveIxArgs::deserialize(buf)?)),
            UPDATE_ACTIVE_IX_DISCM => Ok(Self::UpdateActive(UpdateActiveIxArgs::deserialize(buf)?)),
            UPDATE_DEACTIVATED_IX_DISCM => Ok(Self::UpdateDeactivated(
                UpdateDeactivatedIxArgs::deserialize(buf)?,
            )),
            DEACTIVATE_STAKE_IX_DISCM => Ok(Self::DeactivateStake(
                DeactivateStakeIxArgs::deserialize(buf)?,
            )),
            EMERGENCY_UNSTAKE_IX_DISCM => Ok(Self::EmergencyUnstake(
                EmergencyUnstakeIxArgs::deserialize(buf)?,
            )),
            MERGE_STAKES_IX_DISCM => Ok(Self::MergeStakes(MergeStakesIxArgs::deserialize(buf)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub creator_authority: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub reserve_pda: &'me AccountInfo<'info>,
    pub stake_list: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub operational_sol_account: &'me AccountInfo<'info>,
    pub liq_pool_lp_mint: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub treasury_msol_account: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub creator_authority: Pubkey,
    pub state: Pubkey,
    pub reserve_pda: Pubkey,
    pub stake_list: Pubkey,
    pub validator_list: Pubkey,
    pub msol_mint: Pubkey,
    pub operational_sol_account: Pubkey,
    pub liq_pool_lp_mint: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub treasury_msol_account: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
}
impl From<&InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: &InitializeAccounts) -> Self {
        Self {
            creator_authority: *accounts.creator_authority.key,
            state: *accounts.state.key,
            reserve_pda: *accounts.reserve_pda.key,
            stake_list: *accounts.stake_list.key,
            validator_list: *accounts.validator_list.key,
            msol_mint: *accounts.msol_mint.key,
            operational_sol_account: *accounts.operational_sol_account.key,
            liq_pool_lp_mint: *accounts.liq_pool_lp_mint.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            treasury_msol_account: *accounts.treasury_msol_account.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.creator_authority, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.reserve_pda, false),
            AccountMeta::new(keys.stake_list, false),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new_readonly(keys.msol_mint, false),
            AccountMeta::new_readonly(keys.operational_sol_account, false),
            AccountMeta::new_readonly(keys.liq_pool_lp_mint, false),
            AccountMeta::new_readonly(keys.liq_pool_sol_leg_pda, false),
            AccountMeta::new_readonly(keys.liq_pool_msol_leg, false),
            AccountMeta::new_readonly(keys.treasury_msol_account, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator_authority: pubkeys[0],
            state: pubkeys[1],
            reserve_pda: pubkeys[2],
            stake_list: pubkeys[3],
            validator_list: pubkeys[4],
            msol_mint: pubkeys[5],
            operational_sol_account: pubkeys[6],
            liq_pool_lp_mint: pubkeys[7],
            liq_pool_sol_leg_pda: pubkeys[8],
            liq_pool_msol_leg: pubkeys[9],
            treasury_msol_account: pubkeys[10],
            clock: pubkeys[11],
            rent: pubkeys[12],
        }
    }
}
impl<'info> From<&InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.creator_authority.clone(),
            accounts.state.clone(),
            accounts.reserve_pda.clone(),
            accounts.stake_list.clone(),
            accounts.validator_list.clone(),
            accounts.msol_mint.clone(),
            accounts.operational_sol_account.clone(),
            accounts.liq_pool_lp_mint.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.treasury_msol_account.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            creator_authority: &arr[0],
            state: &arr[1],
            reserve_pda: &arr[2],
            stake_list: &arr[3],
            validator_list: &arr[4],
            msol_mint: &arr[5],
            operational_sol_account: &arr[6],
            liq_pool_lp_mint: &arr[7],
            liq_pool_sol_leg_pda: &arr[8],
            liq_pool_msol_leg: &arr[9],
            treasury_msol_account: &arr[10],
            clock: &arr[11],
            rent: &arr[12],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub data: InitializeData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(buf)?))
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
        (accounts.creator_authority.key, &keys.creator_authority),
        (accounts.state.key, &keys.state),
        (accounts.reserve_pda.key, &keys.reserve_pda),
        (accounts.stake_list.key, &keys.stake_list),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.msol_mint.key, &keys.msol_mint),
        (
            accounts.operational_sol_account.key,
            &keys.operational_sol_account,
        ),
        (accounts.liq_pool_lp_mint.key, &keys.liq_pool_lp_mint),
        (
            accounts.liq_pool_sol_leg_pda.key,
            &keys.liq_pool_sol_leg_pda,
        ),
        (accounts.liq_pool_msol_leg.key, &keys.liq_pool_msol_leg),
        (
            accounts.treasury_msol_account.key,
            &keys.treasury_msol_account,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.rent.key, &keys.rent),
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
    for should_be_writable in [accounts.state, accounts.stake_list, accounts.validator_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.creator_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CHANGE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ChangeAuthorityAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChangeAuthorityKeys {
    pub state: Pubkey,
    pub admin_authority: Pubkey,
}
impl From<&ChangeAuthorityAccounts<'_, '_>> for ChangeAuthorityKeys {
    fn from(accounts: &ChangeAuthorityAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin_authority: *accounts.admin_authority.key,
        }
    }
}
impl From<&ChangeAuthorityKeys> for [AccountMeta; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ChangeAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.admin_authority, true),
        ]
    }
}
impl From<[Pubkey; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN]> for ChangeAuthorityKeys {
    fn from(pubkeys: [Pubkey; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            admin_authority: pubkeys[1],
        }
    }
}
impl<'info> From<&ChangeAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ChangeAuthorityAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.admin_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for ChangeAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            admin_authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChangeAuthorityIxArgs {
    pub data: ChangeAuthorityData,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeAuthorityIxData(pub ChangeAuthorityIxArgs);
pub const CHANGE_AUTHORITY_IX_DISCM: [u8; 8] = [50, 106, 66, 104, 99, 118, 145, 88];
impl From<ChangeAuthorityIxArgs> for ChangeAuthorityIxData {
    fn from(args: ChangeAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ChangeAuthorityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CHANGE_AUTHORITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl ChangeAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CHANGE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeAuthorityIxArgs::deserialize(buf)?))
    }
}
pub fn change_authority_ix<K: Into<ChangeAuthorityKeys>, A: Into<ChangeAuthorityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ChangeAuthorityKeys = accounts.into();
    let metas: [AccountMeta; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ChangeAuthorityIxArgs = args.into();
    let data: ChangeAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn change_authority_invoke<'info, A: Into<ChangeAuthorityIxArgs>>(
    accounts: &ChangeAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = change_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn change_authority_invoke_signed<'info, A: Into<ChangeAuthorityIxArgs>>(
    accounts: &ChangeAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = change_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CHANGE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn change_authority_verify_account_keys(
    accounts: &ChangeAuthorityAccounts<'_, '_>,
    keys: &ChangeAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.admin_authority.key, &keys.admin_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn change_authority_verify_account_privileges<'me, 'info>(
    accounts: &ChangeAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_VALIDATOR_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct AddValidatorAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub manager_authority: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub validator_vote: &'me AccountInfo<'info>,
    pub duplication_flag: &'me AccountInfo<'info>,
    pub rent_payer: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddValidatorKeys {
    pub state: Pubkey,
    pub manager_authority: Pubkey,
    pub validator_list: Pubkey,
    pub validator_vote: Pubkey,
    pub duplication_flag: Pubkey,
    pub rent_payer: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<&AddValidatorAccounts<'_, '_>> for AddValidatorKeys {
    fn from(accounts: &AddValidatorAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            manager_authority: *accounts.manager_authority.key,
            validator_list: *accounts.validator_list.key,
            validator_vote: *accounts.validator_vote.key,
            duplication_flag: *accounts.duplication_flag.key,
            rent_payer: *accounts.rent_payer.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&AddValidatorKeys> for [AccountMeta; ADD_VALIDATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddValidatorKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.manager_authority, true),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new_readonly(keys.validator_vote, false),
            AccountMeta::new(keys.duplication_flag, false),
            AccountMeta::new(keys.rent_payer, true),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_VALIDATOR_IX_ACCOUNTS_LEN]> for AddValidatorKeys {
    fn from(pubkeys: [Pubkey; ADD_VALIDATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            manager_authority: pubkeys[1],
            validator_list: pubkeys[2],
            validator_vote: pubkeys[3],
            duplication_flag: pubkeys[4],
            rent_payer: pubkeys[5],
            clock: pubkeys[6],
            rent: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<&AddValidatorAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_VALIDATOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddValidatorAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.manager_authority.clone(),
            accounts.validator_list.clone(),
            accounts.validator_vote.clone(),
            accounts.duplication_flag.clone(),
            accounts.rent_payer.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_VALIDATOR_IX_ACCOUNTS_LEN]>
    for AddValidatorAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_VALIDATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            manager_authority: &arr[1],
            validator_list: &arr[2],
            validator_vote: &arr[3],
            duplication_flag: &arr[4],
            rent_payer: &arr[5],
            clock: &arr[6],
            rent: &arr[7],
            system_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddValidatorIxArgs {
    pub score: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddValidatorIxData(pub AddValidatorIxArgs);
pub const ADD_VALIDATOR_IX_DISCM: [u8; 8] = [250, 113, 53, 54, 141, 117, 215, 185];
impl From<AddValidatorIxArgs> for AddValidatorIxData {
    fn from(args: AddValidatorIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddValidatorIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADD_VALIDATOR_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl AddValidatorIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADD_VALIDATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_VALIDATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddValidatorIxArgs::deserialize(buf)?))
    }
}
pub fn add_validator_ix<K: Into<AddValidatorKeys>, A: Into<AddValidatorIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddValidatorKeys = accounts.into();
    let metas: [AccountMeta; ADD_VALIDATOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddValidatorIxArgs = args.into();
    let data: AddValidatorIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_validator_invoke<'info, A: Into<AddValidatorIxArgs>>(
    accounts: &AddValidatorAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_validator_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_VALIDATOR_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_validator_invoke_signed<'info, A: Into<AddValidatorIxArgs>>(
    accounts: &AddValidatorAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_validator_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_VALIDATOR_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_validator_verify_account_keys(
    accounts: &AddValidatorAccounts<'_, '_>,
    keys: &AddValidatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.manager_authority.key, &keys.manager_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.validator_vote.key, &keys.validator_vote),
        (accounts.duplication_flag.key, &keys.duplication_flag),
        (accounts.rent_payer.key, &keys.rent_payer),
        (accounts.clock.key, &keys.clock),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_validator_verify_account_privileges<'me, 'info>(
    accounts: &AddValidatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.validator_list,
        accounts.duplication_flag,
        accounts.rent_payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager_authority, accounts.rent_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_VALIDATOR_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RemoveValidatorAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub manager_authority: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub duplication_flag: &'me AccountInfo<'info>,
    pub operational_sol_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveValidatorKeys {
    pub state: Pubkey,
    pub manager_authority: Pubkey,
    pub validator_list: Pubkey,
    pub duplication_flag: Pubkey,
    pub operational_sol_account: Pubkey,
}
impl From<&RemoveValidatorAccounts<'_, '_>> for RemoveValidatorKeys {
    fn from(accounts: &RemoveValidatorAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            manager_authority: *accounts.manager_authority.key,
            validator_list: *accounts.validator_list.key,
            duplication_flag: *accounts.duplication_flag.key,
            operational_sol_account: *accounts.operational_sol_account.key,
        }
    }
}
impl From<&RemoveValidatorKeys> for [AccountMeta; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemoveValidatorKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.manager_authority, true),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new(keys.duplication_flag, false),
            AccountMeta::new(keys.operational_sol_account, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN]> for RemoveValidatorKeys {
    fn from(pubkeys: [Pubkey; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            manager_authority: pubkeys[1],
            validator_list: pubkeys[2],
            duplication_flag: pubkeys[3],
            operational_sol_account: pubkeys[4],
        }
    }
}
impl<'info> From<&RemoveValidatorAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveValidatorAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.manager_authority.clone(),
            accounts.validator_list.clone(),
            accounts.duplication_flag.clone(),
            accounts.operational_sol_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN]>
    for RemoveValidatorAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            manager_authority: &arr[1],
            validator_list: &arr[2],
            duplication_flag: &arr[3],
            operational_sol_account: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveValidatorIxArgs {
    pub index: u32,
    pub validator_vote: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveValidatorIxData(pub RemoveValidatorIxArgs);
pub const REMOVE_VALIDATOR_IX_DISCM: [u8; 8] = [25, 96, 211, 155, 161, 14, 168, 188];
impl From<RemoveValidatorIxArgs> for RemoveValidatorIxData {
    fn from(args: RemoveValidatorIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveValidatorIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_VALIDATOR_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl RemoveValidatorIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REMOVE_VALIDATOR_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_VALIDATOR_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveValidatorIxArgs::deserialize(buf)?))
    }
}
pub fn remove_validator_ix<K: Into<RemoveValidatorKeys>, A: Into<RemoveValidatorIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveValidatorKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveValidatorIxArgs = args.into();
    let data: RemoveValidatorIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_validator_invoke<'info, A: Into<RemoveValidatorIxArgs>>(
    accounts: &RemoveValidatorAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_validator_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_validator_invoke_signed<'info, A: Into<RemoveValidatorIxArgs>>(
    accounts: &RemoveValidatorAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_validator_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_VALIDATOR_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_validator_verify_account_keys(
    accounts: &RemoveValidatorAccounts<'_, '_>,
    keys: &RemoveValidatorKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.manager_authority.key, &keys.manager_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.duplication_flag.key, &keys.duplication_flag),
        (
            accounts.operational_sol_account.key,
            &keys.operational_sol_account,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_validator_verify_account_privileges<'me, 'info>(
    accounts: &RemoveValidatorAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.validator_list,
        accounts.duplication_flag,
        accounts.operational_sol_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetValidatorScoreAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub manager_authority: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetValidatorScoreKeys {
    pub state: Pubkey,
    pub manager_authority: Pubkey,
    pub validator_list: Pubkey,
}
impl From<&SetValidatorScoreAccounts<'_, '_>> for SetValidatorScoreKeys {
    fn from(accounts: &SetValidatorScoreAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            manager_authority: *accounts.manager_authority.key,
            validator_list: *accounts.validator_list.key,
        }
    }
}
impl From<&SetValidatorScoreKeys> for [AccountMeta; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetValidatorScoreKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.manager_authority, true),
            AccountMeta::new(keys.validator_list, false),
        ]
    }
}
impl From<[Pubkey; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN]> for SetValidatorScoreKeys {
    fn from(pubkeys: [Pubkey; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            manager_authority: pubkeys[1],
            validator_list: pubkeys[2],
        }
    }
}
impl<'info> From<&SetValidatorScoreAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetValidatorScoreAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.manager_authority.clone(),
            accounts.validator_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN]>
    for SetValidatorScoreAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            manager_authority: &arr[1],
            validator_list: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetValidatorScoreIxArgs {
    pub index: u32,
    pub validator_vote: Pubkey,
    pub score: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetValidatorScoreIxData(pub SetValidatorScoreIxArgs);
pub const SET_VALIDATOR_SCORE_IX_DISCM: [u8; 8] = [101, 41, 206, 33, 216, 111, 25, 78];
impl From<SetValidatorScoreIxArgs> for SetValidatorScoreIxData {
    fn from(args: SetValidatorScoreIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetValidatorScoreIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SET_VALIDATOR_SCORE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl SetValidatorScoreIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_VALIDATOR_SCORE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_VALIDATOR_SCORE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetValidatorScoreIxArgs::deserialize(buf)?))
    }
}
pub fn set_validator_score_ix<K: Into<SetValidatorScoreKeys>, A: Into<SetValidatorScoreIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetValidatorScoreKeys = accounts.into();
    let metas: [AccountMeta; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetValidatorScoreIxArgs = args.into();
    let data: SetValidatorScoreIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_validator_score_invoke<'info, A: Into<SetValidatorScoreIxArgs>>(
    accounts: &SetValidatorScoreAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_validator_score_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_validator_score_invoke_signed<'info, A: Into<SetValidatorScoreIxArgs>>(
    accounts: &SetValidatorScoreAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_validator_score_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_VALIDATOR_SCORE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_validator_score_verify_account_keys(
    accounts: &SetValidatorScoreAccounts<'_, '_>,
    keys: &SetValidatorScoreKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.manager_authority.key, &keys.manager_authority),
        (accounts.validator_list.key, &keys.validator_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_validator_score_verify_account_privileges<'me, 'info>(
    accounts: &SetValidatorScoreAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state, accounts.validator_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigValidatorSystemAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub manager_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigValidatorSystemKeys {
    pub state: Pubkey,
    pub manager_authority: Pubkey,
}
impl From<&ConfigValidatorSystemAccounts<'_, '_>> for ConfigValidatorSystemKeys {
    fn from(accounts: &ConfigValidatorSystemAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            manager_authority: *accounts.manager_authority.key,
        }
    }
}
impl From<&ConfigValidatorSystemKeys> for [AccountMeta; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN] {
    fn from(keys: &ConfigValidatorSystemKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.manager_authority, true),
        ]
    }
}
impl From<[Pubkey; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN]> for ConfigValidatorSystemKeys {
    fn from(pubkeys: [Pubkey; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            manager_authority: pubkeys[1],
        }
    }
}
impl<'info> From<&ConfigValidatorSystemAccounts<'_, 'info>>
    for [AccountInfo<'info>; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ConfigValidatorSystemAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.manager_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN]>
    for ConfigValidatorSystemAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            manager_authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigValidatorSystemIxArgs {
    pub extra_runs: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigValidatorSystemIxData(pub ConfigValidatorSystemIxArgs);
pub const CONFIG_VALIDATOR_SYSTEM_IX_DISCM: [u8; 8] = [27, 90, 97, 209, 17, 115, 7, 40];
impl From<ConfigValidatorSystemIxArgs> for ConfigValidatorSystemIxData {
    fn from(args: ConfigValidatorSystemIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ConfigValidatorSystemIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_VALIDATOR_SYSTEM_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl ConfigValidatorSystemIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_VALIDATOR_SYSTEM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_VALIDATOR_SYSTEM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigValidatorSystemIxArgs::deserialize(buf)?))
    }
}
pub fn config_validator_system_ix<
    K: Into<ConfigValidatorSystemKeys>,
    A: Into<ConfigValidatorSystemIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ConfigValidatorSystemKeys = accounts.into();
    let metas: [AccountMeta; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ConfigValidatorSystemIxArgs = args.into();
    let data: ConfigValidatorSystemIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn config_validator_system_invoke<'info, A: Into<ConfigValidatorSystemIxArgs>>(
    accounts: &ConfigValidatorSystemAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = config_validator_system_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn config_validator_system_invoke_signed<'info, A: Into<ConfigValidatorSystemIxArgs>>(
    accounts: &ConfigValidatorSystemAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = config_validator_system_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CONFIG_VALIDATOR_SYSTEM_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn config_validator_system_verify_account_keys(
    accounts: &ConfigValidatorSystemAccounts<'_, '_>,
    keys: &ConfigValidatorSystemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.manager_authority.key, &keys.manager_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn config_validator_system_verify_account_privileges<'me, 'info>(
    accounts: &ConfigValidatorSystemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.manager_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg_authority: &'me AccountInfo<'info>,
    pub reserve_pda: &'me AccountInfo<'info>,
    pub transfer_from: &'me AccountInfo<'info>,
    pub mint_to: &'me AccountInfo<'info>,
    pub msol_mint_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositKeys {
    pub state: Pubkey,
    pub msol_mint: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_msol_leg_authority: Pubkey,
    pub reserve_pda: Pubkey,
    pub transfer_from: Pubkey,
    pub mint_to: Pubkey,
    pub msol_mint_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&DepositAccounts<'_, '_>> for DepositKeys {
    fn from(accounts: &DepositAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            msol_mint: *accounts.msol_mint.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            liq_pool_msol_leg_authority: *accounts.liq_pool_msol_leg_authority.key,
            reserve_pda: *accounts.reserve_pda.key,
            transfer_from: *accounts.transfer_from.key,
            mint_to: *accounts.mint_to.key,
            msol_mint_authority: *accounts.msol_mint_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &DepositKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.msol_mint, false),
            AccountMeta::new(keys.liq_pool_sol_leg_pda, false),
            AccountMeta::new(keys.liq_pool_msol_leg, false),
            AccountMeta::new_readonly(keys.liq_pool_msol_leg_authority, false),
            AccountMeta::new(keys.reserve_pda, false),
            AccountMeta::new(keys.transfer_from, true),
            AccountMeta::new(keys.mint_to, false),
            AccountMeta::new_readonly(keys.msol_mint_authority, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]> for DepositKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            msol_mint: pubkeys[1],
            liq_pool_sol_leg_pda: pubkeys[2],
            liq_pool_msol_leg: pubkeys[3],
            liq_pool_msol_leg_authority: pubkeys[4],
            reserve_pda: pubkeys[5],
            transfer_from: pubkeys[6],
            mint_to: pubkeys[7],
            msol_mint_authority: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<&DepositAccounts<'_, 'info>> for [AccountInfo<'info>; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: &DepositAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.msol_mint.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.liq_pool_msol_leg_authority.clone(),
            accounts.reserve_pda.clone(),
            accounts.transfer_from.clone(),
            accounts.mint_to.clone(),
            accounts.msol_mint_authority.clone(),
            accounts.system_program.clone(),
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
            msol_mint: &arr[1],
            liq_pool_sol_leg_pda: &arr[2],
            liq_pool_msol_leg: &arr[3],
            liq_pool_msol_leg_authority: &arr[4],
            reserve_pda: &arr[5],
            transfer_from: &arr[6],
            mint_to: &arr[7],
            msol_mint_authority: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositIxArgs {
    pub lamports: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositIxData(pub DepositIxArgs);
pub const DEPOSIT_IX_DISCM: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
impl From<DepositIxArgs> for DepositIxData {
    fn from(args: DepositIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DepositIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl DepositIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEPOSIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositIxArgs::deserialize(buf)?))
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
        (accounts.msol_mint.key, &keys.msol_mint),
        (
            accounts.liq_pool_sol_leg_pda.key,
            &keys.liq_pool_sol_leg_pda,
        ),
        (accounts.liq_pool_msol_leg.key, &keys.liq_pool_msol_leg),
        (
            accounts.liq_pool_msol_leg_authority.key,
            &keys.liq_pool_msol_leg_authority,
        ),
        (accounts.reserve_pda.key, &keys.reserve_pda),
        (accounts.transfer_from.key, &keys.transfer_from),
        (accounts.mint_to.key, &keys.mint_to),
        (accounts.msol_mint_authority.key, &keys.msol_mint_authority),
        (accounts.system_program.key, &keys.system_program),
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
        accounts.state,
        accounts.msol_mint,
        accounts.liq_pool_sol_leg_pda,
        accounts.liq_pool_msol_leg,
        accounts.reserve_pda,
        accounts.transfer_from,
        accounts.mint_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.transfer_from] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct DepositStakeAccountAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub stake_list: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_authority: &'me AccountInfo<'info>,
    pub duplication_flag: &'me AccountInfo<'info>,
    pub rent_payer: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub mint_to: &'me AccountInfo<'info>,
    pub msol_mint_authority: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositStakeAccountKeys {
    pub state: Pubkey,
    pub validator_list: Pubkey,
    pub stake_list: Pubkey,
    pub stake_account: Pubkey,
    pub stake_authority: Pubkey,
    pub duplication_flag: Pubkey,
    pub rent_payer: Pubkey,
    pub msol_mint: Pubkey,
    pub mint_to: Pubkey,
    pub msol_mint_authority: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&DepositStakeAccountAccounts<'_, '_>> for DepositStakeAccountKeys {
    fn from(accounts: &DepositStakeAccountAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            validator_list: *accounts.validator_list.key,
            stake_list: *accounts.stake_list.key,
            stake_account: *accounts.stake_account.key,
            stake_authority: *accounts.stake_authority.key,
            duplication_flag: *accounts.duplication_flag.key,
            rent_payer: *accounts.rent_payer.key,
            msol_mint: *accounts.msol_mint.key,
            mint_to: *accounts.mint_to.key,
            msol_mint_authority: *accounts.msol_mint_authority.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&DepositStakeAccountKeys> for [AccountMeta; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &DepositStakeAccountKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new(keys.stake_list, false),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new_readonly(keys.stake_authority, true),
            AccountMeta::new(keys.duplication_flag, false),
            AccountMeta::new(keys.rent_payer, true),
            AccountMeta::new(keys.msol_mint, false),
            AccountMeta::new(keys.mint_to, false),
            AccountMeta::new_readonly(keys.msol_mint_authority, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]> for DepositStakeAccountKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            validator_list: pubkeys[1],
            stake_list: pubkeys[2],
            stake_account: pubkeys[3],
            stake_authority: pubkeys[4],
            duplication_flag: pubkeys[5],
            rent_payer: pubkeys[6],
            msol_mint: pubkeys[7],
            mint_to: pubkeys[8],
            msol_mint_authority: pubkeys[9],
            clock: pubkeys[10],
            rent: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
            stake_program: pubkeys[14],
        }
    }
}
impl<'info> From<&DepositStakeAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DepositStakeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.validator_list.clone(),
            accounts.stake_list.clone(),
            accounts.stake_account.clone(),
            accounts.stake_authority.clone(),
            accounts.duplication_flag.clone(),
            accounts.rent_payer.clone(),
            accounts.msol_mint.clone(),
            accounts.mint_to.clone(),
            accounts.msol_mint_authority.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for DepositStakeAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            validator_list: &arr[1],
            stake_list: &arr[2],
            stake_account: &arr[3],
            stake_authority: &arr[4],
            duplication_flag: &arr[5],
            rent_payer: &arr[6],
            msol_mint: &arr[7],
            mint_to: &arr[8],
            msol_mint_authority: &arr[9],
            clock: &arr[10],
            rent: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
            stake_program: &arr[14],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositStakeAccountIxArgs {
    pub validator_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStakeAccountIxData(pub DepositStakeAccountIxArgs);
pub const DEPOSIT_STAKE_ACCOUNT_IX_DISCM: [u8; 8] = [110, 130, 115, 41, 164, 102, 2, 59];
impl From<DepositStakeAccountIxArgs> for DepositStakeAccountIxData {
    fn from(args: DepositStakeAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DepositStakeAccountIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_STAKE_ACCOUNT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl DepositStakeAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEPOSIT_STAKE_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_STAKE_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositStakeAccountIxArgs::deserialize(buf)?))
    }
}
pub fn deposit_stake_account_ix<
    K: Into<DepositStakeAccountKeys>,
    A: Into<DepositStakeAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositStakeAccountKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositStakeAccountIxArgs = args.into();
    let data: DepositStakeAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_stake_account_invoke<'info, A: Into<DepositStakeAccountIxArgs>>(
    accounts: &DepositStakeAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deposit_stake_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_stake_account_invoke_signed<'info, A: Into<DepositStakeAccountIxArgs>>(
    accounts: &DepositStakeAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_stake_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPOSIT_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deposit_stake_account_verify_account_keys(
    accounts: &DepositStakeAccountAccounts<'_, '_>,
    keys: &DepositStakeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.stake_list.key, &keys.stake_list),
        (accounts.stake_account.key, &keys.stake_account),
        (accounts.stake_authority.key, &keys.stake_authority),
        (accounts.duplication_flag.key, &keys.duplication_flag),
        (accounts.rent_payer.key, &keys.rent_payer),
        (accounts.msol_mint.key, &keys.msol_mint),
        (accounts.mint_to.key, &keys.mint_to),
        (accounts.msol_mint_authority.key, &keys.msol_mint_authority),
        (accounts.clock.key, &keys.clock),
        (accounts.rent.key, &keys.rent),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deposit_stake_account_verify_account_privileges<'me, 'info>(
    accounts: &DepositStakeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.validator_list,
        accounts.stake_list,
        accounts.stake_account,
        accounts.duplication_flag,
        accounts.rent_payer,
        accounts.msol_mint,
        accounts.mint_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.stake_authority, accounts.rent_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const LIQUID_UNSTAKE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct LiquidUnstakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub treasury_msol_account: &'me AccountInfo<'info>,
    pub get_msol_from: &'me AccountInfo<'info>,
    pub get_msol_from_authority: &'me AccountInfo<'info>,
    pub transfer_sol_to: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidUnstakeKeys {
    pub state: Pubkey,
    pub msol_mint: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub treasury_msol_account: Pubkey,
    pub get_msol_from: Pubkey,
    pub get_msol_from_authority: Pubkey,
    pub transfer_sol_to: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&LiquidUnstakeAccounts<'_, '_>> for LiquidUnstakeKeys {
    fn from(accounts: &LiquidUnstakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            msol_mint: *accounts.msol_mint.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            treasury_msol_account: *accounts.treasury_msol_account.key,
            get_msol_from: *accounts.get_msol_from.key,
            get_msol_from_authority: *accounts.get_msol_from_authority.key,
            transfer_sol_to: *accounts.transfer_sol_to.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&LiquidUnstakeKeys> for [AccountMeta; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &LiquidUnstakeKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.msol_mint, false),
            AccountMeta::new(keys.liq_pool_sol_leg_pda, false),
            AccountMeta::new(keys.liq_pool_msol_leg, false),
            AccountMeta::new(keys.treasury_msol_account, false),
            AccountMeta::new(keys.get_msol_from, false),
            AccountMeta::new_readonly(keys.get_msol_from_authority, true),
            AccountMeta::new(keys.transfer_sol_to, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN]> for LiquidUnstakeKeys {
    fn from(pubkeys: [Pubkey; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            msol_mint: pubkeys[1],
            liq_pool_sol_leg_pda: pubkeys[2],
            liq_pool_msol_leg: pubkeys[3],
            treasury_msol_account: pubkeys[4],
            get_msol_from: pubkeys[5],
            get_msol_from_authority: pubkeys[6],
            transfer_sol_to: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
        }
    }
}
impl<'info> From<&LiquidUnstakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.msol_mint.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.treasury_msol_account.clone(),
            accounts.get_msol_from.clone(),
            accounts.get_msol_from_authority.clone(),
            accounts.transfer_sol_to.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN]>
    for LiquidUnstakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            msol_mint: &arr[1],
            liq_pool_sol_leg_pda: &arr[2],
            liq_pool_msol_leg: &arr[3],
            treasury_msol_account: &arr[4],
            get_msol_from: &arr[5],
            get_msol_from_authority: &arr[6],
            transfer_sol_to: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidUnstakeIxArgs {
    pub msol_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeIxData(pub LiquidUnstakeIxArgs);
pub const LIQUID_UNSTAKE_IX_DISCM: [u8; 8] = [30, 30, 119, 240, 191, 227, 12, 16];
impl From<LiquidUnstakeIxArgs> for LiquidUnstakeIxData {
    fn from(args: LiquidUnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LiquidUnstakeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&LIQUID_UNSTAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl LiquidUnstakeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUID_UNSTAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUID_UNSTAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidUnstakeIxArgs::deserialize(buf)?))
    }
}
pub fn liquid_unstake_ix<K: Into<LiquidUnstakeKeys>, A: Into<LiquidUnstakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidUnstakeKeys = accounts.into();
    let metas: [AccountMeta; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidUnstakeIxArgs = args.into();
    let data: LiquidUnstakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquid_unstake_invoke<'info, A: Into<LiquidUnstakeIxArgs>>(
    accounts: &LiquidUnstakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = liquid_unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquid_unstake_invoke_signed<'info, A: Into<LiquidUnstakeIxArgs>>(
    accounts: &LiquidUnstakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquid_unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; LIQUID_UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn liquid_unstake_verify_account_keys(
    accounts: &LiquidUnstakeAccounts<'_, '_>,
    keys: &LiquidUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.msol_mint.key, &keys.msol_mint),
        (
            accounts.liq_pool_sol_leg_pda.key,
            &keys.liq_pool_sol_leg_pda,
        ),
        (accounts.liq_pool_msol_leg.key, &keys.liq_pool_msol_leg),
        (
            accounts.treasury_msol_account.key,
            &keys.treasury_msol_account,
        ),
        (accounts.get_msol_from.key, &keys.get_msol_from),
        (
            accounts.get_msol_from_authority.key,
            &keys.get_msol_from_authority,
        ),
        (accounts.transfer_sol_to.key, &keys.transfer_sol_to),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn liquid_unstake_verify_account_privileges<'me, 'info>(
    accounts: &LiquidUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.msol_mint,
        accounts.liq_pool_sol_leg_pda,
        accounts.liq_pool_msol_leg,
        accounts.treasury_msol_account,
        accounts.get_msol_from,
        accounts.transfer_sol_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.get_msol_from_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lp_mint_authority: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub transfer_from: &'me AccountInfo<'info>,
    pub mint_to: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddLiquidityKeys {
    pub state: Pubkey,
    pub lp_mint: Pubkey,
    pub lp_mint_authority: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub transfer_from: Pubkey,
    pub mint_to: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: &AddLiquidityAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            lp_mint: *accounts.lp_mint.key,
            lp_mint_authority: *accounts.lp_mint_authority.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            transfer_from: *accounts.transfer_from.key,
            mint_to: *accounts.mint_to.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddLiquidityKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.lp_mint, false),
            AccountMeta::new_readonly(keys.lp_mint_authority, false),
            AccountMeta::new_readonly(keys.liq_pool_msol_leg, false),
            AccountMeta::new(keys.liq_pool_sol_leg_pda, false),
            AccountMeta::new(keys.transfer_from, true),
            AccountMeta::new(keys.mint_to, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            lp_mint: pubkeys[1],
            lp_mint_authority: pubkeys[2],
            liq_pool_msol_leg: pubkeys[3],
            liq_pool_sol_leg_pda: pubkeys[4],
            transfer_from: pubkeys[5],
            mint_to: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<&AddLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.lp_mint.clone(),
            accounts.lp_mint_authority.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.transfer_from.clone(),
            accounts.mint_to.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for AddLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            lp_mint: &arr[1],
            lp_mint_authority: &arr[2],
            liq_pool_msol_leg: &arr[3],
            liq_pool_sol_leg_pda: &arr[4],
            transfer_from: &arr[5],
            mint_to: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityIxArgs {
    pub lamports: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityIxData(pub AddLiquidityIxArgs);
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
impl From<AddLiquidityIxArgs> for AddLiquidityIxData {
    fn from(args: AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddLiquidityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl AddLiquidityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddLiquidityIxArgs::deserialize(buf)?))
    }
}
pub fn add_liquidity_ix<K: Into<AddLiquidityKeys>, A: Into<AddLiquidityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddLiquidityKeys = accounts.into();
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddLiquidityIxArgs = args.into();
    let data: AddLiquidityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_invoke<'info, A: Into<AddLiquidityIxArgs>>(
    accounts: &AddLiquidityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_liquidity_invoke_signed<'info, A: Into<AddLiquidityIxArgs>>(
    accounts: &AddLiquidityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn add_liquidity_verify_account_keys(
    accounts: &AddLiquidityAccounts<'_, '_>,
    keys: &AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.lp_mint.key, &keys.lp_mint),
        (accounts.lp_mint_authority.key, &keys.lp_mint_authority),
        (accounts.liq_pool_msol_leg.key, &keys.liq_pool_msol_leg),
        (
            accounts.liq_pool_sol_leg_pda.key,
            &keys.liq_pool_sol_leg_pda,
        ),
        (accounts.transfer_from.key, &keys.transfer_from),
        (accounts.mint_to.key, &keys.mint_to),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: &AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.lp_mint,
        accounts.liq_pool_sol_leg_pda,
        accounts.transfer_from,
        accounts.mint_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.transfer_from] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub burn_from: &'me AccountInfo<'info>,
    pub burn_from_authority: &'me AccountInfo<'info>,
    pub transfer_sol_to: &'me AccountInfo<'info>,
    pub transfer_msol_to: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RemoveLiquidityKeys {
    pub state: Pubkey,
    pub lp_mint: Pubkey,
    pub burn_from: Pubkey,
    pub burn_from_authority: Pubkey,
    pub transfer_sol_to: Pubkey,
    pub transfer_msol_to: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_msol_leg_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: &RemoveLiquidityAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            lp_mint: *accounts.lp_mint.key,
            burn_from: *accounts.burn_from.key,
            burn_from_authority: *accounts.burn_from_authority.key,
            transfer_sol_to: *accounts.transfer_sol_to.key,
            transfer_msol_to: *accounts.transfer_msol_to.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            liq_pool_msol_leg_authority: *accounts.liq_pool_msol_leg_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemoveLiquidityKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.lp_mint, false),
            AccountMeta::new(keys.burn_from, false),
            AccountMeta::new_readonly(keys.burn_from_authority, true),
            AccountMeta::new(keys.transfer_sol_to, false),
            AccountMeta::new(keys.transfer_msol_to, false),
            AccountMeta::new(keys.liq_pool_sol_leg_pda, false),
            AccountMeta::new(keys.liq_pool_msol_leg, false),
            AccountMeta::new_readonly(keys.liq_pool_msol_leg_authority, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            lp_mint: pubkeys[1],
            burn_from: pubkeys[2],
            burn_from_authority: pubkeys[3],
            transfer_sol_to: pubkeys[4],
            transfer_msol_to: pubkeys[5],
            liq_pool_sol_leg_pda: pubkeys[6],
            liq_pool_msol_leg: pubkeys[7],
            liq_pool_msol_leg_authority: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<&RemoveLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.lp_mint.clone(),
            accounts.burn_from.clone(),
            accounts.burn_from_authority.clone(),
            accounts.transfer_sol_to.clone(),
            accounts.transfer_msol_to.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.liq_pool_msol_leg_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for RemoveLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            lp_mint: &arr[1],
            burn_from: &arr[2],
            burn_from_authority: &arr[3],
            transfer_sol_to: &arr[4],
            transfer_msol_to: &arr[5],
            liq_pool_sol_leg_pda: &arr[6],
            liq_pool_msol_leg: &arr[7],
            liq_pool_msol_leg_authority: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityIxArgs {
    pub tokens: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityIxData(pub RemoveLiquidityIxArgs);
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
impl From<RemoveLiquidityIxArgs> for RemoveLiquidityIxData {
    fn from(args: RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveLiquidityIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl RemoveLiquidityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveLiquidityIxArgs::deserialize(buf)?))
    }
}
pub fn remove_liquidity_ix<K: Into<RemoveLiquidityKeys>, A: Into<RemoveLiquidityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveLiquidityKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveLiquidityIxArgs = args.into();
    let data: RemoveLiquidityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_invoke<'info, A: Into<RemoveLiquidityIxArgs>>(
    accounts: &RemoveLiquidityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_liquidity_invoke_signed<'info, A: Into<RemoveLiquidityIxArgs>>(
    accounts: &RemoveLiquidityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_liquidity_verify_account_keys(
    accounts: &RemoveLiquidityAccounts<'_, '_>,
    keys: &RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.lp_mint.key, &keys.lp_mint),
        (accounts.burn_from.key, &keys.burn_from),
        (accounts.burn_from_authority.key, &keys.burn_from_authority),
        (accounts.transfer_sol_to.key, &keys.transfer_sol_to),
        (accounts.transfer_msol_to.key, &keys.transfer_msol_to),
        (
            accounts.liq_pool_sol_leg_pda.key,
            &keys.liq_pool_sol_leg_pda,
        ),
        (accounts.liq_pool_msol_leg.key, &keys.liq_pool_msol_leg),
        (
            accounts.liq_pool_msol_leg_authority.key,
            &keys.liq_pool_msol_leg_authority,
        ),
        (accounts.system_program.key, &keys.system_program),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: &RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.lp_mint,
        accounts.burn_from,
        accounts.transfer_sol_to,
        accounts.transfer_msol_to,
        accounts.liq_pool_sol_leg_pda,
        accounts.liq_pool_msol_leg,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.burn_from_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_LP_PARAMS_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLpParamsAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetLpParamsKeys {
    pub state: Pubkey,
    pub admin_authority: Pubkey,
}
impl From<&SetLpParamsAccounts<'_, '_>> for SetLpParamsKeys {
    fn from(accounts: &SetLpParamsAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin_authority: *accounts.admin_authority.key,
        }
    }
}
impl From<&SetLpParamsKeys> for [AccountMeta; SET_LP_PARAMS_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetLpParamsKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.admin_authority, true),
        ]
    }
}
impl From<[Pubkey; SET_LP_PARAMS_IX_ACCOUNTS_LEN]> for SetLpParamsKeys {
    fn from(pubkeys: [Pubkey; SET_LP_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            admin_authority: pubkeys[1],
        }
    }
}
impl<'info> From<&SetLpParamsAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LP_PARAMS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetLpParamsAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.admin_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LP_PARAMS_IX_ACCOUNTS_LEN]>
    for SetLpParamsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LP_PARAMS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            admin_authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetLpParamsIxArgs {
    pub min_fee: Fee,
    pub max_fee: Fee,
    pub liquidity_target: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetLpParamsIxData(pub SetLpParamsIxArgs);
pub const SET_LP_PARAMS_IX_DISCM: [u8; 8] = [227, 163, 242, 45, 79, 203, 106, 44];
impl From<SetLpParamsIxArgs> for SetLpParamsIxData {
    fn from(args: SetLpParamsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetLpParamsIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SET_LP_PARAMS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl SetLpParamsIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_LP_PARAMS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_LP_PARAMS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetLpParamsIxArgs::deserialize(buf)?))
    }
}
pub fn set_lp_params_ix<K: Into<SetLpParamsKeys>, A: Into<SetLpParamsIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetLpParamsKeys = accounts.into();
    let metas: [AccountMeta; SET_LP_PARAMS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetLpParamsIxArgs = args.into();
    let data: SetLpParamsIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_lp_params_invoke<'info, A: Into<SetLpParamsIxArgs>>(
    accounts: &SetLpParamsAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_lp_params_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LP_PARAMS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_lp_params_invoke_signed<'info, A: Into<SetLpParamsIxArgs>>(
    accounts: &SetLpParamsAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_lp_params_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_LP_PARAMS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_lp_params_verify_account_keys(
    accounts: &SetLpParamsAccounts<'_, '_>,
    keys: &SetLpParamsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.admin_authority.key, &keys.admin_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lp_params_verify_account_privileges<'me, 'info>(
    accounts: &SetLpParamsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CONFIG_MARINADE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigMarinadeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub admin_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigMarinadeKeys {
    pub state: Pubkey,
    pub admin_authority: Pubkey,
}
impl From<&ConfigMarinadeAccounts<'_, '_>> for ConfigMarinadeKeys {
    fn from(accounts: &ConfigMarinadeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            admin_authority: *accounts.admin_authority.key,
        }
    }
}
impl From<&ConfigMarinadeKeys> for [AccountMeta; CONFIG_MARINADE_IX_ACCOUNTS_LEN] {
    fn from(keys: &ConfigMarinadeKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.admin_authority, true),
        ]
    }
}
impl From<[Pubkey; CONFIG_MARINADE_IX_ACCOUNTS_LEN]> for ConfigMarinadeKeys {
    fn from(pubkeys: [Pubkey; CONFIG_MARINADE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            admin_authority: pubkeys[1],
        }
    }
}
impl<'info> From<&ConfigMarinadeAccounts<'_, 'info>>
    for [AccountInfo<'info>; CONFIG_MARINADE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ConfigMarinadeAccounts<'_, 'info>) -> Self {
        [accounts.state.clone(), accounts.admin_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIG_MARINADE_IX_ACCOUNTS_LEN]>
    for ConfigMarinadeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CONFIG_MARINADE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            admin_authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigMarinadeIxArgs {
    pub params: ConfigMarinadeParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigMarinadeIxData(pub ConfigMarinadeIxArgs);
pub const CONFIG_MARINADE_IX_DISCM: [u8; 8] = [67, 3, 34, 114, 190, 185, 17, 62];
impl From<ConfigMarinadeIxArgs> for ConfigMarinadeIxData {
    fn from(args: ConfigMarinadeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ConfigMarinadeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_MARINADE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl ConfigMarinadeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_MARINADE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_MARINADE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigMarinadeIxArgs::deserialize(buf)?))
    }
}
pub fn config_marinade_ix<K: Into<ConfigMarinadeKeys>, A: Into<ConfigMarinadeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ConfigMarinadeKeys = accounts.into();
    let metas: [AccountMeta; CONFIG_MARINADE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ConfigMarinadeIxArgs = args.into();
    let data: ConfigMarinadeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn config_marinade_invoke<'info, A: Into<ConfigMarinadeIxArgs>>(
    accounts: &ConfigMarinadeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = config_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CONFIG_MARINADE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn config_marinade_invoke_signed<'info, A: Into<ConfigMarinadeIxArgs>>(
    accounts: &ConfigMarinadeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = config_marinade_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CONFIG_MARINADE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn config_marinade_verify_account_keys(
    accounts: &ConfigMarinadeAccounts<'_, '_>,
    keys: &ConfigMarinadeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.admin_authority.key, &keys.admin_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn config_marinade_verify_account_privileges<'me, 'info>(
    accounts: &ConfigMarinadeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.admin_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const ORDER_UNSTAKE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct OrderUnstakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub burn_msol_from: &'me AccountInfo<'info>,
    pub burn_msol_authority: &'me AccountInfo<'info>,
    pub new_ticket_account: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OrderUnstakeKeys {
    pub state: Pubkey,
    pub msol_mint: Pubkey,
    pub burn_msol_from: Pubkey,
    pub burn_msol_authority: Pubkey,
    pub new_ticket_account: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
impl From<&OrderUnstakeAccounts<'_, '_>> for OrderUnstakeKeys {
    fn from(accounts: &OrderUnstakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            msol_mint: *accounts.msol_mint.key,
            burn_msol_from: *accounts.burn_msol_from.key,
            burn_msol_authority: *accounts.burn_msol_authority.key,
            new_ticket_account: *accounts.new_ticket_account.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&OrderUnstakeKeys> for [AccountMeta; ORDER_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &OrderUnstakeKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.msol_mint, false),
            AccountMeta::new(keys.burn_msol_from, false),
            AccountMeta::new_readonly(keys.burn_msol_authority, true),
            AccountMeta::new(keys.new_ticket_account, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; ORDER_UNSTAKE_IX_ACCOUNTS_LEN]> for OrderUnstakeKeys {
    fn from(pubkeys: [Pubkey; ORDER_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            msol_mint: pubkeys[1],
            burn_msol_from: pubkeys[2],
            burn_msol_authority: pubkeys[3],
            new_ticket_account: pubkeys[4],
            clock: pubkeys[5],
            rent: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<&OrderUnstakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; ORDER_UNSTAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &OrderUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.msol_mint.clone(),
            accounts.burn_msol_from.clone(),
            accounts.burn_msol_authority.clone(),
            accounts.new_ticket_account.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ORDER_UNSTAKE_IX_ACCOUNTS_LEN]>
    for OrderUnstakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ORDER_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            msol_mint: &arr[1],
            burn_msol_from: &arr[2],
            burn_msol_authority: &arr[3],
            new_ticket_account: &arr[4],
            clock: &arr[5],
            rent: &arr[6],
            token_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderUnstakeIxArgs {
    pub msol_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderUnstakeIxData(pub OrderUnstakeIxArgs);
pub const ORDER_UNSTAKE_IX_DISCM: [u8; 8] = [97, 167, 144, 107, 117, 190, 128, 36];
impl From<OrderUnstakeIxArgs> for OrderUnstakeIxData {
    fn from(args: OrderUnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for OrderUnstakeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ORDER_UNSTAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl OrderUnstakeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ORDER_UNSTAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ORDER_UNSTAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OrderUnstakeIxArgs::deserialize(buf)?))
    }
}
pub fn order_unstake_ix<K: Into<OrderUnstakeKeys>, A: Into<OrderUnstakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: OrderUnstakeKeys = accounts.into();
    let metas: [AccountMeta; ORDER_UNSTAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: OrderUnstakeIxArgs = args.into();
    let data: OrderUnstakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn order_unstake_invoke<'info, A: Into<OrderUnstakeIxArgs>>(
    accounts: &OrderUnstakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = order_unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ORDER_UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn order_unstake_invoke_signed<'info, A: Into<OrderUnstakeIxArgs>>(
    accounts: &OrderUnstakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = order_unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ORDER_UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn order_unstake_verify_account_keys(
    accounts: &OrderUnstakeAccounts<'_, '_>,
    keys: &OrderUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.msol_mint.key, &keys.msol_mint),
        (accounts.burn_msol_from.key, &keys.burn_msol_from),
        (accounts.burn_msol_authority.key, &keys.burn_msol_authority),
        (accounts.new_ticket_account.key, &keys.new_ticket_account),
        (accounts.clock.key, &keys.clock),
        (accounts.rent.key, &keys.rent),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn order_unstake_verify_account_privileges<'me, 'info>(
    accounts: &OrderUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.msol_mint,
        accounts.burn_msol_from,
        accounts.new_ticket_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.burn_msol_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CLAIM_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct ClaimAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub reserve_pda: &'me AccountInfo<'info>,
    pub ticket_account: &'me AccountInfo<'info>,
    pub transfer_sol_to: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimKeys {
    pub state: Pubkey,
    pub reserve_pda: Pubkey,
    pub ticket_account: Pubkey,
    pub transfer_sol_to: Pubkey,
    pub clock: Pubkey,
    pub system_program: Pubkey,
}
impl From<&ClaimAccounts<'_, '_>> for ClaimKeys {
    fn from(accounts: &ClaimAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            reserve_pda: *accounts.reserve_pda.key,
            ticket_account: *accounts.ticket_account.key,
            transfer_sol_to: *accounts.transfer_sol_to.key,
            clock: *accounts.clock.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&ClaimKeys> for [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(keys: &ClaimKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.reserve_pda, false),
            AccountMeta::new(keys.ticket_account, false),
            AccountMeta::new(keys.transfer_sol_to, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; CLAIM_IX_ACCOUNTS_LEN]> for ClaimKeys {
    fn from(pubkeys: [Pubkey; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            reserve_pda: pubkeys[1],
            ticket_account: pubkeys[2],
            transfer_sol_to: pubkeys[3],
            clock: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<&ClaimAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(accounts: &ClaimAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.reserve_pda.clone(),
            accounts.ticket_account.clone(),
            accounts.transfer_sol_to.clone(),
            accounts.clock.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]>
    for ClaimAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            reserve_pda: &arr[1],
            ticket_account: &arr[2],
            transfer_sol_to: &arr[3],
            clock: &arr[4],
            system_program: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimIxData(pub ClaimIxArgs);
pub const CLAIM_IX_DISCM: [u8; 8] = [62, 198, 214, 193, 213, 159, 108, 210];
impl From<ClaimIxArgs> for ClaimIxData {
    fn from(args: ClaimIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ClaimIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl ClaimIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimIxArgs::deserialize(buf)?))
    }
}
pub fn claim_ix<K: Into<ClaimKeys>, A: Into<ClaimIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ClaimKeys = accounts.into();
    let metas: [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ClaimIxArgs = args.into();
    let data: ClaimIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_invoke<'info, A: Into<ClaimIxArgs>>(
    accounts: &ClaimAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = claim_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn claim_invoke_signed<'info, A: Into<ClaimIxArgs>>(
    accounts: &ClaimAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = claim_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn claim_verify_account_keys(
    accounts: &ClaimAccounts<'_, '_>,
    keys: &ClaimKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.reserve_pda.key, &keys.reserve_pda),
        (accounts.ticket_account.key, &keys.ticket_account),
        (accounts.transfer_sol_to.key, &keys.transfer_sol_to),
        (accounts.clock.key, &keys.clock),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn claim_verify_account_privileges<'me, 'info>(
    accounts: &ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.reserve_pda,
        accounts.ticket_account,
        accounts.transfer_sol_to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const STAKE_RESERVE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct StakeReserveAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub stake_list: &'me AccountInfo<'info>,
    pub validator_vote: &'me AccountInfo<'info>,
    pub reserve_pda: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_deposit_authority: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub epoch_schedule: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub stake_config: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StakeReserveKeys {
    pub state: Pubkey,
    pub validator_list: Pubkey,
    pub stake_list: Pubkey,
    pub validator_vote: Pubkey,
    pub reserve_pda: Pubkey,
    pub stake_account: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub clock: Pubkey,
    pub epoch_schedule: Pubkey,
    pub rent: Pubkey,
    pub stake_history: Pubkey,
    pub stake_config: Pubkey,
    pub system_program: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&StakeReserveAccounts<'_, '_>> for StakeReserveKeys {
    fn from(accounts: &StakeReserveAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            validator_list: *accounts.validator_list.key,
            stake_list: *accounts.stake_list.key,
            validator_vote: *accounts.validator_vote.key,
            reserve_pda: *accounts.reserve_pda.key,
            stake_account: *accounts.stake_account.key,
            stake_deposit_authority: *accounts.stake_deposit_authority.key,
            clock: *accounts.clock.key,
            epoch_schedule: *accounts.epoch_schedule.key,
            rent: *accounts.rent.key,
            stake_history: *accounts.stake_history.key,
            stake_config: *accounts.stake_config.key,
            system_program: *accounts.system_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&StakeReserveKeys> for [AccountMeta; STAKE_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: &StakeReserveKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new(keys.stake_list, false),
            AccountMeta::new(keys.validator_vote, false),
            AccountMeta::new(keys.reserve_pda, false),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new_readonly(keys.stake_deposit_authority, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.epoch_schedule, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.stake_config, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; STAKE_RESERVE_IX_ACCOUNTS_LEN]> for StakeReserveKeys {
    fn from(pubkeys: [Pubkey; STAKE_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            validator_list: pubkeys[1],
            stake_list: pubkeys[2],
            validator_vote: pubkeys[3],
            reserve_pda: pubkeys[4],
            stake_account: pubkeys[5],
            stake_deposit_authority: pubkeys[6],
            clock: pubkeys[7],
            epoch_schedule: pubkeys[8],
            rent: pubkeys[9],
            stake_history: pubkeys[10],
            stake_config: pubkeys[11],
            system_program: pubkeys[12],
            stake_program: pubkeys[13],
        }
    }
}
impl<'info> From<&StakeReserveAccounts<'_, 'info>>
    for [AccountInfo<'info>; STAKE_RESERVE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &StakeReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.validator_list.clone(),
            accounts.stake_list.clone(),
            accounts.validator_vote.clone(),
            accounts.reserve_pda.clone(),
            accounts.stake_account.clone(),
            accounts.stake_deposit_authority.clone(),
            accounts.clock.clone(),
            accounts.epoch_schedule.clone(),
            accounts.rent.clone(),
            accounts.stake_history.clone(),
            accounts.stake_config.clone(),
            accounts.system_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STAKE_RESERVE_IX_ACCOUNTS_LEN]>
    for StakeReserveAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; STAKE_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            validator_list: &arr[1],
            stake_list: &arr[2],
            validator_vote: &arr[3],
            reserve_pda: &arr[4],
            stake_account: &arr[5],
            stake_deposit_authority: &arr[6],
            clock: &arr[7],
            epoch_schedule: &arr[8],
            rent: &arr[9],
            stake_history: &arr[10],
            stake_config: &arr[11],
            system_program: &arr[12],
            stake_program: &arr[13],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeReserveIxArgs {
    pub validator_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeReserveIxData(pub StakeReserveIxArgs);
pub const STAKE_RESERVE_IX_DISCM: [u8; 8] = [87, 217, 23, 179, 205, 25, 113, 129];
impl From<StakeReserveIxArgs> for StakeReserveIxData {
    fn from(args: StakeReserveIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for StakeReserveIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&STAKE_RESERVE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl StakeReserveIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != STAKE_RESERVE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STAKE_RESERVE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(StakeReserveIxArgs::deserialize(buf)?))
    }
}
pub fn stake_reserve_ix<K: Into<StakeReserveKeys>, A: Into<StakeReserveIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: StakeReserveKeys = accounts.into();
    let metas: [AccountMeta; STAKE_RESERVE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: StakeReserveIxArgs = args.into();
    let data: StakeReserveIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_reserve_invoke<'info, A: Into<StakeReserveIxArgs>>(
    accounts: &StakeReserveAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = stake_reserve_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; STAKE_RESERVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn stake_reserve_invoke_signed<'info, A: Into<StakeReserveIxArgs>>(
    accounts: &StakeReserveAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = stake_reserve_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; STAKE_RESERVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn stake_reserve_verify_account_keys(
    accounts: &StakeReserveAccounts<'_, '_>,
    keys: &StakeReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.stake_list.key, &keys.stake_list),
        (accounts.validator_vote.key, &keys.validator_vote),
        (accounts.reserve_pda.key, &keys.reserve_pda),
        (accounts.stake_account.key, &keys.stake_account),
        (
            accounts.stake_deposit_authority.key,
            &keys.stake_deposit_authority,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.epoch_schedule.key, &keys.epoch_schedule),
        (accounts.rent.key, &keys.rent),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_config.key, &keys.stake_config),
        (accounts.system_program.key, &keys.system_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn stake_reserve_verify_account_privileges<'me, 'info>(
    accounts: &StakeReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.validator_list,
        accounts.stake_list,
        accounts.validator_vote,
        accounts.reserve_pda,
        accounts.stake_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_ACTIVE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct UpdateActiveAccounts<'me, 'info> {
    pub common_state: &'me AccountInfo<'info>,
    pub common_stake_list: &'me AccountInfo<'info>,
    pub common_stake_account: &'me AccountInfo<'info>,
    pub common_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub common_reserve_pda: &'me AccountInfo<'info>,
    pub common_msol_mint: &'me AccountInfo<'info>,
    pub common_msol_mint_authority: &'me AccountInfo<'info>,
    pub common_treasury_msol_account: &'me AccountInfo<'info>,
    pub common_clock: &'me AccountInfo<'info>,
    pub common_stake_history: &'me AccountInfo<'info>,
    pub common_stake_program: &'me AccountInfo<'info>,
    pub common_token_program: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateActiveKeys {
    pub common_state: Pubkey,
    pub common_stake_list: Pubkey,
    pub common_stake_account: Pubkey,
    pub common_stake_withdraw_authority: Pubkey,
    pub common_reserve_pda: Pubkey,
    pub common_msol_mint: Pubkey,
    pub common_msol_mint_authority: Pubkey,
    pub common_treasury_msol_account: Pubkey,
    pub common_clock: Pubkey,
    pub common_stake_history: Pubkey,
    pub common_stake_program: Pubkey,
    pub common_token_program: Pubkey,
    pub validator_list: Pubkey,
}
impl From<&UpdateActiveAccounts<'_, '_>> for UpdateActiveKeys {
    fn from(accounts: &UpdateActiveAccounts) -> Self {
        Self {
            common_state: *accounts.common_state.key,
            common_stake_list: *accounts.common_stake_list.key,
            common_stake_account: *accounts.common_stake_account.key,
            common_stake_withdraw_authority: *accounts.common_stake_withdraw_authority.key,
            common_reserve_pda: *accounts.common_reserve_pda.key,
            common_msol_mint: *accounts.common_msol_mint.key,
            common_msol_mint_authority: *accounts.common_msol_mint_authority.key,
            common_treasury_msol_account: *accounts.common_treasury_msol_account.key,
            common_clock: *accounts.common_clock.key,
            common_stake_history: *accounts.common_stake_history.key,
            common_stake_program: *accounts.common_stake_program.key,
            common_token_program: *accounts.common_token_program.key,
            validator_list: *accounts.validator_list.key,
        }
    }
}
impl From<&UpdateActiveKeys> for [AccountMeta; UPDATE_ACTIVE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateActiveKeys) -> Self {
        [
            AccountMeta::new(keys.common_state, false),
            AccountMeta::new(keys.common_stake_list, false),
            AccountMeta::new(keys.common_stake_account, false),
            AccountMeta::new_readonly(keys.common_stake_withdraw_authority, false),
            AccountMeta::new(keys.common_reserve_pda, false),
            AccountMeta::new(keys.common_msol_mint, false),
            AccountMeta::new_readonly(keys.common_msol_mint_authority, false),
            AccountMeta::new(keys.common_treasury_msol_account, false),
            AccountMeta::new_readonly(keys.common_clock, false),
            AccountMeta::new_readonly(keys.common_stake_history, false),
            AccountMeta::new_readonly(keys.common_stake_program, false),
            AccountMeta::new_readonly(keys.common_token_program, false),
            AccountMeta::new(keys.validator_list, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_ACTIVE_IX_ACCOUNTS_LEN]> for UpdateActiveKeys {
    fn from(pubkeys: [Pubkey; UPDATE_ACTIVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            common_state: pubkeys[0],
            common_stake_list: pubkeys[1],
            common_stake_account: pubkeys[2],
            common_stake_withdraw_authority: pubkeys[3],
            common_reserve_pda: pubkeys[4],
            common_msol_mint: pubkeys[5],
            common_msol_mint_authority: pubkeys[6],
            common_treasury_msol_account: pubkeys[7],
            common_clock: pubkeys[8],
            common_stake_history: pubkeys[9],
            common_stake_program: pubkeys[10],
            common_token_program: pubkeys[11],
            validator_list: pubkeys[12],
        }
    }
}
impl<'info> From<&UpdateActiveAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_ACTIVE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateActiveAccounts<'_, 'info>) -> Self {
        [
            accounts.common_state.clone(),
            accounts.common_stake_list.clone(),
            accounts.common_stake_account.clone(),
            accounts.common_stake_withdraw_authority.clone(),
            accounts.common_reserve_pda.clone(),
            accounts.common_msol_mint.clone(),
            accounts.common_msol_mint_authority.clone(),
            accounts.common_treasury_msol_account.clone(),
            accounts.common_clock.clone(),
            accounts.common_stake_history.clone(),
            accounts.common_stake_program.clone(),
            accounts.common_token_program.clone(),
            accounts.validator_list.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_ACTIVE_IX_ACCOUNTS_LEN]>
    for UpdateActiveAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_ACTIVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            common_state: &arr[0],
            common_stake_list: &arr[1],
            common_stake_account: &arr[2],
            common_stake_withdraw_authority: &arr[3],
            common_reserve_pda: &arr[4],
            common_msol_mint: &arr[5],
            common_msol_mint_authority: &arr[6],
            common_treasury_msol_account: &arr[7],
            common_clock: &arr[8],
            common_stake_history: &arr[9],
            common_stake_program: &arr[10],
            common_token_program: &arr[11],
            validator_list: &arr[12],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateActiveIxArgs {
    pub stake_index: u32,
    pub validator_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateActiveIxData(pub UpdateActiveIxArgs);
pub const UPDATE_ACTIVE_IX_DISCM: [u8; 8] = [4, 67, 81, 64, 136, 245, 93, 152];
impl From<UpdateActiveIxArgs> for UpdateActiveIxData {
    fn from(args: UpdateActiveIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateActiveIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ACTIVE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl UpdateActiveIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_ACTIVE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_ACTIVE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateActiveIxArgs::deserialize(buf)?))
    }
}
pub fn update_active_ix<K: Into<UpdateActiveKeys>, A: Into<UpdateActiveIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateActiveKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_ACTIVE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateActiveIxArgs = args.into();
    let data: UpdateActiveIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_active_invoke<'info, A: Into<UpdateActiveIxArgs>>(
    accounts: &UpdateActiveAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_active_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_ACTIVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_active_invoke_signed<'info, A: Into<UpdateActiveIxArgs>>(
    accounts: &UpdateActiveAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_active_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_ACTIVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_active_verify_account_keys(
    accounts: &UpdateActiveAccounts<'_, '_>,
    keys: &UpdateActiveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.common_state.key, &keys.common_state),
        (accounts.common_stake_list.key, &keys.common_stake_list),
        (
            accounts.common_stake_account.key,
            &keys.common_stake_account,
        ),
        (
            accounts.common_stake_withdraw_authority.key,
            &keys.common_stake_withdraw_authority,
        ),
        (accounts.common_reserve_pda.key, &keys.common_reserve_pda),
        (accounts.common_msol_mint.key, &keys.common_msol_mint),
        (
            accounts.common_msol_mint_authority.key,
            &keys.common_msol_mint_authority,
        ),
        (
            accounts.common_treasury_msol_account.key,
            &keys.common_treasury_msol_account,
        ),
        (accounts.common_clock.key, &keys.common_clock),
        (
            accounts.common_stake_history.key,
            &keys.common_stake_history,
        ),
        (
            accounts.common_stake_program.key,
            &keys.common_stake_program,
        ),
        (
            accounts.common_token_program.key,
            &keys.common_token_program,
        ),
        (accounts.validator_list.key, &keys.validator_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_active_verify_account_privileges<'me, 'info>(
    accounts: &UpdateActiveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.common_state,
        accounts.common_stake_list,
        accounts.common_stake_account,
        accounts.common_reserve_pda,
        accounts.common_msol_mint,
        accounts.common_treasury_msol_account,
        accounts.validator_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDeactivatedAccounts<'me, 'info> {
    pub common_state: &'me AccountInfo<'info>,
    pub common_stake_list: &'me AccountInfo<'info>,
    pub common_stake_account: &'me AccountInfo<'info>,
    pub common_stake_withdraw_authority: &'me AccountInfo<'info>,
    pub common_reserve_pda: &'me AccountInfo<'info>,
    pub common_msol_mint: &'me AccountInfo<'info>,
    pub common_msol_mint_authority: &'me AccountInfo<'info>,
    pub common_treasury_msol_account: &'me AccountInfo<'info>,
    pub common_clock: &'me AccountInfo<'info>,
    pub common_stake_history: &'me AccountInfo<'info>,
    pub common_stake_program: &'me AccountInfo<'info>,
    pub common_token_program: &'me AccountInfo<'info>,
    pub operational_sol_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateDeactivatedKeys {
    pub common_state: Pubkey,
    pub common_stake_list: Pubkey,
    pub common_stake_account: Pubkey,
    pub common_stake_withdraw_authority: Pubkey,
    pub common_reserve_pda: Pubkey,
    pub common_msol_mint: Pubkey,
    pub common_msol_mint_authority: Pubkey,
    pub common_treasury_msol_account: Pubkey,
    pub common_clock: Pubkey,
    pub common_stake_history: Pubkey,
    pub common_stake_program: Pubkey,
    pub common_token_program: Pubkey,
    pub operational_sol_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<&UpdateDeactivatedAccounts<'_, '_>> for UpdateDeactivatedKeys {
    fn from(accounts: &UpdateDeactivatedAccounts) -> Self {
        Self {
            common_state: *accounts.common_state.key,
            common_stake_list: *accounts.common_stake_list.key,
            common_stake_account: *accounts.common_stake_account.key,
            common_stake_withdraw_authority: *accounts.common_stake_withdraw_authority.key,
            common_reserve_pda: *accounts.common_reserve_pda.key,
            common_msol_mint: *accounts.common_msol_mint.key,
            common_msol_mint_authority: *accounts.common_msol_mint_authority.key,
            common_treasury_msol_account: *accounts.common_treasury_msol_account.key,
            common_clock: *accounts.common_clock.key,
            common_stake_history: *accounts.common_stake_history.key,
            common_stake_program: *accounts.common_stake_program.key,
            common_token_program: *accounts.common_token_program.key,
            operational_sol_account: *accounts.operational_sol_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&UpdateDeactivatedKeys> for [AccountMeta; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateDeactivatedKeys) -> Self {
        [
            AccountMeta::new(keys.common_state, false),
            AccountMeta::new(keys.common_stake_list, false),
            AccountMeta::new(keys.common_stake_account, false),
            AccountMeta::new_readonly(keys.common_stake_withdraw_authority, false),
            AccountMeta::new(keys.common_reserve_pda, false),
            AccountMeta::new(keys.common_msol_mint, false),
            AccountMeta::new_readonly(keys.common_msol_mint_authority, false),
            AccountMeta::new(keys.common_treasury_msol_account, false),
            AccountMeta::new_readonly(keys.common_clock, false),
            AccountMeta::new_readonly(keys.common_stake_history, false),
            AccountMeta::new_readonly(keys.common_stake_program, false),
            AccountMeta::new_readonly(keys.common_token_program, false),
            AccountMeta::new(keys.operational_sol_account, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN]> for UpdateDeactivatedKeys {
    fn from(pubkeys: [Pubkey; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            common_state: pubkeys[0],
            common_stake_list: pubkeys[1],
            common_stake_account: pubkeys[2],
            common_stake_withdraw_authority: pubkeys[3],
            common_reserve_pda: pubkeys[4],
            common_msol_mint: pubkeys[5],
            common_msol_mint_authority: pubkeys[6],
            common_treasury_msol_account: pubkeys[7],
            common_clock: pubkeys[8],
            common_stake_history: pubkeys[9],
            common_stake_program: pubkeys[10],
            common_token_program: pubkeys[11],
            operational_sol_account: pubkeys[12],
            system_program: pubkeys[13],
        }
    }
}
impl<'info> From<&UpdateDeactivatedAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateDeactivatedAccounts<'_, 'info>) -> Self {
        [
            accounts.common_state.clone(),
            accounts.common_stake_list.clone(),
            accounts.common_stake_account.clone(),
            accounts.common_stake_withdraw_authority.clone(),
            accounts.common_reserve_pda.clone(),
            accounts.common_msol_mint.clone(),
            accounts.common_msol_mint_authority.clone(),
            accounts.common_treasury_msol_account.clone(),
            accounts.common_clock.clone(),
            accounts.common_stake_history.clone(),
            accounts.common_stake_program.clone(),
            accounts.common_token_program.clone(),
            accounts.operational_sol_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN]>
    for UpdateDeactivatedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            common_state: &arr[0],
            common_stake_list: &arr[1],
            common_stake_account: &arr[2],
            common_stake_withdraw_authority: &arr[3],
            common_reserve_pda: &arr[4],
            common_msol_mint: &arr[5],
            common_msol_mint_authority: &arr[6],
            common_treasury_msol_account: &arr[7],
            common_clock: &arr[8],
            common_stake_history: &arr[9],
            common_stake_program: &arr[10],
            common_token_program: &arr[11],
            operational_sol_account: &arr[12],
            system_program: &arr[13],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateDeactivatedIxArgs {
    pub stake_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDeactivatedIxData(pub UpdateDeactivatedIxArgs);
pub const UPDATE_DEACTIVATED_IX_DISCM: [u8; 8] = [16, 232, 131, 115, 156, 100, 239, 50];
impl From<UpdateDeactivatedIxArgs> for UpdateDeactivatedIxData {
    fn from(args: UpdateDeactivatedIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateDeactivatedIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DEACTIVATED_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl UpdateDeactivatedIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_DEACTIVATED_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_DEACTIVATED_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateDeactivatedIxArgs::deserialize(buf)?))
    }
}
pub fn update_deactivated_ix<K: Into<UpdateDeactivatedKeys>, A: Into<UpdateDeactivatedIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateDeactivatedKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateDeactivatedIxArgs = args.into();
    let data: UpdateDeactivatedIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_deactivated_invoke<'info, A: Into<UpdateDeactivatedIxArgs>>(
    accounts: &UpdateDeactivatedAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_deactivated_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_deactivated_invoke_signed<'info, A: Into<UpdateDeactivatedIxArgs>>(
    accounts: &UpdateDeactivatedAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_deactivated_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_DEACTIVATED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_deactivated_verify_account_keys(
    accounts: &UpdateDeactivatedAccounts<'_, '_>,
    keys: &UpdateDeactivatedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.common_state.key, &keys.common_state),
        (accounts.common_stake_list.key, &keys.common_stake_list),
        (
            accounts.common_stake_account.key,
            &keys.common_stake_account,
        ),
        (
            accounts.common_stake_withdraw_authority.key,
            &keys.common_stake_withdraw_authority,
        ),
        (accounts.common_reserve_pda.key, &keys.common_reserve_pda),
        (accounts.common_msol_mint.key, &keys.common_msol_mint),
        (
            accounts.common_msol_mint_authority.key,
            &keys.common_msol_mint_authority,
        ),
        (
            accounts.common_treasury_msol_account.key,
            &keys.common_treasury_msol_account,
        ),
        (accounts.common_clock.key, &keys.common_clock),
        (
            accounts.common_stake_history.key,
            &keys.common_stake_history,
        ),
        (
            accounts.common_stake_program.key,
            &keys.common_stake_program,
        ),
        (
            accounts.common_token_program.key,
            &keys.common_token_program,
        ),
        (
            accounts.operational_sol_account.key,
            &keys.operational_sol_account,
        ),
        (accounts.system_program.key, &keys.system_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_deactivated_verify_account_privileges<'me, 'info>(
    accounts: &UpdateDeactivatedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.common_state,
        accounts.common_stake_list,
        accounts.common_stake_account,
        accounts.common_reserve_pda,
        accounts.common_msol_mint,
        accounts.common_treasury_msol_account,
        accounts.operational_sol_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const DEACTIVATE_STAKE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DeactivateStakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub reserve_pda: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub stake_list: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_deposit_authority: &'me AccountInfo<'info>,
    pub split_stake_account: &'me AccountInfo<'info>,
    pub split_stake_rent_payer: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub epoch_schedule: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeactivateStakeKeys {
    pub state: Pubkey,
    pub reserve_pda: Pubkey,
    pub validator_list: Pubkey,
    pub stake_list: Pubkey,
    pub stake_account: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub split_stake_account: Pubkey,
    pub split_stake_rent_payer: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub epoch_schedule: Pubkey,
    pub stake_history: Pubkey,
    pub system_program: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&DeactivateStakeAccounts<'_, '_>> for DeactivateStakeKeys {
    fn from(accounts: &DeactivateStakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            reserve_pda: *accounts.reserve_pda.key,
            validator_list: *accounts.validator_list.key,
            stake_list: *accounts.stake_list.key,
            stake_account: *accounts.stake_account.key,
            stake_deposit_authority: *accounts.stake_deposit_authority.key,
            split_stake_account: *accounts.split_stake_account.key,
            split_stake_rent_payer: *accounts.split_stake_rent_payer.key,
            clock: *accounts.clock.key,
            rent: *accounts.rent.key,
            epoch_schedule: *accounts.epoch_schedule.key,
            stake_history: *accounts.stake_history.key,
            system_program: *accounts.system_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&DeactivateStakeKeys> for [AccountMeta; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &DeactivateStakeKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.reserve_pda, false),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new(keys.stake_list, false),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new_readonly(keys.stake_deposit_authority, false),
            AccountMeta::new(keys.split_stake_account, true),
            AccountMeta::new(keys.split_stake_rent_payer, true),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.epoch_schedule, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN]> for DeactivateStakeKeys {
    fn from(pubkeys: [Pubkey; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            reserve_pda: pubkeys[1],
            validator_list: pubkeys[2],
            stake_list: pubkeys[3],
            stake_account: pubkeys[4],
            stake_deposit_authority: pubkeys[5],
            split_stake_account: pubkeys[6],
            split_stake_rent_payer: pubkeys[7],
            clock: pubkeys[8],
            rent: pubkeys[9],
            epoch_schedule: pubkeys[10],
            stake_history: pubkeys[11],
            system_program: pubkeys[12],
            stake_program: pubkeys[13],
        }
    }
}
impl<'info> From<&DeactivateStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeactivateStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.reserve_pda.clone(),
            accounts.validator_list.clone(),
            accounts.stake_list.clone(),
            accounts.stake_account.clone(),
            accounts.stake_deposit_authority.clone(),
            accounts.split_stake_account.clone(),
            accounts.split_stake_rent_payer.clone(),
            accounts.clock.clone(),
            accounts.rent.clone(),
            accounts.epoch_schedule.clone(),
            accounts.stake_history.clone(),
            accounts.system_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN]>
    for DeactivateStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            reserve_pda: &arr[1],
            validator_list: &arr[2],
            stake_list: &arr[3],
            stake_account: &arr[4],
            stake_deposit_authority: &arr[5],
            split_stake_account: &arr[6],
            split_stake_rent_payer: &arr[7],
            clock: &arr[8],
            rent: &arr[9],
            epoch_schedule: &arr[10],
            stake_history: &arr[11],
            system_program: &arr[12],
            stake_program: &arr[13],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeactivateStakeIxArgs {
    pub stake_index: u32,
    pub validator_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeactivateStakeIxData(pub DeactivateStakeIxArgs);
pub const DEACTIVATE_STAKE_IX_DISCM: [u8; 8] = [165, 158, 229, 97, 168, 220, 187, 225];
impl From<DeactivateStakeIxArgs> for DeactivateStakeIxData {
    fn from(args: DeactivateStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeactivateStakeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEACTIVATE_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl DeactivateStakeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEACTIVATE_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEACTIVATE_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeactivateStakeIxArgs::deserialize(buf)?))
    }
}
pub fn deactivate_stake_ix<K: Into<DeactivateStakeKeys>, A: Into<DeactivateStakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeactivateStakeKeys = accounts.into();
    let metas: [AccountMeta; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeactivateStakeIxArgs = args.into();
    let data: DeactivateStakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deactivate_stake_invoke<'info, A: Into<DeactivateStakeIxArgs>>(
    accounts: &DeactivateStakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deactivate_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deactivate_stake_invoke_signed<'info, A: Into<DeactivateStakeIxArgs>>(
    accounts: &DeactivateStakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deactivate_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEACTIVATE_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deactivate_stake_verify_account_keys(
    accounts: &DeactivateStakeAccounts<'_, '_>,
    keys: &DeactivateStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.reserve_pda.key, &keys.reserve_pda),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.stake_list.key, &keys.stake_list),
        (accounts.stake_account.key, &keys.stake_account),
        (
            accounts.stake_deposit_authority.key,
            &keys.stake_deposit_authority,
        ),
        (accounts.split_stake_account.key, &keys.split_stake_account),
        (
            accounts.split_stake_rent_payer.key,
            &keys.split_stake_rent_payer,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.rent.key, &keys.rent),
        (accounts.epoch_schedule.key, &keys.epoch_schedule),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.system_program.key, &keys.system_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deactivate_stake_verify_account_privileges<'me, 'info>(
    accounts: &DeactivateStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.validator_list,
        accounts.stake_list,
        accounts.stake_account,
        accounts.split_stake_account,
        accounts.split_stake_rent_payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.split_stake_account,
        accounts.split_stake_rent_payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct EmergencyUnstakeAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub validator_manager_authority: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub stake_list: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub stake_deposit_authority: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmergencyUnstakeKeys {
    pub state: Pubkey,
    pub validator_manager_authority: Pubkey,
    pub validator_list: Pubkey,
    pub stake_list: Pubkey,
    pub stake_account: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub clock: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&EmergencyUnstakeAccounts<'_, '_>> for EmergencyUnstakeKeys {
    fn from(accounts: &EmergencyUnstakeAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            validator_manager_authority: *accounts.validator_manager_authority.key,
            validator_list: *accounts.validator_list.key,
            stake_list: *accounts.stake_list.key,
            stake_account: *accounts.stake_account.key,
            stake_deposit_authority: *accounts.stake_deposit_authority.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&EmergencyUnstakeKeys> for [AccountMeta; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &EmergencyUnstakeKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.validator_manager_authority, true),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new(keys.stake_list, false),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new_readonly(keys.stake_deposit_authority, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN]> for EmergencyUnstakeKeys {
    fn from(pubkeys: [Pubkey; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            validator_manager_authority: pubkeys[1],
            validator_list: pubkeys[2],
            stake_list: pubkeys[3],
            stake_account: pubkeys[4],
            stake_deposit_authority: pubkeys[5],
            clock: pubkeys[6],
            stake_program: pubkeys[7],
        }
    }
}
impl<'info> From<&EmergencyUnstakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EmergencyUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.validator_manager_authority.clone(),
            accounts.validator_list.clone(),
            accounts.stake_list.clone(),
            accounts.stake_account.clone(),
            accounts.stake_deposit_authority.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN]>
    for EmergencyUnstakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            validator_manager_authority: &arr[1],
            validator_list: &arr[2],
            stake_list: &arr[3],
            stake_account: &arr[4],
            stake_deposit_authority: &arr[5],
            clock: &arr[6],
            stake_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmergencyUnstakeIxArgs {
    pub stake_index: u32,
    pub validator_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EmergencyUnstakeIxData(pub EmergencyUnstakeIxArgs);
pub const EMERGENCY_UNSTAKE_IX_DISCM: [u8; 8] = [123, 69, 168, 195, 183, 213, 199, 214];
impl From<EmergencyUnstakeIxArgs> for EmergencyUnstakeIxData {
    fn from(args: EmergencyUnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EmergencyUnstakeIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&EMERGENCY_UNSTAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl EmergencyUnstakeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EMERGENCY_UNSTAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EMERGENCY_UNSTAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EmergencyUnstakeIxArgs::deserialize(buf)?))
    }
}
pub fn emergency_unstake_ix<K: Into<EmergencyUnstakeKeys>, A: Into<EmergencyUnstakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EmergencyUnstakeKeys = accounts.into();
    let metas: [AccountMeta; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EmergencyUnstakeIxArgs = args.into();
    let data: EmergencyUnstakeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn emergency_unstake_invoke<'info, A: Into<EmergencyUnstakeIxArgs>>(
    accounts: &EmergencyUnstakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = emergency_unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn emergency_unstake_invoke_signed<'info, A: Into<EmergencyUnstakeIxArgs>>(
    accounts: &EmergencyUnstakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = emergency_unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; EMERGENCY_UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn emergency_unstake_verify_account_keys(
    accounts: &EmergencyUnstakeAccounts<'_, '_>,
    keys: &EmergencyUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (
            accounts.validator_manager_authority.key,
            &keys.validator_manager_authority,
        ),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.stake_list.key, &keys.stake_list),
        (accounts.stake_account.key, &keys.stake_account),
        (
            accounts.stake_deposit_authority.key,
            &keys.stake_deposit_authority,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn emergency_unstake_verify_account_privileges<'me, 'info>(
    accounts: &EmergencyUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.validator_list,
        accounts.stake_list,
        accounts.stake_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.validator_manager_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const MERGE_STAKES_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct MergeStakesAccounts<'me, 'info> {
    pub state: &'me AccountInfo<'info>,
    pub stake_list: &'me AccountInfo<'info>,
    pub validator_list: &'me AccountInfo<'info>,
    pub destination_stake: &'me AccountInfo<'info>,
    pub source_stake: &'me AccountInfo<'info>,
    pub stake_deposit_authority: &'me AccountInfo<'info>,
    pub stake_withdraw_authority: &'me AccountInfo<'info>,
    pub operational_sol_account: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MergeStakesKeys {
    pub state: Pubkey,
    pub stake_list: Pubkey,
    pub validator_list: Pubkey,
    pub destination_stake: Pubkey,
    pub source_stake: Pubkey,
    pub stake_deposit_authority: Pubkey,
    pub stake_withdraw_authority: Pubkey,
    pub operational_sol_account: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&MergeStakesAccounts<'_, '_>> for MergeStakesKeys {
    fn from(accounts: &MergeStakesAccounts) -> Self {
        Self {
            state: *accounts.state.key,
            stake_list: *accounts.stake_list.key,
            validator_list: *accounts.validator_list.key,
            destination_stake: *accounts.destination_stake.key,
            source_stake: *accounts.source_stake.key,
            stake_deposit_authority: *accounts.stake_deposit_authority.key,
            stake_withdraw_authority: *accounts.stake_withdraw_authority.key,
            operational_sol_account: *accounts.operational_sol_account.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&MergeStakesKeys> for [AccountMeta; MERGE_STAKES_IX_ACCOUNTS_LEN] {
    fn from(keys: &MergeStakesKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.stake_list, false),
            AccountMeta::new(keys.validator_list, false),
            AccountMeta::new(keys.destination_stake, false),
            AccountMeta::new(keys.source_stake, false),
            AccountMeta::new_readonly(keys.stake_deposit_authority, false),
            AccountMeta::new_readonly(keys.stake_withdraw_authority, false),
            AccountMeta::new(keys.operational_sol_account, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; MERGE_STAKES_IX_ACCOUNTS_LEN]> for MergeStakesKeys {
    fn from(pubkeys: [Pubkey; MERGE_STAKES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: pubkeys[0],
            stake_list: pubkeys[1],
            validator_list: pubkeys[2],
            destination_stake: pubkeys[3],
            source_stake: pubkeys[4],
            stake_deposit_authority: pubkeys[5],
            stake_withdraw_authority: pubkeys[6],
            operational_sol_account: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            stake_program: pubkeys[10],
        }
    }
}
impl<'info> From<&MergeStakesAccounts<'_, 'info>>
    for [AccountInfo<'info>; MERGE_STAKES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &MergeStakesAccounts<'_, 'info>) -> Self {
        [
            accounts.state.clone(),
            accounts.stake_list.clone(),
            accounts.validator_list.clone(),
            accounts.destination_stake.clone(),
            accounts.source_stake.clone(),
            accounts.stake_deposit_authority.clone(),
            accounts.stake_withdraw_authority.clone(),
            accounts.operational_sol_account.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MERGE_STAKES_IX_ACCOUNTS_LEN]>
    for MergeStakesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MERGE_STAKES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            state: &arr[0],
            stake_list: &arr[1],
            validator_list: &arr[2],
            destination_stake: &arr[3],
            source_stake: &arr[4],
            stake_deposit_authority: &arr[5],
            stake_withdraw_authority: &arr[6],
            operational_sol_account: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            stake_program: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MergeStakesIxArgs {
    pub destination_stake_index: u32,
    pub source_stake_index: u32,
    pub validator_index: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MergeStakesIxData(pub MergeStakesIxArgs);
pub const MERGE_STAKES_IX_DISCM: [u8; 8] = [216, 36, 141, 225, 243, 78, 125, 237];
impl From<MergeStakesIxArgs> for MergeStakesIxData {
    fn from(args: MergeStakesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MergeStakesIxData {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&MERGE_STAKES_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
impl MergeStakesIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != MERGE_STAKES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MERGE_STAKES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MergeStakesIxArgs::deserialize(buf)?))
    }
}
pub fn merge_stakes_ix<K: Into<MergeStakesKeys>, A: Into<MergeStakesIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MergeStakesKeys = accounts.into();
    let metas: [AccountMeta; MERGE_STAKES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: MergeStakesIxArgs = args.into();
    let data: MergeStakesIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn merge_stakes_invoke<'info, A: Into<MergeStakesIxArgs>>(
    accounts: &MergeStakesAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = merge_stakes_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MERGE_STAKES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn merge_stakes_invoke_signed<'info, A: Into<MergeStakesIxArgs>>(
    accounts: &MergeStakesAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = merge_stakes_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; MERGE_STAKES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn merge_stakes_verify_account_keys(
    accounts: &MergeStakesAccounts<'_, '_>,
    keys: &MergeStakesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.state.key, &keys.state),
        (accounts.stake_list.key, &keys.stake_list),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.destination_stake.key, &keys.destination_stake),
        (accounts.source_stake.key, &keys.source_stake),
        (
            accounts.stake_deposit_authority.key,
            &keys.stake_deposit_authority,
        ),
        (
            accounts.stake_withdraw_authority.key,
            &keys.stake_withdraw_authority,
        ),
        (
            accounts.operational_sol_account.key,
            &keys.operational_sol_account,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn merge_stakes_verify_account_privileges<'me, 'info>(
    accounts: &MergeStakesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.stake_list,
        accounts.validator_list,
        accounts.destination_stake,
        accounts.source_stake,
        accounts.operational_sol_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
