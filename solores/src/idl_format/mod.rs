#![doc = include_str!("./README.md")]

use proc_macro2::TokenStream;
use toml::{map::Map, Value};

pub mod anchor;
pub mod shank;

pub trait IdlCodegenModule {
    /// The module file's name e.g. "errors"
    fn name(&self) -> &str;

    /// Generate the headers to prefix the module file with.
    /// Typically import statements
    fn gen_head(&self) -> TokenStream;

    /// Generate the main body content of the module file
    fn gen_body(&self) -> TokenStream;
}

pub trait IdlFormat {
    fn program_name(&self) -> &str;

    fn program_version(&self) -> &str;

    fn program_address(&self) -> Option<&str>;

    fn is_correct_idl_format(&self) -> bool;

    fn dependencies(&self, args: &crate::Args) -> Map<String, Value>;

    fn modules<'me>(&'me self, args: &'me crate::Args) -> Vec<Box<dyn IdlCodegenModule + 'me>>;
}
