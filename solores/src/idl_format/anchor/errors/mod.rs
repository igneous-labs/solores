use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::idl_format::IdlCodegenModule;

mod error;
pub use error::*;

pub struct ErrorsCodegenModule<'a> {
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
        let error_enum_variants: TokenStream = self
            .variants
            .iter()
            .map(|e| e.into_token_stream())
            .collect();

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
