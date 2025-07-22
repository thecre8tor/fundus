use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The program has already been initialized")]
    AlreadyInitialized,
    #[msg("Title exceeds the maximum length of 64 characters.")]
    TitleTooLong,
    #[msg("Description exceeds the maximum length of 512 characters.")]
    DescriptionTooLong,
    #[msg("Image URL exceeds the maximum length of 256 characters.")]
    ImageUrlTooLong,
    #[msg("Invalid goal amount, Goal must be greater than zero.")]
    InvalidGoalAmount,
}
