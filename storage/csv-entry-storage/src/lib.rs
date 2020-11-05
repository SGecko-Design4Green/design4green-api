#[macro_use]
extern crate serde_derive;

use domain::core::entry::Entry;
use domain::storage::error::*;
use entry_csv::EntryCSV;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;
use thiserror::Error;
pub mod entry_csv;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

//Define the possible errors
#[derive(Error, Debug)]
pub enum CSVEntryStorageError {
    #[error("Another error")]
    AnotherError,
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("CSV error: {source}")]
    CSV {
        #[from]
        source: csv::Error,
    },
}

//Define a generic error type to simplify return.
pub type CSVEntryStorageResult<T> = std::result::Result<T, CSVEntryStorageError>;

pub struct CSVEntryStorage {
    pub path: String,
    pub entries: Option<Vec<EntryCSV>>,
}

impl CSVEntryStorage {
    pub fn new(path: String) -> Self {
        CSVEntryStorage {
            path: path,
            entries: None,
        }
    }
    pub fn load(&mut self) -> CSVEntryStorageResult<()> {
        let mut entries = Vec::new();
        let file = fs::read(&self.path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(&*file);

        for result in rdr.deserialize() {
            let record: EntryCSV = result?;
            println!("{:#?}", &record);
            entries.push(record);
        }
        self.entries = Some(entries);
        Ok(())
    }
    fn concat_name(&self, code: String, name: String) -> String {
        format!("{} - {}", code, name)
    }

    pub fn get_entries(&self) -> Vec<Entry> {
        self.get_csv_entries()
            .iter()
            .map(|csv_entry| csv_entry.to_entry())
            .collect()
    }

    pub fn get_departments(&self) -> HashSet<String> {
        let all_dep: Vec<String> = self
            .get_csv_entries()
            .iter()
            .map(|csv_entry| {
                self.concat_name(csv_entry.dep.to_owned(), csv_entry.libdep.to_owned())
            })
            .collect();

        std::collections::HashSet::from_iter(all_dep)
    }

    pub fn get_regions(&self) -> HashSet<String> {
        let all_reg: Vec<String> = self
            .get_csv_entries()
            .iter()
            .map(|csv_entry| csv_entry.libreg.to_owned())
            .collect();

        std::collections::HashSet::from_iter(all_reg)
    }

    pub fn get_regions_with_iris(&self) -> HashMap<String, Vec<String>> {
        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let regions = self.get_regions();

        for region in regions {
            results.insert(
                region.clone(),
                self.get_csv_entries()
                    .iter()
                    .filter_map(|csv_entry| {
                        match &self
                            .concat_name(csv_entry.dep.to_owned(), csv_entry.libdep.to_owned())
                            == &region
                        {
                            true => Some(csv_entry.code_iris.clone()),
                            false => None,
                        }
                    })
                    .collect(),
            );
        }
        results
    }

    pub fn get_departements_with_iris(&self) -> HashMap<String, Vec<String>> {
        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let departements = self.get_departments();

        for departement in departements {
            results.insert(
                departement.clone(),
                self.get_csv_entries()
                    .iter()
                    .filter_map(|csv_entry| match &csv_entry.libdep == &departement {
                        true => Some(csv_entry.code_iris.clone()),
                        false => None,
                    })
                    .collect(),
            );
        }

        results
    }

    pub fn get_coms(&self) -> HashSet<String> {
        let all_com: Vec<String> = self
            .get_csv_entries()
            .iter()
            .map(|csv_entry| csv_entry.libcom.to_owned())
            .collect();

        std::collections::HashSet::from_iter(all_com)
    }

    pub fn get_csv_entries(&self) -> Vec<EntryCSV> {
        match &self.entries {
            Some(entries) => entries.to_vec(),
            None => Vec::new(),
        }
    }
}
