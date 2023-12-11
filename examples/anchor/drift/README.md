# drift v2 program

IDL downloaded from https://github.com/drift-labs/protocol-v2/blob/master/sdk/src/idl/drift.json

## Notes

- idl doesnt contain programId, so output crate will be set to default `TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111`
- only works with `solana-program = ^1.16` since it requires `borsh = ^0.10` which introduces the use of const generics in arrays