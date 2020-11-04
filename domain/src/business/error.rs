use std::result::Result;
use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum EntryDomainError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Storage Error")]
    StorageError,
    #[error("Not found error")]
    NotFoundError,
}

//Define a generic error type to simplify return.
pub type EntryDomainResult<T> = Result<T, EntryDomainError>;
