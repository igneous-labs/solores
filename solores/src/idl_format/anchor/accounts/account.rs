use heck::{ToPascalCase, ToShoutySnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::idl_format::anchor::typedefs::NamedType;
use crate::utils::conditional_pascal_case;

#[derive(Deserialize)]
pub struct NamedAccount(pub NamedType);

impl NamedAccount {
    pub fn to_token_stream(&self, cli_args: &crate::Args) -> TokenStream {
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

        let struct_def = self.0.to_token_stream(cli_args);

        let struct_ident = format_ident!("{}", conditional_pascal_case(name));
        let account_ident = format_ident!("{}Account", conditional_pascal_case(name));
        quote! {
            pub const #account_discm_ident: [u8; 8] = #discm_tokens;

            #struct_def

            #[derive(Clone, Debug, PartialEq)]
            pub struct #account_ident(pub #struct_ident);

            impl #account_ident {
                pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
                    use std::io::Read;
                    let mut reader = buf;
                    let mut maybe_discm = [0u8; 8];
                    reader.read_exact(&mut maybe_discm)?;
                    if maybe_discm != #account_discm_ident {
                        return Err(
                            std::io::Error::new(
                                std::io::ErrorKind::Other, format!("discm does not match. Expected: {:?}. Received: {:?}", #account_discm_ident, maybe_discm)
                            )
                        );
                    }
                    Ok(Self(#struct_ident::deserialize(&mut reader)?))
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    writer.write_all(&#account_discm_ident)?;
                    self.0.serialize(&mut writer)
                }

                pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
                    let mut data = Vec::new();
                    self.serialize(&mut data)?;
                    Ok(data)
                }
            }
        }
    }
}
