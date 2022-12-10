pub mod shank;

pub trait IdlFormat {
    fn program_name(&self) -> &str;

    fn program_version(&self) -> &str;

    fn program_address(&self) -> &str;
}
