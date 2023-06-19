# marinade liquid staking program

IDL downloaded from https://github.com/marinade-finance/marinade-ts-sdk/blob/main/src/programs/idl/json/marinade_finance.json

## Notes

- the original idl.json contained some typedefs that weren't in the src code: `enum CommonError` (different from the one in `errors.rs`) and `enum InitializeError`. These have been removed.
- idl doesnt contain programId, so output crate will be set to default `11111111111111111111111111111111`