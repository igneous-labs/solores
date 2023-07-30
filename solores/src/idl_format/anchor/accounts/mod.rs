use proc_macro2::TokenStream;
use quote::quote;

use crate::idl_format::IdlCodegenModule;

mod account;
pub use account::*;

pub struct AccountsCodegenModule<'a> {
    pub cli_args: &'a crate::Args,
    pub named_accounts: &'a [NamedAccount],
}

impl IdlCodegenModule for AccountsCodegenModule<'_> {
    fn name(&self) -> &str {
        "accounts"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        for a in self.named_accounts {
            if self.cli_args.zero_copy.iter().any(|e| e == &a.0.name) {
                res.extend(quote! {
                    use bytemuck::{Pod, Zeroable};
                });
                break;
            }
        }
        let mut has_pubkey = false;
        let mut has_defined = false;
        for a in self.named_accounts {
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

    fn gen_body(&self) -> TokenStream {
        self.named_accounts
            .iter()
            .map(|e| e.to_token_stream(self.cli_args))
            .collect()
    }
}
