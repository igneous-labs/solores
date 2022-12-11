mod write_accounts;
mod write_lib;
mod write_typedefs;

use std::{io::Write, path::Path};

use proc_macro2::TokenStream;

pub use write_accounts::*;
pub use write_lib::*;
pub use write_typedefs::*;

use crate::{utils::open_file_create_overwrite, Args};

pub fn write_src_file<P: AsRef<Path>>(
    args: &Args,
    src_file_path: P,
    contents: TokenStream,
) -> std::io::Result<()> {
    let unpretty = syn::parse2(contents).unwrap();
    let formatted = prettyplease::unparse(&unpretty);

    let path = args.output_dir.join(src_file_path);
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(formatted.as_bytes())
}
