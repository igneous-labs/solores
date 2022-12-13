use quote::{quote, ToTokens};

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_lib<'a, T: ToTokens, A: ToTokens, I: ToTokens, Idl: IdlFormat<T, A, I>>(
    args: &'a Args,
    idl: &'a Idl,
) -> std::io::Result<()> {
    let id = idl.program_address();

    let maybe_accounts = match idl.accounts() {
        Some(_) => quote! {
            pub mod accounts;
            pub use accounts::*;
        },
        None => quote! {},
    };

    let maybe_ixs = match idl.instructions() {
        Some(_) => quote! {
            pub mod instructions;
            pub use instructions::*;
        },
        None => quote! {},
    };

    let maybe_typedefs = match idl.typedefs() {
        Some(_) => quote! {
            pub mod typedefs;
            pub use typedefs::*;
        },
        None => quote! {},
    };

    let contents = quote! {
        #maybe_accounts
        #maybe_ixs
        #maybe_typedefs

        solana_program::declare_id!(#id);
    };
    write_src_file(args, "src/lib.rs", contents)
}
