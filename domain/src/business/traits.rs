use crate::business::error::EntryDomainResult;
use crate::core::entry::*;

pub trait EntryDomainTrait: Sync + Send {
    fn get_all_entries(&self) -> EntryDomainResult<Vec<Entry>>;
    fn search_entries_by_region(&self) -> EntryDomainResult<Vec<Entry>>;
    fn search_entries_by_department(&self) -> EntryDomainResult<Vec<Entry>>;

    fn get_department(&self) -> EntryDomainResult<Vec<Entry>>;
}
