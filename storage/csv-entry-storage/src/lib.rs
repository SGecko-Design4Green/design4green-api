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
use std::fs;

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
}

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

impl CSVEntryStorage {
    pub fn new(path: String) -> Self {
        CSVEntryStorage { path: path }
    }

    pub fn get(&self) -> CSVEntryStorageResult<Vec<Entry>> {
        println!("LOAD LINES");

        //println!("{:?}", input_line);
        let file = fs::read(&self.path).unwrap();

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(&*file);

        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: EntryCSV = result?;
            //println!("{:#?}", record);
        }
        println!("DONE");
        Err(CSVEntryStorageError::AnotherError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let storage = CSVEntryStorage::new(
            "D:\\DEV\\GIT_PROJECTS\\design4green-api\\resources\\dataset_ARRAS.csv".to_string(),
        );

        storage.get();
    }
}
