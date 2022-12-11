use quote::ToTokens;

pub mod shank;

pub trait IdlFormat<TypedefElem: ToTokens, AccountElem: ToTokens, IxElem: ToTokens> {
    fn program_name(&self) -> &str;

    fn program_version(&self) -> &str;

    fn program_address(&self) -> &str;

    fn typedefs(&self) -> &[TypedefElem];

    fn accounts(&self) -> &[AccountElem];

    fn instructions(&self) -> &[IxElem];
}
