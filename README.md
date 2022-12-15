# solores

Solana IDL to Rust client / CPI interface generator.

> [solita](https://github.com/metaplex-foundation/solita), light of my life, fire of my loins

This software is still in its early stages of development. USE AT YOUR OWN RISK.

## Contents

- [solores](#solores)
  - [Contents](#contents)
  - [Supported IDL Formats](#supported-idl-formats)
  - [Installation](#installation)
  - [Examples](#examples)
    - [Shank IDL](#shank-idl)
    - [Anchor IDL](#anchor-idl)
  - [Features](#features)
    - [Instruction Function Generics](#instruction-function-generics)
  - [Comparison to similar libs](#comparison-to-similar-libs)
    - [anchor-gen](#anchor-gen)
  - [Known Missing Features](#known-missing-features)
    - [General](#general)
    - [Anchor](#anchor)

## Supported IDL Formats

- [Shank](https://github.com/metaplex-foundation/shank)
- [Anchor](https://github.com/coral-xyz/anchor)

## Installation

`cargo install solores` to install the CLI binary.

## Examples

### Shank IDL

Lets say you had the following shank generated IDL, `my_token_idl.json`:

```json
{
  "name": "my_token",
  "instructions": [
    {
      "name": "transfer",
      "accounts": [
        {
          "name": "src",
          "isMut": true,
          "isSigner": true,
        },
        {
          "name": "dest",
          "isMut": true,
          "isSigner": false,
        },
      ],
      "args": [
        {
          "name": "transferArgs",
          "type": {
            "defined": "TransferArgs"
          }
        }
      ],
      "discriminant": {
        "type": "u8",
        "value": 0
      }
    }
  ],
  "types": [
    {
      "name": "TransferArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
```

Running `solores my_token_idl.json` should generate a `my_token_interface` rust crate that allows you to use it in an on-chain program as so:

```rust
use my_token_interface::{TransferAccounts, TransferArgs, transfer_invoke_signed};
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let src = next_account_info(account_info_iter)?;
    let dest = next_account_info(account_info_iter)?;

    transfer_invoke_signed(
        &TransferAccounts { src, dest },
        TransferArgs { amount: 1_000u64 },
        &[&[&[0u8]]],
    )
}
```

or in a client-side app:

```rust
use my_token_interface::{TransferKeys, TransferArgs, transfer_ix};

pub fn do_something_with_instruction() -> std::io::Result<()> {
    ...

    let transfer_accounts = TransferKeys {
        src: some_pubkey,
        dest: another_pubkey,
    };
    let transfer_ix_args = TransferArgs { amount: 1_000u64 };
    let ix = transfer_ix(&transfer_accounts, transfer_ix_args)?;

    ...
}

```

The crate will also export the instructions' discriminant as consts, and any error types defined in the IDL as an enum convertible to/from u32.

### Anchor IDL

The usage for anchor IDLs is essentially the same as [Shank IDL's](#shank-idl). The crate will also export all accounts' discriminant as consts.

## Features

### Instruction Function Generics

The generated `*_ix()` function parameters are genericized over any type that impls `Into<*Keys>` for the first arg and any type that impls `Into<*Args>` for the second arg. This allows users to easily implement, for example, account structs that compute/retrieve known pubkeys (like PDAs) at runtime:

```rust
use my_token_interface::{TransferArgs, TransferKeys, ID};
use solana_program::pubkey::Pubkey;

struct MyTransferKeys {
    pub src: Pubkey,
}

impl From<&MyTransferKeys> for TransferKeys {
    fn from(my_transfer_keys: MyTransferKeys) -> Self {
        let (my_pda_dest, _bump) = Pubkey::find_program_address(
            &[&[0u8]],
            &ID,
        );
        Self {
          src: my_transfer_keys.src,
          dest: my_pda_dest,
        }
    }
}

struct MyTransferArgs {};

impl From<MyTransferArgs> for TransferArgs {
    fn from(_unused: MyTransferArgs) -> Self {
        Self {
            amount: 1000u64,
        }
    }
}

//  Now you can do:
//  let ix = transfer_ix(
//      &MyTransferKeys { src: my_pubkey },
//      MyTransferArgs {},
//  );
```

## Comparison to similar libs

### anchor-gen

Compared to [anchor-gen](https://github.com/saber-hq/anchor-gen), solores:

- Has no dependency on [anchor](https://github.com/coral-xyz/anchor). The generated crate's dependencies are [borsh](https://github.com/near/borsh-rs) + [solana-program](https://github.com/solana-labs/solana/tree/master/sdk/program), and also [thiserror](https://github.com/dtolnay/thiserror) + [num-derive](https://github.com/rust-num/num-derive) + [num-traits](https://github.com/rust-num/num-traits) if the idl contains error enum definitions.

- Produces (almost) human-readable rust code in a new, separate crate instead of using a proc-macro.

- Exposes lower-level constructs such as functions for creating the `solana_program::instruction::Instruction` struct to allow for greater customizability.

## Known Missing Features

Please check the repo's issues list for more.

### General

- Does not handle tuple struct enum variants like `pub enum MyEnum { MyVariant(field1: u8, field2: u64) }` 

### Anchor

- Does not handle account namespaces
- Does not handle the state instruction namespace
