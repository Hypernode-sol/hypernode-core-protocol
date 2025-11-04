use anchor_lang::prelude::*;

#[error_code]
pub enum JobReceiptError {
    #[msg("Job ID is too long (max 32 characters)")]
    JobIdTooLong,

    #[msg("Job already exists")]
    JobAlreadyExists,

    #[msg("Job not found")]
    JobNotFound,

    #[msg("Invalid job type")]
    InvalidJobType,

    #[msg("Invalid job status")]
    InvalidJobStatus,

    #[msg("Job is not pending")]
    JobNotPending,

    #[msg("Job is not assigned")]
    JobNotAssigned,

    #[msg("Job is not completed")]
    JobNotCompleted,

    #[msg("Job already settled")]
    JobAlreadySettled,

    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("Requirements hash too long")]
    RequirementsHashTooLong,

    #[msg("Result hash invalid")]
    ResultHashInvalid,

    #[msg("Logs URL too long")]
    LogsUrlTooLong,
}
