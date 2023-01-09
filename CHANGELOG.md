# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [UNRELEASED]

### Fixed

- `defined` types being incorrectly converted to pascal case
- `metadata` field is now optional for anchor IDLs and program address is set to `11111111111111111111111111111111`, with warning logged, if not present

### Added

- Support for tuple enums

## [0.1.0] - 2022-12-15

Initial release
