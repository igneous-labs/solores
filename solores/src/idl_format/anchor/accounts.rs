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
        tokens.extend(quote! {
            pub const #account_discm_ident: [u8; 8] = #discm_tokens;
        });
        // struct def
        tokens.extend(self.0.to_token_stream());
        // TODO: BorshDeserialize trait definition changed between 0.9 - 0.10
        // Not sure how to do this while enabling workspace dependency of borsh
        /*
        // impl borsh=0.10 BorshSerialize and BorshDeserialize for account newtype
        let struct_ident = format_ident!("{}", name);
        let account_ident = format_ident!("{}Account", name);
        tokens.extend(quote! {
            #[derive(Clone, Debug)]
            pub struct #account_ident(pub #struct_ident);

            impl BorshSerialize for #account_ident {
                fn serialize<W: borsh::maybestd::io::Write>(&self, writer: &mut W) -> borsh::maybestd::io::Result<()> {
                    #account_discm_ident.serialize(writer)?;
                    self.0.serialize(writer)
                }
            }

            impl BorshDeserialize for #account_ident  {
                fn deserialize_reader<R: borsh::maybestd::io::Read>(reader: &mut R) -> borsh::maybestd::io::Result<Self> {
                    let discm = <[u8; 8]>::deserialize_reader(reader)?;
                    if discm != #account_discm_ident {
                        return Err(borsh::maybestd::io::Error::new(borsh::maybestd::io::ErrorKind::Other, "Account discriminant does not match"));
                    }
                    Ok(Self(#struct_ident::deserialize_reader(reader)?))
                }
            }
        });
         */
    }
}
