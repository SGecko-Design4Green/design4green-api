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

#[derive(Copy, Clone)]
pub struct AvgStat {
    avg_entries_global_score: f32,
    avg_entries_numeric_competencies: f32,
    avg_entries_administrative_competencies: f32,
    avg_entries_numeric_interface_access: f32,
    avg_entries_information_access: f32,
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
        match name.trim().is_empty() {
            false => format!("{} - {}", code, name),
            true => "".to_string(),
        }
    }

    fn get_stats(&self, csv_entries: Vec<EntryCSV>) -> AvgStat {
        let number_of_entries: f32 = csv_entries.len() as f32;
        let sum_entries_global_score: f32 = csv_entries
            .iter()
                .map(|csv_entry|
                    csv_entry.clean_and_parse_f32(&csv_entry.score_global_region_star)
                        .unwrap()).sum();
        let sum_entries_numeric_competencies: f32 = csv_entries
            .iter()
                .map(|csv_entry|
                    csv_entry.clean_and_parse_f32(&csv_entry.competences_numeriques_scolaires_region_1)
                        .unwrap()).sum();
        let sum_entries_administrative_competencies: f32 = csv_entries
            .iter()
                .map(|csv_entry|
                    csv_entry.clean_and_parse_f32(&csv_entry.competences_administatives_region_1)
                        .unwrap()).sum();
        let sum_entries_numeric_interface_access: f32 = csv_entries
            .iter()
                .map(|csv_entry|
                    csv_entry.clean_and_parse_f32(&csv_entry.acces_aux_interfaces_numeriques_region_1)
                        .unwrap()).sum();
        let sum_entries_information_access: f32 = csv_entries
            .iter()
                .map(|csv_entry|
                    csv_entry.clean_and_parse_f32(&csv_entry.acces_information_region_1)
                        .unwrap()).sum();

        AvgStat {
            avg_entries_global_score: sum_entries_global_score / number_of_entries,
            avg_entries_numeric_competencies: sum_entries_numeric_competencies / number_of_entries,
            avg_entries_administrative_competencies: sum_entries_administrative_competencies / number_of_entries,
            avg_entries_numeric_interface_access: sum_entries_numeric_interface_access / number_of_entries,
            avg_entries_information_access: sum_entries_information_access / number_of_entries,
        }
    }

    pub fn get_entries(&self) -> Vec<Entry> {
        let nationalStats = self.get_stats(self.get_csv_entries());
        let mut regionsStats: BTreeMap<String, AvgStat> = BTreeMap::new();
        for region in self.get_regions() {
            regionsStats
                .insert(region.to_string(),
                        self.get_stats(self.get_csv_entries()
                            .iter().filter_map(|entry_csv| match &entry_csv.nom_reg == &region {
                            true => Some(entry_csv.clone()),
                            false => None
                        }).collect()));
        }

        let mut departmentsStats: BTreeMap<String, AvgStat> = BTreeMap::new();
        for department in self.get_departments() {
            departmentsStats
                .insert(department.to_string(),
                self.get_stats(self.get_csv_entries()
                    .iter()
                        .filter_map(|csv_entry|
                            match &department == &self.concat_name(csv_entry.dep.to_owned(), csv_entry.nom_dep.to_owned()) {
                                true => Some(csv_entry.clone()),
                                false => None
                            }).collect()));
        }

        self.get_csv_entries()
            .iter()
            .map(|csv_entry| csv_entry.to_entry(&nationalStats, &regionsStats, &departmentsStats))
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

    pub fn get_insee_coms(&self) -> HashSet<String> {
        let all_insee_com: Vec<String> = self
            .get_csv_entries()
            .iter()
            .map(|csv_entry| csv_entry.insee_com.to_owned())
            .collect();

        std::collections::HashSet::from_iter(all_insee_com)
    }

    pub fn get_insee_com_with_iris(&self) -> BTreeMap<String, Vec<String>> {
        let mut results: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let insee_coms = self.get_insee_coms();

        for insee_com in insee_coms {
            results.insert(
                insee_com.clone(),
                self.get_csv_entries()
                    .iter()
                    .filter_map(|csv_entry| match &csv_entry.insee_com == &insee_com {
                        true => Some(csv_entry.code_iris.clone()),
                        false => None,
                    })
                    .collect(),
            );
        }

        //Remove unasigned items
        results.remove_entry(&"".to_string());
        results
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

        //Remove unasigned items
        results.remove_entry(&"".to_string());
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

        //Remove unasigned items
        results.remove_entry(&"".to_string());
        results
    }

    pub fn get_national_entries() -> Entry {
        unimplemented!()
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

            let key = self.concat_name(
                postal_code.get_code(),
                postal_code.nom_com.to_owned(),
            );

            results.insert(
                key,
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
