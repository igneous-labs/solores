use proc_macro2::TokenStream;
use quote::quote;

use crate::idl_format::IdlCodegenModule;

mod typedef;
pub use typedef::*;

pub struct TypedefsCodegenModule<'a> {
    pub cli_args: &'a crate::Args,
    pub named_types: &'a [NamedType],
}

impl IdlCodegenModule for TypedefsCodegenModule<'_> {
    fn name(&self) -> &str {
        "typedefs"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        for a in self.named_types {
            if self.cli_args.zero_copy.iter().any(|e| e == &a.name) {
                res.extend(quote! {
                    use bytemuck::{Pod, Zeroable};
                });
                break;
            }
        }
        for t in self.named_types {
            if t.r#type.has_pubkey_field() {
                res.extend(quote! {
                    use solana_program::pubkey::Pubkey;
                });
                break;
            }
        }
        res
    }

    fn gen_body(&self) -> TokenStream {
        self.named_types
            .iter()
            .map(|e| e.to_token_stream(self.cli_args))
            .collect()
    }
}
