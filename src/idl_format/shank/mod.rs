use serde::Deserialize;

use super::IdlFormat;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShankIdl {
    name: String,
    version: String,
}

impl IdlFormat for ShankIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }
}
