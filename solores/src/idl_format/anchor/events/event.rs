use heck::{ToPascalCase, ToShoutySnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::idl_format::anchor::typedefs::TypedefField;

#[derive(Deserialize)]
pub struct Event(pub EventType);

#[derive(Deserialize)]
pub struct EventType {
    pub name: String,
    // NB: theres also an `index` field that's ignored for now since we dk what it does:
    // https://github.com/coral-xyz/anchor/blob/8f30f00ec363b7e82aa0b3c7041e912919b33cf5/lang/attribute/event/src/lib.rs#L62C1-L64
    pub fields: Vec<TypedefField>,
}

impl EventType {
    pub fn struct_ident(&self) -> Ident {
        format_ident!("{}", self.name.to_pascal_case())
    }
}

impl ToTokens for Event {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // discriminant
        let event_discm_ident = format_ident!("{}_EVENT_DISCM", self.0.name.to_shouty_snake_case());
        // pre-image: "event:{EventName}"
        let discm = <[u8; 8]>::try_from(
            &Sha256::digest(format!("event:{}", self.0.name).as_bytes()).as_slice()[..8],
        )
        .unwrap();
        let discm_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();

        let struct_def = &self.0;

        let struct_ident = self.0.struct_ident();
        let event_ident = format_ident!("{}Event", struct_ident);
        tokens.extend(quote! {
            pub const #event_discm_ident: [u8; 8] = #discm_tokens;

            #struct_def

            #[derive(Clone, Debug, PartialEq)]
            pub struct #event_ident(pub #struct_ident);

            impl BorshSerialize for #event_ident {
                fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    #event_discm_ident.serialize(writer)?;
                    self.0.serialize(writer)
                }
            }

            impl #event_ident {
                pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                    let maybe_discm = <[u8; 8]>::deserialize(buf)?;
                    if maybe_discm != #event_discm_ident {
                        return Err(
                            std::io::Error::new(
                                std::io::ErrorKind::Other, format!("discm does not match. Expected: {:?}. Received: {:?}", #event_discm_ident, maybe_discm)
                            )
                        );
                    }
                    Ok(Self(#struct_ident::deserialize(buf)?))
                }
            }
        });
    }
}

impl ToTokens for EventType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let struct_ident = self.struct_ident();
        let struct_fields = &self.fields;
        tokens.extend(quote! {
            #[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
            pub struct #struct_ident {
                #(#struct_fields),*
            }
        });
    }
}
