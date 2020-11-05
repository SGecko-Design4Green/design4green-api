use csv_entry_storage::CSVEntryStorage;
use csv_entry_storage::PostalCodeCsvStorage;

use domain::core::entry::*;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Another error")]
    AnotherError,
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
}

//Define a generic error type to simplify return.
pub type ImportResult<T> = std::result::Result<T, ImportError>;

fn main() -> ImportResult<()> {
    let now = Instant::now();
    let mut storage = PostalCodeCsvStorage::new("resources/postal.csv".to_string());
    &storage.load();

    let iris_codes_postal_codes = &storage.get_iris_and_geoloc_with_postal_code();
    serialize_index_to_file("postal".to_string(), iris_codes_postal_codes)?;
    println!("Postal >> Lines {:?}", iris_codes_postal_codes.len());
    println!(
        "Duration : {} seconds and {} nanoseconds",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos()
    );

    let now = Instant::now();
    let mut storage = CSVEntryStorage::new("resources/full.csv".to_string());
    &storage.load();

    //CREATE INDEX FOR REGIONS
    let reg_iris = &storage.get_regions_with_iris();
    println!("REG_IRIS >> Lines {:?}", reg_iris.len());
    serialize_index_to_file("regions".to_string(), reg_iris);

    //CREATE INDEX FOR DEPARTEMENTS
    let dep_iris = &storage.get_departements_with_iris();
    println!("DEP_IRIS >> Lines {:?}", dep_iris.len());
    serialize_index_to_file("departments".to_string(), dep_iris);

    println!(
        "Duration : {} seconds and {} nanoseconds",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos()
    );
    Ok(())
}

fn serialize_index_to_file<T: DeserializeOwned + serde::Serialize>(
    name: String,
    value: &T,
) -> ImportResult<()> {
    let path = format!("resources/indexes/idx_{}.json", name);
    let path = Path::new(&path);

    fs::remove_file(path);
    let file = match path.exists() {
        true => OpenOptions::new().write(true).open(path)?,
        false => File::create(path)?,
    };

    let result = serde_json::to_writer(file, &value);

    Ok(())
}
