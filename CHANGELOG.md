# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2024-08-23

### Breaking

- Updated all dependencies to latest compatible versions.

### Changed

- Anchor event struct fields are now public.

### Added

- `bytes_to_u8` feature that replaces all generated primitive instances of `bytes` with `u8`.

### Fixed

- Anchor `accounts` and `typedefs` generated names now conform to Rust PascalCase convention.

## [0.7.0] - 2023-12-28

### Breaking

- Changed instruction codegen internals to support `*_with_program_id()`
- Changed `IdlFormat` trait def to make dependency and `Cargo.toml` writing more dyn
- Now panics instead of continuing if accounts with the same name are detected in the any IDL instruction.

### Added

- bincode support
- `*_ix_with_program_id()`, `*_invoke_with_program_id()`, `*_invoke_signed_with_program_id()`

## [0.6.1] - 2023-12-27

### Fixed

- imports for `instructions.rs` not importing `solana_program::pubkey::Pubkey` if no instruction has accounts but some args has pubkey

## [0.6.0] - 2023-12-27

### Breaking

- Removed instruction function generics

### Changed

- split `_verify_account_privileges()` to be composed of 2 separate functions: `_verify_writable_privileges()` and `_verify_signer_privileges()`

### Fixed

- Add required `derive` feature to `bytemuck` dependency

## [0.5.0] - 2023-12-12

### Breaking

- Program ID now defaults to `TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111` instead of system program ID if ID not provided in IDL
- `*IxData`, `*ProgramIx` and anchor accounts no longer implement `BorshSerialize` since it does not follow the borsh spec. The methods have been moved to their intrinsic impl.
- Change `*IxData` and `*ProgramIx`s' `deserialize()` fn signature to accept `&[u8]` instead of `&mut &[u8]`. `&mut &[u8]` was previously used for borsh compatibility.
- No longer generates `*_verify_account_privileges()` function if instruction has no privileged accounts (only non-signer and non-writable accounts).
- No longer generates `*IxArgs` struct if no instruction args.
- No longer generates `*Accounts` `*Keys` structs and `*_verify_account_keys()` function if instruction has no account inputs.
- All reference types for `*Keys` and `*Accounts` have been changed to pass by value since they impl `Copy`
- Replaced `solana-program` dependency with `bs58`

### Changed

- `From <XKeys> for [AccountMeta; LEN]` now uses direct struct initialization instead of the `AccountMeta` constructor functions

## [0.4.0] - 2023-08-07

### Breaking

- `*_verify_account_privileges()` functions now return `Err((&AccountInfo, ProgramError))` on err where `&AccountInfo` is the first offending account.

## [0.3.0] - 2023-08-03

### Breaking

- Removed unused macro `gen_body_newtype_slice`

### Added

- Simple support for Pod/zero copy typedefs with the `-z` option

## [0.2.2] - 2023-07-30

### Changed

- made more stuff `pub` for the lib

### Added

- `*_verify_account_privileges()` generated function for instructions
- `*_verify_account_keys()` generated function for instructions

## [0.2.1] - 2023-07-30

### Changed

- document `--program-id` default values in `--help`
- all internal modules are now `pub` to allow people to use the library

## [0.2.0] - 2023-07-30

### Breaking

- Removed variable lifetimes from the various `AccountInfo`s in `*Accounts`. They all now share the same lifetime `'info`
- Changed the various `*IxData` structs to own the `*IxArgs` structs instead of a reference

### Added

- `impl From<[Pubkey; *_IX_ACCOUNTS_LEN]> for *Keys` for easier indexing
- `impl From<&[AccountInfo; *_IX_ACCOUNTS_LEN]> for *Accounts` for easier CPIs
- `deserialize` method for `*IxData`. Not using `BorshDeserialize` trait due to breaking change in trait def between 0.9 and 0.10
- `derive(PartialEq)` for all typedefs and `*IxArgs` and `*IxData`
- `**Account` for anchor accounts newtype that includes the discriminant in borsh serde
- Added `--program-id` option to allow setting of custom program IDs to accomodate anchor IDLs not containing their program IDs.

### Changed

- internals refactored to enable dynamic modules for the generated crate
- Anchor: standardize `*IxArgs` to be PascalCase per rust convention instead of camelCase in the IDL

### Fixed

- Bug that wasn't including the generated `errors` module into the crate

## [0.1.4] - 2023-07-21

### Added

- `--serde-vers` to configure `serde` as an optional dependency for the generated crate

### Changed

- Allow toml maps to be passed to the various `--*-vers` options to allow for values like `"workspace = true"`

## [0.1.3] - 2023-06-19

### Changed

- Upgrade default solana-program version to `^1.16` and borsh version to `^0.10`

## [0.1.2] - 2023-01-27

### Fixed

- Handle inner `Accounts<'_>` struct for anchor.

## [0.1.1] - 2023-01-09

### Fixed

- `defined` types being incorrectly converted to pascal case
- `metadata` field is now optional for anchor IDLs and program address is set to `11111111111111111111111111111111`, with warning logged, if not present

### Added

- Support for tuple enums

## [0.1.0] - 2022-12-15

Initial release
