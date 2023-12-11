use solana_program::{
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum AnchorIxBlankProgramIx {
    BlankIx,
}
impl AnchorIxBlankProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
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
            Self::BlankIx => writer.write_all(&BLANK_IX_IX_DISCM),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const BLANK_IX_IX_DISCM: [u8; 8] = [29, 47, 197, 250, 126, 165, 198, 197];
#[derive(Clone, Debug, PartialEq)]
pub struct BlankIxIxData;
impl BlankIxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
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
        writer.write_all(&BLANK_IX_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn blank_ix_ix() -> std::io::Result<Instruction> {
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::new(),
        data: BlankIxIxData.try_to_vec()?,
    })
}
pub fn blank_ix_invoke() -> ProgramResult {
    let ix = blank_ix_ix()?;
    invoke(&ix, &[])
}
pub fn blank_ix_invoke_signed(seeds: &[&[&[u8]]]) -> ProgramResult {
    let ix = blank_ix_ix()?;
    invoke_signed(&ix, &[], seeds)
}
