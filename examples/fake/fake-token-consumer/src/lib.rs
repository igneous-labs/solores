#![cfg_attr(not(test), forbid(unsafe_code))]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

solana_program::declare_id!("11111111111111111111111111111111");
