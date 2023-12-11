use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::idl_format::IdlCodegenModule;

mod instruction;
pub use instruction::*;

pub struct IxCodegenModule<'a> {
    pub program_name: &'a str,
    pub instructions: &'a [NamedInstruction],
}

impl IdlCodegenModule for IxCodegenModule<'_> {
    fn name(&self) -> &str {
        "instructions"
    }

    fn gen_head(&self) -> TokenStream {
        // imports
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            use solana_program::{
                account_info::AccountInfo,
                entrypoint::ProgramResult,
                instruction::{AccountMeta, Instruction},
                program::{invoke, invoke_signed},
                program_error::ProgramError,
                pubkey::Pubkey,
            };
        };
        for ix in self.instructions {
            if ix
                .args
                .iter()
                .map(|a| a.r#type.is_or_has_defined())
                .any(|b| b)
            {
                res.extend(quote! {
                    use crate::*;
                });
                break;
            }
        }

        // program ix enum
        let program_ix_enum_ident =
            format_ident!("{}ProgramIx", self.program_name.to_pascal_case());
        let program_ix_enum_variants = self.instructions.iter().map(enum_variant);
        let serialize_variant_match_arms =
            self.instructions.iter().map(serialize_variant_match_arm);
        let deserialize_variant_match_arms =
            self.instructions.iter().map(deserialize_variant_match_arm);

        res.extend(quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub enum #program_ix_enum_ident {
                #(#program_ix_enum_variants),*
            }

            impl #program_ix_enum_ident {
                pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
                    use std::io::Read;
                    let mut reader = buf;
                    let mut maybe_discm_buf = [0u8; 1];
                    reader.read_exact(&mut maybe_discm_buf)?;
                    let maybe_discm = maybe_discm_buf[0];
                    match maybe_discm {
                        #(#deserialize_variant_match_arms),*,
                        _ => Err(
                            std::io::Error::new(
                                std::io::ErrorKind::Other, format!("discm {:?} not found", maybe_discm)
                            )
                        ),
                    }
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    match self {
                        #(#serialize_variant_match_arms)*
                    }
                }

                pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
                    let mut data = Vec::new();
                    self.serialize(&mut data)?;
                    Ok(data)
                }
            }
        });

        res
    }

    fn gen_body(&self) -> TokenStream {
        self.instructions
            .iter()
            .map(|e| e.into_token_stream())
            .collect()
    }
}

pub fn enum_variant(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = format_ident!("{}", ix.name.to_pascal_case());
    let ix_args_ident = ix.ix_args_ident();
    quote!(
        #variant_ident(#ix_args_ident)
    )
}

pub fn serialize_variant_match_arm(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = format_ident!("{}", ix.name.to_pascal_case());
    let discm_ident = ix.discm_ident();
    quote! {
        Self::#variant_ident(args) => {
            #discm_ident.serialize(&mut writer)?;
            args.serialize(&mut writer)
        }
    }
}

pub fn deserialize_variant_match_arm(ix: &NamedInstruction) -> TokenStream {
    let variant_ident = format_ident!("{}", ix.name.to_pascal_case());
    let discm_ident = ix.discm_ident();
    let ix_args_ident = ix.ix_args_ident();
    quote! {
        #discm_ident => Ok(Self::#variant_ident(#ix_args_ident::deserialize(&mut reader)?))
    }
}
