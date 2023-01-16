# marinade liquid staking program

IDL downloaded from https://github.com/marinade-finance/marinade-ts-sdk/blob/main/src/programs/idl/marinade-finance-idl.json

## Notes

- the original idl.json contained some typedefs that weren't in the src code: `enum CommonError` (different from the one in `errors.rs`) and `enum InitializeError`. These have been removed.