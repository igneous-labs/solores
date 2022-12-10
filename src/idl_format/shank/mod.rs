use serde::Deserialize;

use super::IdlFormat;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShankIdl {
    name: String,
    version: String,
    metadata: Metadata,
}

#[derive(Deserialize)]
pub struct Metadata {
    address: String,
}

impl IdlFormat for ShankIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> &str {
        &self.metadata.address
    }
}
