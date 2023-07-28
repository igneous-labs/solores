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
        // discriminant
        let account_discm_ident = format_ident!("{}_ACCOUNT_DISCM", name.to_shouty_snake_case());
        // pre-image: "account:{AccountStructName}"
        let discm = <[u8; 8]>::try_from(
            &Sha256::digest(format!("account:{}", name.to_pascal_case()).as_bytes()).as_slice()
                [..8],
        )
        .unwrap();
        let discm_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();

        let struct_def = &self.0;

        let struct_ident = format_ident!("{}", name);
        let account_ident = format_ident!("{}Account", name);
        tokens.extend(quote! {
            pub const #account_discm_ident: [u8; 8] = #discm_tokens;

            #struct_def

            #[derive(Clone, Debug, PartialEq)]
            pub struct #account_ident(pub #struct_ident);

            impl BorshSerialize for #account_ident {
                fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    #account_discm_ident.serialize(writer)?;
                    self.0.serialize(writer)
                }
            }

            impl #account_ident {
                pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                    let maybe_discm = <[u8; 8]>::deserialize(buf)?;
                    if maybe_discm != #account_discm_ident {
                        return Err(
                            std::io::Error::new(
                                std::io::ErrorKind::Other, format!("discm does not match. Expected: {:?}. Received: {:?}", #account_discm_ident, maybe_discm)
                            )
                        );
                    }
                    Ok(Self(#struct_ident::deserialize(buf)?))
                }
            }
        });
    }
}
