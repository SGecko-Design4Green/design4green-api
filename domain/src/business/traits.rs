use crate::business::error::EntryDomainResult;
use crate::core::entry::*;

pub trait EntryDomainTrait: Sync + Send {
    fn get_all(&self) -> EntryDomainResult<Vec<Entry>>;
}