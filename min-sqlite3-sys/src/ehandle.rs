use std::{ffi::NulError, str::Utf8Error};

#[derive(Debug)]
pub struct MinSqliteWrapperError<'a> {
    pub kind: &'a str,
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
