use serde::{de, ser};
use std::fmt;
use std::io::Error as IoError;

/// This type represents all possible errors that can occur when serializing or
/// deserializing Bin_prot data.
#[derive(Debug)]
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}

/// Alias for a `Result` with the error type `serde_bin_prot::Error`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct ErrorImpl {
    code: ErrorCode,
}

#[derive(Debug)]
pub(crate) enum ErrorCode {
    Message(Box<str>),
}

impl ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}
