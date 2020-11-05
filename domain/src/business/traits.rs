use crate::business::error::EntryDomainResult;
use crate::core::entry::*;

pub trait EntryDomainTrait: Sync + Send {
    fn get_all(&self) -> EntryDomainResult<Vec<Entry>>;
    fn get_regions(&self) -> EntryDomainResult<Vec<String>>;
    fn get_departments(&self) -> EntryDomainResult<Vec<String>>;
    fn search_cities(&self, department: String, query: String) -> EntryDomainResult<Vec<String>>;
    fn get_national_index(&self) -> EntryDomainResult<Entry>;
}
