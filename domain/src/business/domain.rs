use crate::business::error::*;
use crate::business::traits::EntryDomainTrait;
use crate::core::entry::Entry;
use crate::storage::traits::{EntryStorageTrait, IndexStorageTrait};
use std::boxed::Box;

pub struct EntryDomain {
    pub idx_regions: Box<dyn IndexStorageTrait>,
    pub idx_departments: Box<dyn IndexStorageTrait>,
    pub entry_datastore: Box<dyn EntryStorageTrait>,
}

impl EntryDomain {
    pub fn new(
        idx_regions: Box<dyn IndexStorageTrait>,
        idx_departments: Box<dyn IndexStorageTrait>,
        entry_datastore: Box<dyn EntryStorageTrait>,
    ) -> Self {
        EntryDomain {
            idx_regions,
            idx_departments,
            entry_datastore,
        }
    }
}

impl EntryDomainTrait for EntryDomain {
    fn get_all(&self) -> EntryDomainResult<Vec<Entry>> {
        Ok(Vec::new())
    }

    fn get_regions(&self) -> EntryDomainResult<Vec<String>> {
        Ok(self.idx_regions.get_all_keys().unwrap())
    }

    fn get_departments(&self) -> EntryDomainResult<Vec<String>> {
        Ok(self.idx_departments.get_all_keys().unwrap())
    }

    fn search_cities(&self, department: String, query: String) -> EntryDomainResult<Vec<String>> {
        //1. Get cities by departemt

        //2. Search on index by zipcode or name
        Err(EntryDomainError::NotImplemented)
    }

    fn get_national_index(&self) -> EntryDomainResult<Entry> {
        match self.entry_datastore.get_national_entry().unwrap() {
            Some(national_entry) => Ok(national_entry),
            None => Err(EntryDomainError::NotFoundError),
        }
    }

    fn get_regional_index(&self, region: String) -> EntryDomainResult<Entry> {
        match self.entry_datastore.get_region_entry(region).unwrap() {
            Some(regional_entry) => Ok(regional_entry),
            None => Err(EntryDomainError::NotFoundError)
        }
    }
}
