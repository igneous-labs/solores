#![allow(non_camel_case_types)]

use std::str::FromStr;

use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use syn::Index;
use void::Void;

use crate::utils::string_or_struct;

#[derive(Deserialize)]
pub struct NamedType {
    pub name: String,
    pub r#type: TypedefType,
}

#[derive(Deserialize)]
#[serde(tag = "kind")]
pub enum TypedefType {
    r#struct(TypedefStruct),
    r#enum(TypedefEnum),
}

#[derive(Deserialize)]
pub struct TypedefStruct {
    pub fields: Vec<TypedefField>,
}

#[derive(Deserialize)]
pub struct TypedefField {
    pub name: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub r#type: TypedefFieldType,
}

#[derive(Deserialize)]
pub enum TypedefFieldType {
    PrimitiveOrPubkey(String),
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

#[derive(Deserialize)]
pub struct TypedefEnum {
    pub variants: Vec<EnumVariant>,
}

// TODO: ENUMS WITH STRUCTS IN THEM
#[derive(Deserialize)]
pub struct EnumVariant {
    pub name: String,
}

impl ToTokens for NamedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let def = match &self.r#type {
            TypedefType::r#struct(typedef_struct) => quote! {
                #[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
                pub struct #name {
                    #typedef_struct
                }
            },
            TypedefType::r#enum(typedef_enum) => quote! {
                #[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
                pub enum #name {
                    #typedef_enum
                }
            },
        };
        tokens.extend(def);
    }
}

impl ToTokens for TypedefStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = self.fields.iter();
        tokens.extend(quote! {
            #(#fields),*
        })
    }
}

impl ToTokens for TypedefField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.to_snake_case());
        let ty = &self.r#type;
        tokens.extend(quote! {
            pub #name: #ty
        })
    }
}

impl ToTokens for TypedefFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty: TokenStream = match self {
            Self::PrimitiveOrPubkey(s) => primitive_or_pubkey_to_token(s).parse().unwrap(),
            Self::defined(s) => s.to_pascal_case().parse().unwrap(),
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

impl ToTokens for TypedefEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variants = &self.variants;
        tokens.extend(quote! {
            #(#variants),*
        })
    }
}

// TODO: handle complex enum structs
impl ToTokens for EnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let v = format_ident!("{}", self.name.to_pascal_case());
        tokens.extend(quote! {
            #v
        })
    }
}

const PUBKEY_TOKEN: &str = "Pubkey";

fn primitive_or_pubkey_to_token(s: &str) -> String {
    if s == "publicKey" {
        PUBKEY_TOKEN.to_owned()
    } else if s == "string" {
        s.to_pascal_case()
    } else {
        s.to_owned()
    }
}

impl TypedefType {
    pub fn has_pubkey_field(&self) -> bool {
        match self {
            Self::r#enum(_) => false,
            Self::r#struct(s) => s.fields.iter().any(|f| f.r#type.is_or_has_pubkey()),
        }
    }

    pub fn has_defined_field(&self) -> bool {
        match self {
            Self::r#enum(_) => false,
            Self::r#struct(s) => s.fields.iter().any(|f| f.r#type.is_or_has_defined()),
        }
    }
}

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
