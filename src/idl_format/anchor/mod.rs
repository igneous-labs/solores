use serde::Deserialize;

use self::{
    accounts::NamedAccount, errors::ErrorEnumVariant, instructions::NamedInstruction,
    typedefs::NamedType,
};

use super::{
    AccountsHeaderFlags, IdlCodegenElems, IdlFormat, InstructionsHeaderFlags, TypedefsHeaderFlags,
};

mod accounts;
mod errors;
mod instructions;
mod typedefs;

#[derive(Deserialize)]
pub struct AnchorIdl {
    name: String,
    version: String,
    metadata: Option<Metadata>,
    accounts: Option<Vec<NamedAccount>>,
    types: Option<Vec<NamedType>>,
    instructions: Option<Vec<NamedInstruction>>,
    errors: Option<Vec<ErrorEnumVariant>>,
}

#[derive(Deserialize)]
pub struct Metadata {
    address: String,
}

impl IdlCodegenElems for AnchorIdl {
    type TypedefElem = NamedType;
    type AccountElem = NamedAccount;
    type IxElem = NamedInstruction;
    type ErrorsEnumVariantElem = ErrorEnumVariant;

    fn typedefs(&self) -> Option<&[Self::TypedefElem]> {
        self.types.as_ref().map(|v| v.as_ref())
    }
    fn accounts(&self) -> Option<&[Self::AccountElem]> {
        self.accounts.as_ref().map(|v| v.as_ref())
    }

    fn instructions(&self) -> Option<&[Self::IxElem]> {
        self.instructions.as_ref().map(|v| v.as_ref())
    }

    fn errors(&self) -> Option<&[Self::ErrorsEnumVariantElem]> {
        self.errors.as_ref().map(|v| v.as_ref())
    }
}

impl IdlFormat for AnchorIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.address.as_ref())
    }

    /// Anchor IDLs dont seem to have an identifier,
    /// assume unindentified IDLs are anchor by default.
    /// -> Make sure to try deserializing Anchor last
    fn is_correct_idl_format(&self) -> bool {
        true
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
            if a.0.r#type.has_pubkey_field() {
                res.has_pubkey = true;
            }
            if a.0.r#type.has_defined_field() {
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
