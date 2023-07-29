use proc_macro2::TokenStream;
use quote::quote;

use crate::{gen_body_newtype_slice, idl_format::IdlCodegenModule};

use super::typedefs::NamedType;

pub struct AccountsCodegenModule<'a>(pub &'a [NamedType]);

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
            if a.r#type.has_pubkey_field() && !has_pubkey {
                has_pubkey = true;
                res.extend(quote! {
                    use solana_program::pubkey::Pubkey;
                });
            }
            if a.r#type.has_defined_field() && !has_defined {
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
