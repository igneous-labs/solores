use serde::Deserialize;

use self::typedefs::NamedType;

use super::IdlFormat;

mod typedefs;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShankIdl {
    name: String,
    version: String,
    metadata: Metadata,
    accounts: Vec<NamedType>,
    types: Vec<NamedType>,
}

#[derive(Deserialize)]
pub struct Metadata {
    address: String,
}

impl IdlFormat<NamedType, NamedType> for ShankIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> &str {
        &self.metadata.address
    }

    fn typedefs(&self) -> &[NamedType] {
        &self.types
    }

    fn accounts(&self) -> &[NamedType] {
        &self.accounts
    }
}
