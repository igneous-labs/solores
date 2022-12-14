use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub mod shank;

#[derive(Copy, Clone, Debug, Default)]
pub struct TypedefsHeaderFlags {
    pub has_pubkey: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AccountsHeaderFlags {
    pub has_pubkey: bool,
    pub has_defined: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct InstructionsHeaderFlags {
    pub has_defined: bool,
}

pub trait IdlFormat<TypedefElem: ToTokens, AccountElem: ToTokens, IxElem: ToTokens> {
    fn program_name(&self) -> &str;

    fn program_version(&self) -> &str;

    fn program_address(&self) -> &str;

    fn typedefs(&self) -> Option<&[TypedefElem]>;

    fn accounts(&self) -> Option<&[AccountElem]>;

    fn instructions(&self) -> Option<&[IxElem]>;

    fn is_correct_idl_format(&self) -> bool;

    fn det_typedefs_header_flags(&self) -> TypedefsHeaderFlags;

    fn det_accounts_header_flags(&self) -> AccountsHeaderFlags;

    fn det_instructions_header_flags(&self) -> InstructionsHeaderFlags;

    fn typedefs_header(&self) -> TokenStream {
        let flags = self.det_typedefs_header_flags();
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        if flags.has_pubkey {
            res.extend(quote! {
                use solana_program::pubkey::Pubkey;
            });
        }
        res
    }

    fn accounts_header(&self) -> TokenStream {
        let flags = self.det_accounts_header_flags();
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
        };
        if flags.has_pubkey {
            res.extend(quote! {
                use solana_program::pubkey::Pubkey;
            });
        }
        if flags.has_defined {
            res.extend(quote! {
                use crate::*;
            })
        }
        res
    }

    fn instructions_header(&self) -> TokenStream {
        let flags = self.det_instructions_header_flags();
        let mut res = quote! {
            use borsh::{BorshDeserialize, BorshSerialize};
            use solana_program::{
                account_info::AccountInfo,
                entrypoint::ProgramResult,
                instruction::{AccountMeta, Instruction},
                program::{invoke, invoke_signed},
                pubkey::Pubkey,
            };
        };
        if flags.has_defined {
            res.extend(quote! {
                use crate::*;
            })
        }
        res
    }
}
