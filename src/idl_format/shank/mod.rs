use serde::Deserialize;

use self::{instructions::NamedInstruction, typedefs::NamedType};

use super::IdlFormat;

mod instructions;
mod typedefs;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShankIdl {
    name: String,
    version: String,
    metadata: Metadata,
    accounts: Option<Vec<NamedType>>,
    types: Option<Vec<NamedType>>,
    instructions: Option<Vec<NamedInstruction>>,
}

#[derive(Deserialize)]
pub struct Metadata {
    address: String,
}

impl IdlFormat<NamedType, NamedType, NamedInstruction> for ShankIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> &str {
        &self.metadata.address
    }

    fn typedefs(&self) -> Option<&[NamedType]> {
        self.types.as_ref().map(|v| v.as_ref())
    }

    fn accounts(&self) -> Option<&[NamedType]> {
        self.accounts.as_ref().map(|v| v.as_ref())
    }

    fn instructions(&self) -> Option<&[NamedInstruction]> {
        self.instructions.as_ref().map(|v| v.as_ref())
    }
}
