use crate::configuration::Configuration;
use domain::business::domain::EntryDomain;
use domain::business::traits::EntryDomainTrait;
use memory_index_storage::extended::MemoryIndexStoragePostal;
use memory_index_storage::MemoryIndexStorage;
use sled_db_entry_storage::SledEntriesStorage;
use std::boxed::Box;

pub struct AppState {
    entry_domain: Box<dyn EntryDomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            entry_domain: Box::new(EntryDomain::new(
                Box::new(
                    MemoryIndexStorage::new(Configuration::get_index_path() + "idx_regions.json")
                        .unwrap(),
                ),
                Box::new(
                    MemoryIndexStorage::new(
                        Configuration::get_index_path() + "idx_departments.json",
                    )
                    .unwrap(),
                ),
                Box::new(
                    MemoryIndexStoragePostal::new(
                        Configuration::get_index_path() + "idx_postal.json",
                    )
                    .unwrap(),
                ),
                Box::new(
                    MemoryIndexStorage::new(
                        Configuration::get_index_path() + "idx_insee_coms.json",
                    )
                    .unwrap(),
                ),
                Box::new(MemoryIndexStorage::new(Configuration::get_index_path() + "idx_departments_by_region.json").unwrap()),
                Box::new(SledEntriesStorage::new(Configuration::get_sled_db_path())),
            )),
        }
    }

    pub fn get_domain(&self) -> &Box<dyn EntryDomainTrait + Send> {
        &self.entry_domain
    }
}
