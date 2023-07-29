use proc_macro2::TokenStream;
use quote::quote;

use crate::{gen_body_newtype_slice, idl_format::IdlCodegenModule};

mod instruction;
pub use instruction::*;

pub struct IxCodegenModule<'a>(pub &'a [NamedInstruction]);

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
