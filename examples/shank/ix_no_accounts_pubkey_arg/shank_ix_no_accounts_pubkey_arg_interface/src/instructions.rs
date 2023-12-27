use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum ShankIxNoAccountsPubkeyArgProgramIx {
    NoAccountsPubkeyArgIx(NoAccountsPubkeyArgIxIxArgs),
}
impl ShankIxNoAccountsPubkeyArgProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            NO_ACCOUNTS_PUBKEY_ARG_IX_IX_DISCM => Ok(Self::NoAccountsPubkeyArgIx(
                NoAccountsPubkeyArgIxIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::NoAccountsPubkeyArgIx(args) => {
                writer.write_all(&[NO_ACCOUNTS_PUBKEY_ARG_IX_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const NO_ACCOUNTS_PUBKEY_ARG_IX_IX_DISCM: u8 = 69u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoAccountsPubkeyArgIxIxArgs {
    pub arg: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NoAccountsPubkeyArgIxIxData(pub NoAccountsPubkeyArgIxIxArgs);
impl From<NoAccountsPubkeyArgIxIxArgs> for NoAccountsPubkeyArgIxIxData {
    fn from(args: NoAccountsPubkeyArgIxIxArgs) -> Self {
        Self(args)
    }
}
impl NoAccountsPubkeyArgIxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NO_ACCOUNTS_PUBKEY_ARG_IX_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NO_ACCOUNTS_PUBKEY_ARG_IX_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(NoAccountsPubkeyArgIxIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NO_ACCOUNTS_PUBKEY_ARG_IX_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn no_accounts_pubkey_arg_ix_ix(
    args: NoAccountsPubkeyArgIxIxArgs,
) -> std::io::Result<Instruction> {
    let data: NoAccountsPubkeyArgIxIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn no_accounts_pubkey_arg_ix_invoke(args: NoAccountsPubkeyArgIxIxArgs) -> ProgramResult {
    let ix = no_accounts_pubkey_arg_ix_ix(args)?;
    invoke(&ix, &[])
}
pub fn no_accounts_pubkey_arg_ix_invoke_signed(
    args: NoAccountsPubkeyArgIxIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = no_accounts_pubkey_arg_ix_ix(args)?;
    invoke_signed(&ix, &[], seeds)
}
