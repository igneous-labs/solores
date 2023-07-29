use proc_macro2::TokenStream;
use quote::quote;

use crate::{gen_body_newtype_slice, idl_format::IdlCodegenModule};

mod typedef;
pub use typedef::*;

pub struct TypedefsCodegenModule<'a>(pub &'a [NamedType]);

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
