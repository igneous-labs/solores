use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct State {
    pub msol_mint: Pubkey,
    pub admin_authority: Pubkey,
    pub operational_sol_account: Pubkey,
    pub treasury_msol_account: Pubkey,
    pub reserve_bump_seed: u8,
    pub msol_mint_authority_bump_seed: u8,
    pub rent_exempt_for_token_acc: u64,
    pub reward_fee: Fee,
    pub stake_system: StakeSystem,
    pub validator_system: ValidatorSystem,
    pub liq_pool: LiqPool,
    pub available_reserve_balance: u64,
    pub msol_supply: u64,
    pub msol_price: u64,
    pub circulating_ticket_count: u64,
    pub circulating_ticket_balance: u64,
    pub lent_from_reserve: u64,
    pub min_deposit: u64,
    pub min_withdraw: u64,
    pub staking_sol_cap: u64,
    pub emergency_cooling_down: u64,
}
pub const TICKET_ACCOUNT_DATA_ACCOUNT_DISCM: [u8; 8] = [133, 77, 18, 98, 211, 1, 231, 3];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct TicketAccountData {
    pub state_address: Pubkey,
    pub beneficiary: Pubkey,
    pub lamports_amount: u64,
    pub created_epoch: u64,
}
