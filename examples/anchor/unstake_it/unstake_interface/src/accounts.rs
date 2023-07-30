use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const FEE_ACCOUNT_DISCM: [u8; 8] = [24, 55, 150, 250, 168, 27, 101, 178];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fee {
    pub fee: FeeEnum,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeAccount(pub Fee);
impl BorshSerialize for FeeAccount {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FEE_ACCOUNT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FeeAccount {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FEE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FEE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Fee::deserialize(buf)?))
    }
}
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool {
    pub fee_authority: Pubkey,
    pub lp_mint: Pubkey,
    pub incoming_stake: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);
impl BorshSerialize for PoolAccount {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        POOL_ACCOUNT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl PoolAccount {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    POOL_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Pool::deserialize(buf)?))
    }
}
pub const PROTOCOL_FEE_ACCOUNT_DISCM: [u8; 8] = [121, 127, 98, 139, 72, 110, 44, 118];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtocolFee {
    pub destination: Pubkey,
    pub authority: Pubkey,
    pub fee_ratio: Rational,
    pub referrer_fee_ratio: Rational,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolFeeAccount(pub ProtocolFee);
impl BorshSerialize for ProtocolFeeAccount {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        PROTOCOL_FEE_ACCOUNT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ProtocolFeeAccount {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != PROTOCOL_FEE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PROTOCOL_FEE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ProtocolFee::deserialize(buf)?))
    }
}
pub const STAKE_ACCOUNT_RECORD_ACCOUNT_DISCM: [u8; 8] = [144, 205, 183, 241, 3, 250, 208, 215];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeAccountRecord {
    pub lamports_at_creation: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeAccountRecordAccount(pub StakeAccountRecord);
impl BorshSerialize for StakeAccountRecordAccount {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        STAKE_ACCOUNT_RECORD_ACCOUNT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl StakeAccountRecordAccount {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != STAKE_ACCOUNT_RECORD_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STAKE_ACCOUNT_RECORD_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(StakeAccountRecord::deserialize(buf)?))
    }
}
