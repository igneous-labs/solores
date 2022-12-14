use quote::{quote, ToTokens};

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_lib<'a, T: ToTokens, A: ToTokens, I: ToTokens, Idl: IdlFormat<T, A, I>>(
    args: &'a Args,
    idl: &'a Idl,
) -> std::io::Result<()> {
    let id = idl.program_address();

    let mut contents = quote! {
        solana_program::declare_id!(#id);
    };

    if idl.accounts().is_some() {
        contents.extend(quote! {
            pub mod accounts;
            pub use accounts::*;
        })
    }

    if idl.instructions().is_some() {
        contents.extend(quote! {
            pub mod instructions;
            pub use instructions::*;
        })
    }

    if idl.typedefs().is_some() {
        contents.extend(quote! {
            pub mod typedefs;
            pub use typedefs::*;
        })
    }

    write_src_file(args, "src/lib.rs", contents)
}
