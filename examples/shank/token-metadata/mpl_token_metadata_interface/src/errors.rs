use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MplTokenMetadataError {
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError = 0u32,
    #[error("Failed to pack instruction data")]
    InstructionPackError = 1u32,
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt = 2u32,
    #[error("Already initialized")]
    AlreadyInitialized = 3u32,
    #[error("Uninitialized")]
    Uninitialized = 4u32,
    #[error(" Metadata's key must match seed of ['metadata', program id, mint] provided")]
    InvalidMetadataKey = 5u32,
    #[error("Edition's key must match seed of ['metadata', program id, name, 'edition'] provided")]
    InvalidEditionKey = 6u32,
    #[error("Update Authority given does not match")]
    UpdateAuthorityIncorrect = 7u32,
    #[error("Update Authority needs to be signer to update metadata")]
    UpdateAuthorityIsNotSigner = 8u32,
    #[error("You must be the mint authority and signer on this transaction")]
    NotMintAuthority = 9u32,
    #[error("Mint authority provided does not match the authority on the mint")]
    InvalidMintAuthority = 10u32,
    #[error("Name too long")]
    NameTooLong = 11u32,
    #[error("Symbol too long")]
    SymbolTooLong = 12u32,
    #[error("URI too long")]
    UriTooLong = 13u32,
    #[error(
        "Update authority must be equivalent to the metadata's authority and also signer of this transaction"
    )]
    UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner = 14u32,
    #[error("Mint given does not match mint on Metadata")]
    MintMismatch = 15u32,
    #[error("Editions must have exactly one token")]
    EditionsMustHaveExactlyOneToken = 16u32,
    #[error("Maximum editions printed already")]
    MaxEditionsMintedAlready = 17u32,
    #[error("Token mint to failed")]
    TokenMintToFailed = 18u32,
    #[error("The master edition record passed must match the master record on the edition given")]
    MasterRecordMismatch = 19u32,
    #[error("The destination account does not have the right mint")]
    DestinationMintMismatch = 20u32,
    #[error("An edition can only mint one of its kind!")]
    EditionAlreadyMinted = 21u32,
    #[error("Printing mint decimals should be zero")]
    PrintingMintDecimalsShouldBeZero = 22u32,
    #[error("OneTimePrintingAuthorization mint decimals should be zero")]
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero = 23u32,
    #[error("EditionMintDecimalsShouldBeZero")]
    EditionMintDecimalsShouldBeZero = 24u32,
    #[error("Token burn failed")]
    TokenBurnFailed = 25u32,
    #[error("The One Time authorization mint does not match that on the token account!")]
    TokenAccountOneTimeAuthMintMismatch = 26u32,
    #[error("Derived key invalid")]
    DerivedKeyInvalid = 27u32,
    #[error("The Printing mint does not match that on the master edition!")]
    PrintingMintMismatch = 28u32,
    #[error("The One Time Printing Auth mint does not match that on the master edition!")]
    OneTimePrintingAuthMintMismatch = 29u32,
    #[error("The mint of the token account does not match the Printing mint!")]
    TokenAccountMintMismatch = 30u32,
    #[error("The mint of the token account does not match the master metadata mint!")]
    TokenAccountMintMismatchV2 = 31u32,
    #[error("Not enough tokens to mint a limited edition")]
    NotEnoughTokens = 32u32,
    #[error(
        "The mint on your authorization token holding account does not match your Printing mint!"
    )]
    PrintingMintAuthorizationAccountMismatch = 33u32,
    #[error(
        "The authorization token account has a different owner than the update authority for the master edition!"
    )]
    AuthorizationTokenAccountOwnerMismatch = 34u32,
    #[error("This feature is currently disabled.")]
    Disabled = 35u32,
    #[error("Creators list too long")]
    CreatorsTooLong = 36u32,
    #[error("Creators must be at least one if set")]
    CreatorsMustBeAtleastOne = 37u32,
    #[error("If using a creators array, you must be one of the creators listed")]
    MustBeOneOfCreators = 38u32,
    #[error("This metadata does not have creators")]
    NoCreatorsPresentOnMetadata = 39u32,
    #[error("This creator address was not found")]
    CreatorNotFound = 40u32,
    #[error("Basis points cannot be more than 10000")]
    InvalidBasisPoints = 41u32,
    #[error("Primary sale can only be flipped to true and is immutable")]
    PrimarySaleCanOnlyBeFlippedToTrue = 42u32,
    #[error("Owner does not match that on the account given")]
    OwnerMismatch = 43u32,
    #[error("This account has no tokens to be used for authorization")]
    NoBalanceInAccountForAuthorization = 44u32,
    #[error("Share total must equal 100 for creator array")]
    ShareTotalMustBe100 = 45u32,
    #[error("This reservation list already exists!")]
    ReservationExists = 46u32,
    #[error("This reservation list does not exist!")]
    ReservationDoesNotExist = 47u32,
    #[error("This reservation list exists but was never set with reservations")]
    ReservationNotSet = 48u32,
    #[error("This reservation list has already been set!")]
    ReservationAlreadyMade = 49u32,
    #[error("Provided more addresses than max allowed in single reservation")]
    BeyondMaxAddressSize = 50u32,
    #[error("NumericalOverflowError")]
    NumericalOverflowError = 51u32,
    #[error("This reservation would go beyond the maximum supply of the master edition!")]
    ReservationBreachesMaximumSupply = 52u32,
    #[error("Address not in reservation!")]
    AddressNotInReservation = 53u32,
    #[error("You cannot unilaterally verify another creator, they must sign")]
    CannotVerifyAnotherCreator = 54u32,
    #[error("You cannot unilaterally unverify another creator")]
    CannotUnverifyAnotherCreator = 55u32,
    #[error("In initial reservation setting, spots remaining should equal total spots")]
    SpotMismatch = 56u32,
    #[error("Incorrect account owner")]
    IncorrectOwner = 57u32,
    #[error("printing these tokens would breach the maximum supply limit of the master edition")]
    PrintingWouldBreachMaximumSupply = 58u32,
    #[error("Data is immutable")]
    DataIsImmutable = 59u32,
    #[error("No duplicate creator addresses")]
    DuplicateCreatorAddress = 60u32,
    #[error("Reservation spots remaining should match total spots when first being created")]
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart = 61u32,
    #[error("Invalid token program")]
    InvalidTokenProgram = 62u32,
    #[error("Data type mismatch")]
    DataTypeMismatch = 63u32,
    #[error("Beyond alotted address size in reservation!")]
    BeyondAlottedAddressSize = 64u32,
    #[error("The reservation has only been partially alotted")]
    ReservationNotComplete = 65u32,
    #[error("You cannot splice over an existing reservation!")]
    TriedToReplaceAnExistingReservation = 66u32,
    #[error("Invalid operation")]
    InvalidOperation = 67u32,
    #[error("Invalid Owner")]
    InvalidOwner = 68u32,
    #[error("Printing mint supply must be zero for conversion")]
    PrintingMintSupplyMustBeZeroForConversion = 69u32,
    #[error("One Time Auth mint supply must be zero for conversion")]
    OneTimeAuthMintSupplyMustBeZeroForConversion = 70u32,
    #[error("You tried to insert one edition too many into an edition mark pda")]
    InvalidEditionIndex = 71u32,
    #[error("In the legacy system the reservation needs to be of size one for cpu limit reasons")]
    ReservationArrayShouldBeSizeOne = 72u32,
    #[error("Is Mutable can only be flipped to false")]
    IsMutableCanOnlyBeFlippedToFalse = 73u32,
    #[error("Cannont Verify Collection in this Instruction")]
    CollectionCannotBeVerifiedInThisInstruction = 74u32,
    #[error("This instruction was deprecated in a previous release and is now removed")]
    Removed = 75u32,
    #[error("This token use method is burn and there are no remaining uses, it must be burned")]
    MustBeBurned = 76u32,
    #[error("This use method is invalid")]
    InvalidUseMethod = 77u32,
    #[error("Cannot Change Use Method after the first use")]
    CannotChangeUseMethodAfterFirstUse = 78u32,
    #[error("Cannot Change Remaining or Available uses after the first use")]
    CannotChangeUsesAfterFirstUse = 79u32,
    #[error("Collection Not Found on Metadata")]
    CollectionNotFound = 80u32,
    #[error("Collection Update Authority is invalid")]
    InvalidCollectionUpdateAuthority = 81u32,
    #[error("Collection Must Be a Unique Master Edition v2")]
    CollectionMustBeAUniqueMasterEdition = 82u32,
    #[error("The Use Authority Record Already Exists, to modify it Revoke, then Approve")]
    UseAuthorityRecordAlreadyExists = 83u32,
    #[error("The Use Authority Record is empty or already revoked")]
    UseAuthorityRecordAlreadyRevoked = 84u32,
    #[error("This token has no uses")]
    Unusable = 85u32,
    #[error("There are not enough Uses left on this token.")]
    NotEnoughUses = 86u32,
    #[error("This Collection Authority Record Already Exists.")]
    CollectionAuthorityRecordAlreadyExists = 87u32,
    #[error("This Collection Authority Record Does Not Exist.")]
    CollectionAuthorityDoesNotExist = 88u32,
    #[error("This Use Authority Record is invalid.")]
    InvalidUseAuthorityRecord = 89u32,
    #[error("This Collection Authority Record is invalid.")]
    InvalidCollectionAuthorityRecord = 90u32,
    #[error("Metadata does not match the freeze authority on the mint")]
    InvalidFreezeAuthority = 91u32,
    #[error("All tokens in this account have not been delegated to this user.")]
    InvalidDelegate = 92u32,
    #[error("Creator can not be adjusted once they are verified.")]
    CannotAdjustVerifiedCreator = 93u32,
    #[error("Verified creators cannot be removed.")]
    CannotRemoveVerifiedCreator = 94u32,
    #[error("Can not wipe verified creators.")]
    CannotWipeVerifiedCreators = 95u32,
    #[error("Not allowed to change seller fee basis points.")]
    NotAllowedToChangeSellerFeeBasisPoints = 96u32,
}
impl From<MplTokenMetadataError> for ProgramError {
    fn from(e: MplTokenMetadataError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MplTokenMetadataError {
    fn type_of() -> &'static str {
        "MplTokenMetadataError"
    }
}
impl PrintProgramError for MplTokenMetadataError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
