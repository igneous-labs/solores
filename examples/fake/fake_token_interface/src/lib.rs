pub mod accounts;
pub mod instructions;
pub mod typedefs;

pub use accounts::*;
pub use instructions::*;
pub use typedefs::*;

solana_program::declare_id!("11111111111111111111111111111111");
