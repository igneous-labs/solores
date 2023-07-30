use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MplTokenMetadataError {
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError = 0,
    #[error("Failed to pack instruction data")]
    InstructionPackError = 1,
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt = 2,
    #[error("Already initialized")]
    AlreadyInitialized = 3,
    #[error("Uninitialized")]
    Uninitialized = 4,
    #[error(" Metadata's key must match seed of ['metadata', program id, mint] provided")]
    InvalidMetadataKey = 5,
    #[error("Edition's key must match seed of ['metadata', program id, name, 'edition'] provided")]
    InvalidEditionKey = 6,
    #[error("Update Authority given does not match")]
    UpdateAuthorityIncorrect = 7,
    #[error("Update Authority needs to be signer to update metadata")]
    UpdateAuthorityIsNotSigner = 8,
    #[error("You must be the mint authority and signer on this transaction")]
    NotMintAuthority = 9,
    #[error("Mint authority provided does not match the authority on the mint")]
    InvalidMintAuthority = 10,
    #[error("Name too long")]
    NameTooLong = 11,
    #[error("Symbol too long")]
    SymbolTooLong = 12,
    #[error("URI too long")]
    UriTooLong = 13,
    #[error(
        "Update authority must be equivalent to the metadata's authority and also signer of this transaction"
    )]
    UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner = 14,
    #[error("Mint given does not match mint on Metadata")]
    MintMismatch = 15,
    #[error("Editions must have exactly one token")]
    EditionsMustHaveExactlyOneToken = 16,
    #[error("Maximum editions printed already")]
    MaxEditionsMintedAlready = 17,
    #[error("Token mint to failed")]
    TokenMintToFailed = 18,
    #[error("The master edition record passed must match the master record on the edition given")]
    MasterRecordMismatch = 19,
    #[error("The destination account does not have the right mint")]
    DestinationMintMismatch = 20,
    #[error("An edition can only mint one of its kind!")]
    EditionAlreadyMinted = 21,
    #[error("Printing mint decimals should be zero")]
    PrintingMintDecimalsShouldBeZero = 22,
    #[error("OneTimePrintingAuthorization mint decimals should be zero")]
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero = 23,
    #[error("EditionMintDecimalsShouldBeZero")]
    EditionMintDecimalsShouldBeZero = 24,
    #[error("Token burn failed")]
    TokenBurnFailed = 25,
    #[error("The One Time authorization mint does not match that on the token account!")]
    TokenAccountOneTimeAuthMintMismatch = 26,
    #[error("Derived key invalid")]
    DerivedKeyInvalid = 27,
    #[error("The Printing mint does not match that on the master edition!")]
    PrintingMintMismatch = 28,
    #[error("The One Time Printing Auth mint does not match that on the master edition!")]
    OneTimePrintingAuthMintMismatch = 29,
    #[error("The mint of the token account does not match the Printing mint!")]
    TokenAccountMintMismatch = 30,
    #[error("The mint of the token account does not match the master metadata mint!")]
    TokenAccountMintMismatchV2 = 31,
    #[error("Not enough tokens to mint a limited edition")]
    NotEnoughTokens = 32,
    #[error(
        "The mint on your authorization token holding account does not match your Printing mint!"
    )]
    PrintingMintAuthorizationAccountMismatch = 33,
    #[error(
        "The authorization token account has a different owner than the update authority for the master edition!"
    )]
    AuthorizationTokenAccountOwnerMismatch = 34,
    #[error("This feature is currently disabled.")]
    Disabled = 35,
    #[error("Creators list too long")]
    CreatorsTooLong = 36,
    #[error("Creators must be at least one if set")]
    CreatorsMustBeAtleastOne = 37,
    #[error("If using a creators array, you must be one of the creators listed")]
    MustBeOneOfCreators = 38,
    #[error("This metadata does not have creators")]
    NoCreatorsPresentOnMetadata = 39,
    #[error("This creator address was not found")]
    CreatorNotFound = 40,
    #[error("Basis points cannot be more than 10000")]
    InvalidBasisPoints = 41,
    #[error("Primary sale can only be flipped to true and is immutable")]
    PrimarySaleCanOnlyBeFlippedToTrue = 42,
    #[error("Owner does not match that on the account given")]
    OwnerMismatch = 43,
    #[error("This account has no tokens to be used for authorization")]
    NoBalanceInAccountForAuthorization = 44,
    #[error("Share total must equal 100 for creator array")]
    ShareTotalMustBe100 = 45,
    #[error("This reservation list already exists!")]
    ReservationExists = 46,
    #[error("This reservation list does not exist!")]
    ReservationDoesNotExist = 47,
    #[error("This reservation list exists but was never set with reservations")]
    ReservationNotSet = 48,
    #[error("This reservation list has already been set!")]
    ReservationAlreadyMade = 49,
    #[error("Provided more addresses than max allowed in single reservation")]
    BeyondMaxAddressSize = 50,
    #[error("NumericalOverflowError")]
    NumericalOverflowError = 51,
    #[error("This reservation would go beyond the maximum supply of the master edition!")]
    ReservationBreachesMaximumSupply = 52,
    #[error("Address not in reservation!")]
    AddressNotInReservation = 53,
    #[error("You cannot unilaterally verify another creator, they must sign")]
    CannotVerifyAnotherCreator = 54,
    #[error("You cannot unilaterally unverify another creator")]
    CannotUnverifyAnotherCreator = 55,
    #[error("In initial reservation setting, spots remaining should equal total spots")]
    SpotMismatch = 56,
    #[error("Incorrect account owner")]
    IncorrectOwner = 57,
    #[error("printing these tokens would breach the maximum supply limit of the master edition")]
    PrintingWouldBreachMaximumSupply = 58,
    #[error("Data is immutable")]
    DataIsImmutable = 59,
    #[error("No duplicate creator addresses")]
    DuplicateCreatorAddress = 60,
    #[error("Reservation spots remaining should match total spots when first being created")]
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart = 61,
    #[error("Invalid token program")]
    InvalidTokenProgram = 62,
    #[error("Data type mismatch")]
    DataTypeMismatch = 63,
    #[error("Beyond alotted address size in reservation!")]
    BeyondAlottedAddressSize = 64,
    #[error("The reservation has only been partially alotted")]
    ReservationNotComplete = 65,
    #[error("You cannot splice over an existing reservation!")]
    TriedToReplaceAnExistingReservation = 66,
    #[error("Invalid operation")]
    InvalidOperation = 67,
    #[error("Invalid Owner")]
    InvalidOwner = 68,
    #[error("Printing mint supply must be zero for conversion")]
    PrintingMintSupplyMustBeZeroForConversion = 69,
    #[error("One Time Auth mint supply must be zero for conversion")]
    OneTimeAuthMintSupplyMustBeZeroForConversion = 70,
    #[error("You tried to insert one edition too many into an edition mark pda")]
    InvalidEditionIndex = 71,
    #[error("In the legacy system the reservation needs to be of size one for cpu limit reasons")]
    ReservationArrayShouldBeSizeOne = 72,
    #[error("Is Mutable can only be flipped to false")]
    IsMutableCanOnlyBeFlippedToFalse = 73,
    #[error("Cannont Verify Collection in this Instruction")]
    CollectionCannotBeVerifiedInThisInstruction = 74,
    #[error("This instruction was deprecated in a previous release and is now removed")]
    Removed = 75,
    #[error("This token use method is burn and there are no remaining uses, it must be burned")]
    MustBeBurned = 76,
    #[error("This use method is invalid")]
    InvalidUseMethod = 77,
    #[error("Cannot Change Use Method after the first use")]
    CannotChangeUseMethodAfterFirstUse = 78,
    #[error("Cannot Change Remaining or Available uses after the first use")]
    CannotChangeUsesAfterFirstUse = 79,
    #[error("Collection Not Found on Metadata")]
    CollectionNotFound = 80,
    #[error("Collection Update Authority is invalid")]
    InvalidCollectionUpdateAuthority = 81,
    #[error("Collection Must Be a Unique Master Edition v2")]
    CollectionMustBeAUniqueMasterEdition = 82,
    #[error("The Use Authority Record Already Exists, to modify it Revoke, then Approve")]
    UseAuthorityRecordAlreadyExists = 83,
    #[error("The Use Authority Record is empty or already revoked")]
    UseAuthorityRecordAlreadyRevoked = 84,
    #[error("This token has no uses")]
    Unusable = 85,
    #[error("There are not enough Uses left on this token.")]
    NotEnoughUses = 86,
    #[error("This Collection Authority Record Already Exists.")]
    CollectionAuthorityRecordAlreadyExists = 87,
    #[error("This Collection Authority Record Does Not Exist.")]
    CollectionAuthorityDoesNotExist = 88,
    #[error("This Use Authority Record is invalid.")]
    InvalidUseAuthorityRecord = 89,
    #[error("This Collection Authority Record is invalid.")]
    InvalidCollectionAuthorityRecord = 90,
    #[error("Metadata does not match the freeze authority on the mint")]
    InvalidFreezeAuthority = 91,
    #[error("All tokens in this account have not been delegated to this user.")]
    InvalidDelegate = 92,
    #[error("Creator can not be adjusted once they are verified.")]
    CannotAdjustVerifiedCreator = 93,
    #[error("Verified creators cannot be removed.")]
    CannotRemoveVerifiedCreator = 94,
    #[error("Can not wipe verified creators.")]
    CannotWipeVerifiedCreators = 95,
    #[error("Not allowed to change seller fee basis points.")]
    NotAllowedToChangeSellerFeeBasisPoints = 96,
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
