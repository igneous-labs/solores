use serde::Deserialize;

use self::{instructions::NamedInstruction, typedefs::NamedType};

use super::{AccountsHeaderFlags, IdlFormat, InstructionsHeaderFlags, TypedefsHeaderFlags};

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
    origin: String,
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

    fn is_correct_idl_format(&self) -> bool {
        self.metadata.origin == "shank"
    }

    fn det_typedefs_header_flags(&self) -> TypedefsHeaderFlags {
        let mut res = TypedefsHeaderFlags::default();
        let typedefs = match self.typedefs() {
            None => return res,
            Some(t) => t,
        };
        for t in typedefs {
            if t.r#type.has_pubkey_field() {
                res.has_pubkey = true;
                break;
            }
        }
        res
    }

    fn det_accounts_header_flags(&self) -> AccountsHeaderFlags {
        let mut res = AccountsHeaderFlags::default();
        let accounts = match self.accounts() {
            None => return res,
            Some(t) => t,
        };
        for a in accounts {
            if a.r#type.has_pubkey_field() {
                res.has_pubkey = true;
            }
            if a.r#type.has_defined_field() {
                res.has_defined = true;
            }
            if res.has_defined && res.has_pubkey {
                break;
            }
        }
        res
    }

    fn det_instructions_header_flags(&self) -> InstructionsHeaderFlags {
        let mut res = InstructionsHeaderFlags::default();
        let ixs = match self.instructions() {
            None => return res,
            Some(t) => t,
        };
        for ix in ixs {
            if ix
                .args
                .iter()
                .map(|a| a.r#type.is_or_has_defined())
                .any(|b| b)
            {
                res.has_defined = true;
                break;
            }
        }
        res
    }
}
