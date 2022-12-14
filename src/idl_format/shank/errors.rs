use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorEnumVariant {
    code: u32,
    name: String,
    msg: String,
}

impl ToTokens for ErrorEnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_ident = format_ident!("{}", self.name.to_pascal_case());
        let msg = &self.msg;
        let code = self.code;
        tokens.extend(quote! {
            #[error(#msg)]
            #variant_ident = #code,
        })
    }
}
