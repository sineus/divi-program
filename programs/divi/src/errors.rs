use anchor_lang::prelude::*;

#[error_code]
pub enum DiviError {
    #[msg("String is too long")]
    StringTooLong,
    #[msg("Title is too long")]
    TitleTooLong,
    #[msg("Description is too long")]
    DescriptionTooLong,
    #[msg("Cover is too long")]
    CoverTooLong,
    #[msg("Director is too long")]
    DirectorTooLong,
    #[msg("Too many actors")]
    TooManyActors,
    #[msg("Invalid PDA for the movie")]
    InvalidPDA,
    #[msg("Unauthorized access, you're not the creator of this movie")]
    UnauthorizedAccess,
    #[msg("An error occurred while calculating percentage")]
    ShareCalculationError,
    #[msg("Participant has already paid")]
    ParticipantAlreadyPaid,
    #[msg("Participant doesn't exists")]
    ParticipantNotExist,
    #[msg("The amount is greater than the vault total amount")]
    AmountIsGreaterThanVaultTotalAmount,
    #[msg("The amount is greater than the remaining vault amount")]
    AmountIsGreaterThanRemainingVaultAmount,
    #[msg("The vault issuer is invalid")]
    InvalidVaultAuthority,
    #[msg("Vault is already finalized")]
    VaultIsAlreadyFinalized,
    #[msg("Vault is not finalized")]
    VaultIsNotFinalized,
    #[msg("An error occurred while getting participant from vault")]
    ParticipantAccountNotProvided,
}
