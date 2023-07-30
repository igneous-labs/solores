use serde::Deserialize;

use super::{IdlCodegenModule, IdlFormat};

use self::{
    accounts::AccountsCodegenModule,
    errors::{ErrorEnumVariant, ErrorsCodegenModule},
    instructions::{IxCodegenModule, NamedInstruction},
    typedefs::{NamedType, TypedefsCodegenModule},
};

pub mod accounts;
pub mod errors;
pub mod instructions;
pub mod typedefs;

#[derive(Deserialize)]
pub struct ShankIdl {
    pub name: String,
    pub version: String,
    pub metadata: Metadata,
    pub accounts: Option<Vec<NamedType>>,
    pub types: Option<Vec<NamedType>>,
    pub instructions: Option<Vec<NamedInstruction>>,
    pub errors: Option<Vec<ErrorEnumVariant>>,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub address: String,
    pub origin: String,
}

impl IdlFormat for ShankIdl {
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
        self.metadata.origin == "shank"
    }

    fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    fn modules<'me>(&'me self, args: &'me crate::Args) -> Vec<Box<dyn IdlCodegenModule + 'me>> {
        let mut res: Vec<Box<dyn IdlCodegenModule + 'me>> = Vec::new();
        if let Some(v) = &self.accounts {
            res.push(Box::new(AccountsCodegenModule {
                cli_args: args,
                named_types: v,
            }));
        }
        if let Some(v) = &self.r#types {
            res.push(Box::new(TypedefsCodegenModule {
                cli_args: args,
                named_types: v,
            }));
        }
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
}
