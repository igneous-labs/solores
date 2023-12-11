use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum ShankIxNoArgsProgramIx {
    NoArgsIx,
}
impl ShankIxNoArgsProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            NO_ARGS_IX_IX_DISCM => Ok(Self::NoArgsIx),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::NoArgsIx => NO_ARGS_IX_IX_DISCM.serialize(&mut writer),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const NO_ARGS_IX_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct NoArgsIxAccounts<'me, 'info> {
    pub b: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NoArgsIxKeys {
    pub b: Pubkey,
}
impl From<&NoArgsIxAccounts<'_, '_>> for NoArgsIxKeys {
    fn from(accounts: &NoArgsIxAccounts) -> Self {
        Self { b: *accounts.b.key }
    }
}
impl From<&NoArgsIxKeys> for [AccountMeta; NO_ARGS_IX_IX_ACCOUNTS_LEN] {
    fn from(keys: &NoArgsIxKeys) -> Self {
        [AccountMeta {
            pubkey: keys.b,
            is_signer: false,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; NO_ARGS_IX_IX_ACCOUNTS_LEN]> for NoArgsIxKeys {
    fn from(pubkeys: [Pubkey; NO_ARGS_IX_IX_ACCOUNTS_LEN]) -> Self {
        Self { b: pubkeys[0] }
    }
}
impl<'info> From<&NoArgsIxAccounts<'_, 'info>>
    for [AccountInfo<'info>; NO_ARGS_IX_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &NoArgsIxAccounts<'_, 'info>) -> Self {
        [accounts.b.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NO_ARGS_IX_IX_ACCOUNTS_LEN]>
    for NoArgsIxAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; NO_ARGS_IX_IX_ACCOUNTS_LEN]) -> Self {
        Self { b: &arr[0] }
    }
}
pub const NO_ARGS_IX_IX_DISCM: u8 = 69u8;
#[derive(Clone, Debug, PartialEq)]
pub struct NoArgsIxIxData;
impl NoArgsIxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NO_ARGS_IX_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NO_ARGS_IX_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NO_ARGS_IX_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn no_args_ix_ix<K: Into<NoArgsIxKeys>>(accounts: K) -> std::io::Result<Instruction> {
    let keys: NoArgsIxKeys = accounts.into();
    let metas: [AccountMeta; NO_ARGS_IX_IX_ACCOUNTS_LEN] = (&keys).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: NoArgsIxIxData.try_to_vec()?,
    })
}
pub fn no_args_ix_invoke<'info>(accounts: &NoArgsIxAccounts<'_, 'info>) -> ProgramResult {
    let ix = no_args_ix_ix(accounts)?;
    let account_info: [AccountInfo<'info>; NO_ARGS_IX_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn no_args_ix_invoke_signed<'info>(
    accounts: &NoArgsIxAccounts<'_, 'info>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = no_args_ix_ix(accounts)?;
    let account_info: [AccountInfo<'info>; NO_ARGS_IX_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn no_args_ix_verify_account_keys(
    accounts: &NoArgsIxAccounts<'_, '_>,
    keys: &NoArgsIxKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.b.key, &keys.b)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn no_args_ix_verify_account_privileges<'me, 'info>(
    accounts: &NoArgsIxAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.b] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}