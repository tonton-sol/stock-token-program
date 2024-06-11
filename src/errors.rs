//! Error types

use spl_program_error::*;

/// Errors that may be returned by the Token program.
#[spl_program_error(hash_error_code_start = 818340127)]
pub enum StockMarketError {
    /// Error indicating operation is not allowed outside business hours.
    #[error("Custom error: Operation not allowed outside business hours")]
    OutsideBusinessHours,
    /// Error indicating the conversion to UTC failed.
    #[error("Custom error: Failed to convert timestamp")]
    ConvertUTCError,
}
