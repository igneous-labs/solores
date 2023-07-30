use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::idl_format::IdlCodegenModule;

mod event;
pub use event::*;

pub struct EventsCodegenModule<'a>(pub &'a [Event]);

impl IdlCodegenModule for EventsCodegenModule<'_> {
    fn name(&self) -> &str {
        "events"
    }

    fn gen_head(&self) -> TokenStream {
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        let mut has_pubkey = false;
        let mut has_defined = false;
        for a in self.0 {
            for field in &a.0.fields {
                if field.r#type.is_or_has_pubkey() && !has_pubkey {
                    has_pubkey = true;
                    res.extend(quote! {
                        use solana_program::pubkey::Pubkey;
                    });
                }
                if field.r#type.is_or_has_defined() && !has_defined {
                    has_defined = true;
                    res.extend(quote! {
                        use crate::*;
                    })
                }
            }
            if has_defined && has_pubkey {
                break;
            }
        }
        res
    }

    fn gen_body(&self) -> TokenStream {
        self.0.iter().map(|e| e.into_token_stream()).collect()
    }
}
