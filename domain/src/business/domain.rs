use crate::business::error::*;
use crate::business::traits::EntryDomainTrait;
use crate::core::entry::Entry;
use crate::storage::traits::IndexStorageTrait;
use std::boxed::Box;

pub struct EntryDomain {
    pub idx_regions: Box<dyn IndexStorageTrait>,
    pub idx_departments: Box<dyn IndexStorageTrait>,
}

impl EntryDomain {
    pub fn new(
        idx_regions: Box<dyn IndexStorageTrait>,
        idx_departments: Box<dyn IndexStorageTrait>,
    ) -> Self {
        EntryDomain {
            idx_regions,
            idx_departments,
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
}
