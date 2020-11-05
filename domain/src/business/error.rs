use std::result::Result;
use thiserror::Error;
use crate::storage::error::StorageError;

//Define the possible errors
#[derive(Error, Debug)]
pub enum EntryDomainError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Storage Error")]
    StorageError,
    #[error("Not found error")]
    NotFoundError,
    #[error("Storage error: {source}")]
    Storage {
        #[from]
        source: StorageError,
    }
}

//Define a generic error type to simplify return.
pub type EntryDomainResult<T> = Result<T, EntryDomainError>;
