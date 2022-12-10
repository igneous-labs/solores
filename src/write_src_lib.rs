use quote::quote;
use std::io::Write;

use crate::{idl_format::IdlFormat, utils::open_file_create_overwrite, Args};

pub fn write_src_lib<'a, I: IdlFormat>(args: &'a Args, idl: &'a I) -> std::io::Result<()> {
    let id = idl.program_address();
    let contents = quote! {
        pub mod accounts;
        pub mod instructions;
        pub mod typedefs;

        pub use accounts::*;
        pub use instructions::*;
        pub use typedefs::*;

        solana_program::declare_id!(#id);
    };
    let unpretty = syn::parse2(contents).unwrap();
    let formatted = prettyplease::unparse(&unpretty);

    let path = args.output_dir.join("src/lib.rs");
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(formatted.as_bytes())
}
