use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const FEE_ACCOUNT_DISCM: [u8; 8] = [24, 55, 150, 250, 168, 27, 101, 178];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Fee {
    pub fee: FeeEnum,
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Pool {
    pub fee_authority: Pubkey,
    pub lp_mint: Pubkey,
    pub incoming_stake: u64,
}
pub const PROTOCOL_FEE_ACCOUNT_DISCM: [u8; 8] = [121, 127, 98, 139, 72, 110, 44, 118];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct ProtocolFee {
    pub destination: Pubkey,
    pub authority: Pubkey,
    pub fee_ratio: Rational,
    pub referrer_fee_ratio: Rational,
}
pub const STAKE_ACCOUNT_RECORD_ACCOUNT_DISCM: [u8; 8] = [144, 205, 183, 241, 3, 250, 208, 215];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct StakeAccountRecord {
    pub lamports_at_creation: u64,
}
