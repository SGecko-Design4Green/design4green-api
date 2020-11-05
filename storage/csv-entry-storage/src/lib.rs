#[macro_use]
extern crate serde_derive;

pub mod entry_csv;
pub mod postal_code_csv_index;

use domain::core::entry::Entry;
use domain::core::entry::Iris;
use entry_csv::EntryCSV;
use postal_code_csv_index::PostalCodeIrisCodeCSV;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use thiserror::Error;

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
                self.concat_name(csv_entry.dep.to_owned(), csv_entry.nom_dep.to_owned())
            })
            .collect();

        std::collections::HashSet::from_iter(all_dep)
    }

    pub fn get_regions(&self) -> HashSet<String> {
        let all_reg: Vec<String> = self
            .get_csv_entries()
            .iter()
            .map(|csv_entry| csv_entry.nom_reg.to_owned())
            .collect();

        std::collections::HashSet::from_iter(all_reg)
    }

    pub fn get_regions_with_iris(&self) -> BTreeMap<String, Vec<String>> {
        let mut results: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let regions = self.get_regions();

        for region in regions {
            results.insert(
                region.clone(),
                self.get_csv_entries()
                    .iter()
                    .filter_map(|csv_entry| match &csv_entry.nom_reg == &region {
                        true => Some(csv_entry.code_iris.clone()),
                        false => None,
                    })
                    .collect(),
            );
        }
        results
    }

    pub fn get_departements_with_iris(&self) -> BTreeMap<String, Vec<String>> {
        let mut results: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let departements = self.get_departments();

        for departement in departements {
            results.insert(
                departement.clone(),
                self.get_csv_entries()
                    .iter()
                    .filter_map(|csv_entry| {
                        match &self
                            .concat_name(csv_entry.dep.to_owned(), csv_entry.nom_dep.to_owned())
                            == &departement
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

    pub fn get_csv_entries(&self) -> Vec<EntryCSV> {
        match &self.entries {
            Some(entries) => entries.to_vec(),
            None => Vec::new(),
        }
    }
}

#[derive(Error, Debug)]
pub enum PostalCodeCsvStorageError {
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

pub type PostalCodeCsvStorageResult<T> = std::result::Result<T, PostalCodeCsvStorageError>;

pub struct PostalCodeCsvStorage {
    pub path: String,
    pub postal_codes: Option<Vec<PostalCodeIrisCodeCSV>>,
}

impl PostalCodeCsvStorage {
    pub fn new(path: String) -> Self {
        PostalCodeCsvStorage {
            path: path,
            postal_codes: None,
        }
    }

    pub fn load(&mut self) -> CSVEntryStorageResult<()> {
        let mut entries = Vec::new();

        let file = fs::read(&self.path).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(&*file);

        for result in rdr.deserialize() {
            let record: PostalCodeIrisCodeCSV = result?;
            entries.push(record);
        }

        self.postal_codes = Some(entries);
        Ok(())
    }

    fn concat_name(&self, code: String, name: String) -> String {
        match code.trim().is_empty() {
            false => format!("{} - {}", code, name),
            true => name,
        }
    }

    pub fn get_iris_and_geoloc_with_postal_code(&self) -> BTreeMap<String, Iris> {
        let mut results: BTreeMap<String, Iris> = BTreeMap::new();

        for postal_code in &self.get_csv_postal_codes() {
            let postal_code: &PostalCodeIrisCodeCSV = postal_code;
            let data = postal_code.to_postal_code();

            results.insert(
                self.concat_name(
                    postal_code.postal_code.to_owned(),
                    postal_code.nom_com.to_owned(),
                ),
                data,
            );
        }

        results
    }

    pub fn get_csv_postal_codes(&self) -> Vec<PostalCodeIrisCodeCSV> {
        match &self.postal_codes {
            Some(postal_codes) => postal_codes.to_vec(),
            None => Vec::new(),
        }
    }
}
