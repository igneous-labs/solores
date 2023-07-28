use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;

use crate::gen_body_newtype_slice;

use super::{IdlCodegenModule, IdlFormat};

use self::{
    accounts::NamedAccount, errors::ErrorEnumVariant, instructions::NamedInstruction,
    typedefs::NamedType,
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

    fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    fn modules<'me>(&'me self) -> Vec<Box<dyn IdlCodegenModule + 'me>> {
        let mut res: Vec<Box<dyn IdlCodegenModule + 'me>> = Vec::new();
        if let Some(v) = &self.accounts {
            res.push(Box::new(AccountsCodegenModule(v)));
        }
        if let Some(v) = &self.r#types {
            res.push(Box::new(TypedefsCodegenModule(v)));
        }
        if let Some(v) = &self.instructions {
            res.push(Box::new(IxCodegenModule(v)));
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

struct AccountsCodegenModule<'a>(pub &'a [NamedAccount]);

impl IdlCodegenModule for AccountsCodegenModule<'_> {
    fn name(&self) -> &str {
        "accounts"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        let mut has_pubkey = false;
        let mut has_defined = false;
        for a in self.0 {
            if a.0.r#type.has_pubkey_field() && !has_pubkey {
                has_pubkey = true;
                res.extend(quote! {
                    use solana_program::pubkey::Pubkey;
                });
            }
            if a.0.r#type.has_defined_field() && !has_defined {
                has_defined = true;
                res.extend(quote! {
                    use crate::*;
                })
            }
            if has_defined && has_pubkey {
                break;
            }
        }
        res
    }

    gen_body_newtype_slice!();
}

struct TypedefsCodegenModule<'a>(pub &'a [NamedType]);

impl IdlCodegenModule for TypedefsCodegenModule<'_> {
    fn name(&self) -> &str {
        "typedefs"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        for t in self.0 {
            if t.r#type.has_pubkey_field() {
                res.extend(quote! {
                    use solana_program::pubkey::Pubkey;
                });
                break;
            }
        }
        res
    }

    gen_body_newtype_slice!();
}

struct IxCodegenModule<'a>(pub &'a [NamedInstruction]);

impl IdlCodegenModule for IxCodegenModule<'_> {
    fn name(&self) -> &str {
        "instructions"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            use solana_program::{
                account_info::AccountInfo,
                entrypoint::ProgramResult,
                instruction::{AccountMeta, Instruction},
                program::{invoke, invoke_signed},
                pubkey::Pubkey,
            };
        };
        for ix in self.0 {
            if ix
                .args
                .iter()
                .map(|a| a.r#type.is_or_has_defined())
                .any(|b| b)
            {
                res.extend(quote! {
                    use crate::*;
                });
                break;
            }
        }
        res
    }

    gen_body_newtype_slice!();
}

struct ErrorsCodegenModule<'a> {
    pub program_name: &'a str,
    pub variants: &'a [ErrorEnumVariant],
}

impl IdlCodegenModule for ErrorsCodegenModule<'_> {
    fn name(&self) -> &str {
        "errors"
    }

    fn gen_head(&self) -> TokenStream {
        quote! {
            use solana_program::{
                decode_error::DecodeError,
                msg,
                program_error::{PrintProgramError, ProgramError},
            };
            use thiserror::Error;
        }
    }

    fn gen_body(&self) -> TokenStream {
        let mut error_enum_variants = quote! {};
        error_enum_variants.extend(self.variants.iter().map(|e| e.into_token_stream()));

        let error_enum_ident_str = format!("{}Error", self.program_name.to_pascal_case());
        let error_enum_ident = format_ident!("{}", &error_enum_ident_str);
        quote! {
            #[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
            pub enum #error_enum_ident {
                #error_enum_variants
            }

            impl From<#error_enum_ident> for ProgramError {
                fn from(e: #error_enum_ident) -> Self {
                    ProgramError::Custom(e as u32)
                }
            }

            impl<T> DecodeError<T> for #error_enum_ident {
                fn type_of() -> &'static str {
                    #error_enum_ident_str
                }
            }

            impl PrintProgramError for #error_enum_ident {
                fn print<E>(&self)
                where
                    E: 'static
                        + std::error::Error
                        + DecodeError<E>
                        + PrintProgramError
                        + num_traits::FromPrimitive,
                {
                    msg!(&self.to_string());
                }
            }
        }
    }
}
