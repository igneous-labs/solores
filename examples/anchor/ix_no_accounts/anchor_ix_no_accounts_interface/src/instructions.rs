use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum AnchorIxNoAccountsProgramIx {
    NoAccountsIx(NoAccountsIxIxArgs),
}
impl AnchorIxNoAccountsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            NO_ACCOUNTS_IX_IX_DISCM => Ok(Self::NoAccountsIx(NoAccountsIxIxArgs::deserialize(
                &mut reader,
            )?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::NoAccountsIx(args) => {
                writer.write_all(&NO_ACCOUNTS_IX_IX_DISCM)?;
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
pub const NO_ACCOUNTS_IX_IX_DISCM: [u8; 8] = [195, 226, 242, 196, 225, 147, 32, 41];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoAccountsIxIxArgs {
    pub arg: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NoAccountsIxIxData(pub NoAccountsIxIxArgs);
impl From<NoAccountsIxIxArgs> for NoAccountsIxIxData {
    fn from(args: NoAccountsIxIxArgs) -> Self {
        Self(args)
    }
}
impl NoAccountsIxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != NO_ACCOUNTS_IX_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NO_ACCOUNTS_IX_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(NoAccountsIxIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&NO_ACCOUNTS_IX_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn no_accounts_ix_ix<A: Into<NoAccountsIxIxArgs>>(args: A) -> std::io::Result<Instruction> {
    let args_full: NoAccountsIxIxArgs = args.into();
    let data: NoAccountsIxIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn no_accounts_ix_invoke<A: Into<NoAccountsIxIxArgs>>(args: A) -> ProgramResult {
    let ix = no_accounts_ix_ix(args)?;
    invoke(&ix, &[])
}
pub fn no_accounts_ix_invoke_signed<A: Into<NoAccountsIxIxArgs>>(
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = no_accounts_ix_ix(args)?;
    invoke_signed(&ix, &[], seeds)
}
