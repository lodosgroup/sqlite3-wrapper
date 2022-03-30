use std::{ffi::NulError, str::Utf8Error};

/// Error type that covers all kinds of errors
/// that might occur on some of the wrapped functions.
///
/// # Warning
/// This type isn't for SQL errors. In order to deal with SQL
/// errors, consider checking `SqlPrimaryResult` enum and
/// callback functions.
#[derive(Debug, Clone)]
pub struct MinSqliteWrapperError<'a> {
    /// defines type of the error
    pub kind: &'a str,
    /// provides error message
    pub reason: String,
}

impl<'a> From<NulError> for MinSqliteWrapperError<'a> {
    fn from(error: NulError) -> Self {
        MinSqliteWrapperError {
            kind: "std:ffi:NulError",
            reason: error.to_string(),
        }
    }
}

impl<'a> From<Utf8Error> for MinSqliteWrapperError<'a> {
    fn from(error: Utf8Error) -> Self {
        MinSqliteWrapperError {
            kind: "std:str:Utf8Error",
            reason: error.to_string(),
        }
    }
}
