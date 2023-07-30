#![cfg_attr(not(test), forbid(unsafe_code))]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
