# solores

Solana IDL to Rust client / CPI interface generator.

> [solita](https://github.com/metaplex-foundation/solita), light of my life, fire of my loins

## Contents

- [solores](#solores)
  - [Contents](#contents)
  - [Supported IDL Formats](#supported-idl-formats)
  - [Installation](#installation)
  - [Examples](#examples)
    - [Shank IDL](#shank-idl)
  - [Features](#features)
    - [Instruction Function Generics](#instruction-function-generics)
  - [Comparison to similar libs](#comparison-to-similar-libs)
    - [anchor-gen](#anchor-gen)

## Supported IDL Formats

- [Shank](https://github.com/metaplex-foundation/shank)

Anchor coming soon.

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

- Has no dependency on [anchor](https://github.com/coral-xyz/anchor). The generated crate's only dependencies are [borsh](https://github.com/near/borsh-rs) and [solana-program](https://github.com/solana-labs/solana/tree/master/sdk/program), making it suitable for use in on-chain programs (both anchor and non-anchor) and client programs.

- Produces (almost) human-readable rust code in a new, separate crate instead of using a proc-macro.

- Exposes lower-level constructs such as functions for creating the `solana_program::instruction::Instruction` struct to allow for greater customizability.
