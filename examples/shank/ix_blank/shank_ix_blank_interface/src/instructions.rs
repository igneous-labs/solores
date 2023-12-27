use solana_program::{
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum ShankIxBlankProgramIx {
    BlankIx,
}
impl ShankIxBlankProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            BLANK_IX_IX_DISCM => Ok(Self::BlankIx),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BlankIx => writer.write_all(&[BLANK_IX_IX_DISCM]),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BLANK_IX_IX_DISCM: u8 = 69u8;
#[derive(Clone, Debug, PartialEq)]
pub struct BlankIxIxData;
impl BlankIxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != BLANK_IX_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BLANK_IX_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[BLANK_IX_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn blank_ix_ix_with_program_id(program_id: Pubkey) -> std::io::Result<Instruction> {
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: BlankIxIxData.try_to_vec()?,
    })
}
pub fn blank_ix_ix() -> std::io::Result<Instruction> {
    blank_ix_ix_with_program_id(crate::ID)
}
pub fn blank_ix_invoke() -> ProgramResult {
    let ix = blank_ix_ix()?;
    invoke(&ix, &[])
}
pub fn blank_ix_invoke_signed(seeds: &[&[&[u8]]]) -> ProgramResult {
    let ix = blank_ix_ix()?;
    invoke_signed(&ix, &[], seeds)
}
