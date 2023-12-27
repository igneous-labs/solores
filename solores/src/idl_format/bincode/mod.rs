use serde::Deserialize;
use toml::{map::Map, Value};

use crate::write_cargotoml::{
    DependencyValue, NUM_DERIVE_CRATE, NUM_TRAITS_CRATE, SERDE_CRATE, SOLANA_PROGRAM_CRATE,
    THISERROR_CRATE,
};

use super::{IdlCodegenModule, IdlFormat};

use self::{
    errors::{ErrorEnumVariant, ErrorsCodegenModule},
    instructions::{IxCodegenModule, NamedInstruction},
};

pub mod errors;
pub mod instructions;
pub mod typedefs;

#[derive(Deserialize)]
pub struct BincodeIdl {
    pub name: String,
    pub version: String,
    pub metadata: Metadata,
    pub instructions: Option<Vec<NamedInstruction>>,
    pub errors: Option<Vec<ErrorEnumVariant>>,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub address: String,
    pub origin: String,
}

impl IdlFormat for BincodeIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> Option<&str> {
        Some(&self.metadata.address)
    }

    fn is_correct_idl_format(&self) -> bool {
        self.metadata.origin == "bincode"
    }

    fn modules<'me>(&'me self, _args: &'me crate::Args) -> Vec<Box<dyn IdlCodegenModule + 'me>> {
        let mut res: Vec<Box<dyn IdlCodegenModule + 'me>> = Vec::new();
        if let Some(v) = &self.instructions {
            res.push(Box::new(IxCodegenModule {
                program_name: self.program_name(),
                instructions: v,
            }));
        }
        if let Some(v) = &self.errors {
            res.push(Box::new(ErrorsCodegenModule {
                program_name: self.program_name(),
                variants: v,
            }));
        }
        res
    }

    fn dependencies(&self, args: &crate::Args) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert(
            SOLANA_PROGRAM_CRATE.into(),
            DependencyValue(&args.solana_program_vers).into(),
        );
        map.insert(SERDE_CRATE.into(), DependencyValue(&args.serde_vers).into());
        if self.errors.is_some() {
            map.insert(
                THISERROR_CRATE.into(),
                DependencyValue(&args.thiserror_vers).into(),
            );
            map.insert(
                NUM_DERIVE_CRATE.into(),
                DependencyValue(&args.num_derive_vers).into(),
            );
            map.insert(
                NUM_TRAITS_CRATE.into(),
                DependencyValue(&args.num_traits_vers).into(),
            );
        }
        map
    }
}
