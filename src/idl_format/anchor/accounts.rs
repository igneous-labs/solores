use heck::{ToPascalCase, ToShoutySnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::typedefs::NamedType;

#[derive(Deserialize)]
pub struct NamedAccount(pub NamedType);

impl ToTokens for NamedAccount {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.0.name;
        let account_discm_ident = format_ident!("{}_ACCOUNT_DISCM", name.to_shouty_snake_case());
        // pre-image: "account:{AccountStructName}"
        let discm = <[u8; 8]>::try_from(
            &Sha256::digest(format!("account:{}", name.to_pascal_case()).as_bytes()).as_slice()
                [..8],
        )
        .unwrap();
        let discm_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();
        tokens.extend(quote! {
            pub const #account_discm_ident: [u8; 8] = #discm_tokens;
        });
        tokens.extend(self.0.to_token_stream());
    }
}
