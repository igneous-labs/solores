#![allow(non_camel_case_types)]

use std::str::FromStr;

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use syn::Index;
use void::Void;

use crate::utils::{primitive_or_pubkey_to_token, string_or_struct, PUBKEY_TOKEN};

#[derive(Deserialize)]
pub struct TypedefField {
    pub name: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub r#type: TypedefFieldType,
}

/// All instances should be annotated with
/// deserialize_with = "string_or_struct"
#[derive(Deserialize)]
pub enum TypedefFieldType {
    // handled by string_or_struct's string
    PrimitiveOrPubkey(String),

    // rest handled by string_or_struct's struct
    defined(String),

    array(TypedefFieldArray),

    #[serde(deserialize_with = "string_or_struct")]
    option(Box<TypedefFieldType>),

    #[serde(deserialize_with = "string_or_struct")]
    vec(Box<TypedefFieldType>),
}

#[derive(Deserialize)]
pub struct TypedefFieldArray(
    #[serde(deserialize_with = "string_or_struct")] Box<TypedefFieldType>,
    u32, // borsh spec says array sizes are u32
);

impl TypedefFieldType {
    pub fn is_or_has_pubkey(&self) -> bool {
        match self {
            Self::PrimitiveOrPubkey(s) => primitive_or_pubkey_to_token(s) == PUBKEY_TOKEN,
            Self::array(a) => a.0.is_or_has_pubkey(),
            Self::option(o) => o.is_or_has_pubkey(),
            Self::vec(v) => v.is_or_has_pubkey(),
            Self::defined(_) => false,
        }
    }

    pub fn is_or_has_defined(&self) -> bool {
        match self {
            Self::PrimitiveOrPubkey(_) => false,
            Self::array(a) => a.0.is_or_has_defined(),
            Self::option(o) => o.is_or_has_defined(),
            Self::vec(v) => v.is_or_has_defined(),
            Self::defined(_) => true,
        }
    }
}

impl FromStr for TypedefFieldType {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::PrimitiveOrPubkey(s.into()))
    }
}

impl FromStr for Box<TypedefFieldType> {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Box::new(TypedefFieldType::from_str(s)?))
    }
}

impl ToTokens for TypedefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.to_snake_case());
        let ty = &self.r#type;
        tokens.extend(quote! {
            #name: #ty
        })
    }
}

impl ToTokens for TypedefFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty: TokenStream = match self {
            Self::PrimitiveOrPubkey(s) => primitive_or_pubkey_to_token(s).parse().unwrap(),
            Self::defined(s) => s.parse().unwrap(),
            Self::array(a) => a.to_token_stream(),
            Self::vec(v) => quote! {
                Vec<#v>
            },
            Self::option(o) => quote! {
                Option<#o>
            },
        };
        tokens.extend(ty);
    }
}

impl ToTokens for TypedefFieldArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = &self.0;
        let n = Index::from(self.1 as usize);
        tokens.extend(quote! {
            [#ty; #n]
        })
    }
}
