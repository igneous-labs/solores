use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Fee {
    pub basis_points: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeData {
    pub admin_authority: Pubkey,
    pub validator_manager_authority: Pubkey,
    pub min_stake: u64,
    pub reward_fee: Fee,
    pub liq_pool: LiqPoolInitializeData,
    pub additional_stake_record_space: u32,
    pub additional_validator_record_space: u32,
    pub slots_for_stake_delta: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiqPoolInitializeData {
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub lp_treasury_cut: Fee,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ChangeAuthorityData {
    pub admin: Option<Pubkey>,
    pub validator_manager: Option<Pubkey>,
    pub operational_sol_account: Option<Pubkey>,
    pub treasury_msol_account: Option<Pubkey>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ConfigMarinadeParams {
    pub rewards_fee: Option<Fee>,
    pub slots_for_stake_delta: Option<u64>,
    pub min_stake: Option<u64>,
    pub min_deposit: Option<u64>,
    pub min_withdraw: Option<u64>,
    pub staking_sol_cap: Option<u64>,
    pub liquidity_sol_cap: Option<u64>,
    pub auto_add_validator_enabled: Option<bool>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct LiqPool {
    pub lp_mint: Pubkey,
    pub lp_mint_authority_bump_seed: u8,
    pub sol_leg_bump_seed: u8,
    pub msol_leg_authority_bump_seed: u8,
    pub msol_leg: Pubkey,
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,
    pub lp_supply: u64,
    pub lent_from_sol_leg: u64,
    pub liquidity_sol_cap: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct List {
    pub account: Pubkey,
    pub item_size: u32,
    pub count: u32,
    pub new_account: Pubkey,
    pub copied_count: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct StakeRecord {
    pub stake_account: Pubkey,
    pub last_update_delegated_lamports: u64,
    pub last_update_epoch: u64,
    pub is_emergency_unstaking: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct StakeSystem {
    pub stake_list: List,
    pub delayed_unstake_cooling_down: u64,
    pub stake_deposit_bump_seed: u8,
    pub stake_withdraw_bump_seed: u8,
    pub slots_for_stake_delta: u64,
    pub last_stake_delta_epoch: u64,
    pub min_stake: u64,
    pub extra_stake_delta_runs: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ValidatorRecord {
    pub validator_account: Pubkey,
    pub active_balance: u64,
    pub score: u32,
    pub last_stake_delta_epoch: u64,
    pub duplication_flag_bump_seed: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: Pubkey,
    pub total_validator_score: u32,
    pub total_active_balance: u64,
    pub auto_add_validator_enabled: u8,
}
