# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [UNRELEASED]

### Breaking

- Removed unused macro `gen_body_newtype_slice`

### Added

- Simple support for Pod/zero copy typedefs with the `-z` option

## [0.2.2] - 2023-07-30

### Changed

- made more stuff `pub` for the lib

### Added

- `*_verify_account_privileges()` generated function for instructions
- `*_verify_account_keys` generated function for instructions

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
