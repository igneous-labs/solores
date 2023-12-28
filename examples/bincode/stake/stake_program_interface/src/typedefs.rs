use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Authorized {
    pub staker: Pubkey,
    pub withdrawer: Pubkey,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Lockup {
    pub unix_timestamp: i64,
    pub epoch: u64,
    pub custodian: Pubkey,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StakeAuthorize {
    Staker,
    Withdrawer,
}
