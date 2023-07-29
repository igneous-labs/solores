# solores

Solana IDL to Rust client / CPI interface generator.

> [solita](https://github.com/metaplex-foundation/solita), light of my life, fire of my loins

This software is still in its early stages of development. USE AT YOUR OWN RISK.

## Contents

- [solores](#solores)
  * [Contents](#contents)
  * [Supported IDL Formats](#supported-idl-formats)
  * [Installation](#installation)
  * [Examples](#examples)
    + [Shank IDL](#shank-idl)
    + [Anchor IDL](#anchor-idl)
  * [Features](#features)
    + [Instruction Function Generics](#instruction-function-generics)
    + [Serde](#serde)
    + [Keys from array](#keys-from-array)
    + [Accounts from array](#accounts-from-array)
  * [Comparison to similar libs](#comparison-to-similar-libs)
    + [anchor-gen](#anchor-gen)
  * [Known Missing Features](#known-missing-features)
    + [General](#general)
    + [Anchor](#anchor)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>

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

```rust ignore
use my_token_interface::{TransferAccounts, TransferArgs, TransferIxArgs, transfer_invoke_signed};
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let src = next_account_info(account_info_iter)?;
    let dest = next_account_info(account_info_iter)?;

    transfer_invoke_signed(
        &TransferAccounts { src, dest },
        TransferIxArgs {
            transfer_args: TransferArgs { amount: 1_000 },
        },
        &[&[&[0u8]]],
    )
}
```

or in a client-side app:

```rust ignore
use my_token_interface::{TransferKeys, TransferArgs, transfer_ix};

pub fn do_something_with_instruction() -> std::io::Result<()> {
    ...

    let transfer_accounts = TransferKeys {
        src: some_pubkey,
        dest: another_pubkey,
    };
    let transfer_ix_args = TransferIxArgs {
        transfer_args: TransferArgs { amount: 1_000 },
    };
    let ix = transfer_ix(&transfer_accounts, transfer_ix_args)?;

    ...
}

```

The crate will also combine all instructions into a single borsh de/serializable `ProgramIx` enum

```rust ignore
use borsh::BorshSerialize;
use my_token_interface::{MyTokenProgramIx, TransferArgs, TransferIxArgs};

#[test]
pub fn test_borsh_serde_roundtrip_program_ix() {
    let program_ix = MyTokenProgramIx::Transfer(
        TransferIxArgs {
            transfer_args: TransferArgs { amount: 1 },
        }
    );

    // [0, 1, 0, 0, 0, 0, 0, 0, 0]
    let serialized = program_ix.try_to_vec().unwrap();

    // note that deserialize is an associated function/method
    // rather than the BorshDeserialize trait impl,
    // i.e. MyTokenProgramIx does NOT impl BorshDeserialize
    // because the definition of BorshDeserialize changed between borsh 0.9 and 0.10
    let deserialized = MyTokenProgramIx::deserialize(&mut serialized.as_ref()).unwrap();
    assert_eq!(program_ix, deserialized);
}
```

The crate will also export the instructions' discriminant as consts, and any error types defined in the IDL as an enum convertible to/from u32.

### Anchor IDL

The usage for anchor IDLs is essentially the same as [Shank IDL's](#shank-idl). Additionally, the crate will also:

- export all accounts' discriminant as consts.
- create a `*Account` newtype that includes account discriminant checking in borsh serde operations
- export event struct defs

## Features

### Instruction Function Generics

The generated `*_ix()` function parameters are genericized over any type that impls `Into<*Keys>` for the first arg and any type that impls `Into<*IxArgs>` for the second arg. This allows users to easily implement, for example, account structs that compute/retrieve known pubkeys (like PDAs) at runtime:

```rust ignore
use my_token_interface::{TransferArgs, TransferIxArgs, TransferKeys, ID};
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

impl From<MyTransferArgs> for TransferIxArgs {
    fn from(_unused: MyTransferArgs) -> Self {
        Self {
            transfer_args: TransferArgs { amount: 1000 },
        }
    }
}

//  Now you can do:
//  let ix = transfer_ix(
//      &MyTransferKeys { src: my_pubkey },
//      MyTransferArgs {},
//  );
```

### Serde

`serde` is added as an optional dependency behind the `serde` feature-flag to the generated crate to provide `Serialize` and `Deserialize` implementations for the various typedefs and onchain accounts.

### Keys from array

The various `*Keys` struct also impl `From<[Pubkey; *_IX_ACCOUNTS_LEN]>` to support indexing

```rust ignore
use my_token_interface::{TRANSFER_IX_ACCOUNTS_LEN, TransferKeys};
use solana_program::{pubkey::Pubkey, sysvar::instructions::{BorrowedAccountMeta, BorrowedInstruction}};
use std::convert::TryInto;

fn index_instruction(ix: BorrowedInstruction) {
    let metas: [BorrowedAccountMeta<'_>; TRANSFER_IX_ACCOUNTS_LEN] = ix.accounts.try_into().unwrap();
    let pubkeys = metas.map(|meta| *meta.pubkey);
    let transfer_keys: TransferKeys = pubkeys.into();

    // Now you can do stuff like `transfer_keys.src` instead of
    // having to keep track of the various account indices
    // 
    // ... 
}
```

### Accounts from array

The various `*Accounts` also impl `From<&[AccountInfo; *_IX_ACCOUNTS_LEN]>` to make simple CPIs more ergonomic

```rust ignore
use my_token_interface::{TRANSFER_IX_ACCOUNTS_LEN, TransferAccounts, TransferArgs, TransferIxArgs, transfer_invoke_signed};
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let transfer_accounts: &[AccountInfo; TRANSFER_IX_ACCOUNTS_LEN] = accounts[..TRANSFER_IX_ACCOUNTS_LEN].try_into().unwrap();
    let accounts: TransferAccounts = transfer_accounts.into();

    transfer_invoke_signed(
        &accounts,
        TransferIxArgs {
            transfer_args: TransferArgs { amount: 1_000 },
        },
        &[&[&[0u8]]],
    )
}
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

- Assumes all onchain accounts are borsh-serde, does not handle zero-copy/bytemuck accounts.

### Anchor

- Does not handle account namespaces
- Does not handle the state instruction namespace
