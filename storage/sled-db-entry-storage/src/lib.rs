use domain::core::entry::{Entry, InformationAccess, NumericInterfacesAccess, AdministrativeCompetencies, NumericCompetencies};
use domain::storage::error::*;
use domain::storage::traits::EntryStorageTrait;
use serde_cbor::de::from_slice;
use serde_cbor::ser::to_vec;
use sled::Db;
use sled::Tree;

const ENTRIES_TREE_NAME: &str = "entries";

pub struct SledEntriesStorage {
    storage: Db,
}

impl SledEntriesStorage {
    pub fn new(path: String) -> Self {
        SledEntriesStorage {
            storage: sled::open(path).expect("cannot open the database."),
        }
    }

    fn get_entries_tree(&self) -> Tree {
        self.storage
            .open_tree(ENTRIES_TREE_NAME)
            .expect("cannot open tree")
    }
}

impl EntryStorageTrait for SledEntriesStorage {
    fn get_all(&self) -> StorageResult<Vec<Entry>> {
        let tree = self.get_entries_tree();
        let entries: Vec<Entry> = tree
            .iter()
            .map(|kv| {
                let cbor_entry = kv.unwrap().1;
                from_slice(&cbor_entry).unwrap()
            })
            .collect();
        Ok(entries)
    }
    fn get_entry(&self, iris_code: String) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();
        match tree.get(iris_code.to_string()) {
            Ok(wrap_cbor_entry) => Ok(Some(from_slice(&wrap_cbor_entry.unwrap()).unwrap())),
            Err(_) => Err(StorageError::NotImplemented),
        }
    }
    fn get_national_entry(&self) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();

        match tree.first() {
            Ok(wrap_cbor_entry) => {
                let entry: Entry = from_slice(&wrap_cbor_entry.unwrap().1).unwrap();

                let national_entry = Entry::new(
                    entry.global_national,
                    None,
                    None,
                    None,
                    None,
                    Some(InformationAccess::new(
                        entry.information_access.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(NumericInterfacesAccess::new(
                        entry.numeric_interfaces_access.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(AdministrativeCompetencies::new(
                        entry.administrative_competencies.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(NumericCompetencies::new(
                        entry.numeric_competencies.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None,
                    )),
                );

                Ok(Some(national_entry))
            }
            Err(_) => Err(StorageError::AnotherError)
        }
    }

    fn get_region_entry(&self, code_iris: String) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();

        let mut first_region_entry: Option<Entry> = None;
        for entry in tree.iter() {
            let entry_by_code = entry.unwrap();
            let decoded_entry: Entry = from_slice(&entry_by_code.1).unwrap();
            let stored_iris = decoded_entry.iris_code.clone().unwrap();
            if &stored_iris == &code_iris {
                first_region_entry = Some(decoded_entry.clone());
                break;
            }
        }

        match first_region_entry {
            Some(found_entry) => {
                Ok(Some(Entry::new(
                    found_entry.global_region,
                    None,
                    None,
                    found_entry.global_national,
                    None,
                    Some(InformationAccess::new(
                        found_entry.information_access.clone().unwrap().global_region,
                        None,
                        None,
                        found_entry.information_access.clone().unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(NumericInterfacesAccess::new(
                        found_entry.numeric_interfaces_access.clone().unwrap().global_region,
                        None,
                        None,
                        found_entry.numeric_interfaces_access.clone().unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(AdministrativeCompetencies::new(
                        found_entry.administrative_competencies.clone().unwrap().global_region,
                        None,
                        None,
                        found_entry.administrative_competencies.clone().unwrap().global_national,
                        None,
                        None,
                    )),
                    Some(NumericCompetencies::new(
                        found_entry.numeric_competencies.clone().unwrap().global_region,
                        None,
                        None,
                        found_entry.numeric_competencies.clone().unwrap().global_national,
                        None,
                        None,
                    )),
                )))
            }
            None => Err(StorageError::AnotherError)
        }


    }

    fn get_department_entry(&self, iris_code: String) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();

        let mut first_dept_entry: Option<Entry> = None;
        for entry in tree.iter() {
            let entry_by_code = entry.unwrap();
            let decoded_entry: Entry = from_slice(&entry_by_code.1).unwrap();
            let stored_iris = decoded_entry.iris_code.clone().unwrap();
            if &stored_iris == &iris_code {
                first_dept_entry = Some(decoded_entry.clone());
                break;
            }
        }

        match first_dept_entry {
            Some(found_entry) => {
                Ok(Some(Entry::new(
                    found_entry.global_dept,
                    found_entry.global_region,
                    None,
                    found_entry.global_national,
                    None,
                    Some(InformationAccess::new(
                        found_entry.information_access.clone().unwrap().global_dept,
                        found_entry.information_access.clone().unwrap().global_region,
                        None,
                        found_entry.information_access.clone().unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(NumericInterfacesAccess::new(
                        found_entry.numeric_interfaces_access.clone().unwrap().global_dept,
                        found_entry.numeric_interfaces_access.clone().unwrap().global_region,
                        None,
                        found_entry.numeric_interfaces_access.clone().unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(AdministrativeCompetencies::new(
                        found_entry.administrative_competencies.clone().unwrap().global_dept,
                        found_entry.administrative_competencies.clone().unwrap().global_region,
                        None,
                        found_entry.administrative_competencies.clone().unwrap().global_national,
                        None,
                        None,
                    )),
                    Some(NumericCompetencies::new(
                        found_entry.numeric_competencies.clone().unwrap().global_dept,
                        found_entry.numeric_competencies.clone().unwrap().global_region,
                        None,
                        found_entry.numeric_competencies.clone().unwrap().global_national,
                        None,
                        None,
                    )),
                )))
            }
            None => Err(StorageError::AnotherError)
        }

    }

    fn get_district_entry(&self, iris_code: String) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();

        let mut first_dept_entry: Option<Entry> = None;
        for entry in tree.iter() {
            let entry_by_code = entry.unwrap();
            let decoded_entry: Entry = from_slice(&entry_by_code.1).unwrap();
            let stored_iris = decoded_entry.iris_code.clone().unwrap();
            if &stored_iris == &iris_code {
                first_dept_entry = Some(decoded_entry.clone());
                break;
            }
        }

        match first_dept_entry {
            Some(found_entry) => {
                Ok(Some(Entry::new(
                    found_entry.global,
                    found_entry.global_region,
                    found_entry.global_dept,
                    found_entry.global_national,
                    None,
                    Some(InformationAccess::new(
                        found_entry.information_access.clone().unwrap().global,
                        found_entry.information_access.clone().unwrap().global_region,
                        found_entry.information_access.clone().unwrap().global_dept,
                        found_entry.information_access.clone().unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(NumericInterfacesAccess::new(
                        found_entry.numeric_interfaces_access.clone().unwrap().global,
                        found_entry.numeric_interfaces_access.clone().unwrap().global_region,
                        found_entry.numeric_interfaces_access.clone().unwrap().global_dept,
                        found_entry.numeric_interfaces_access.clone().unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(AdministrativeCompetencies::new(
                        found_entry.administrative_competencies.clone().unwrap().global,
                        found_entry.administrative_competencies.clone().unwrap().global_region,
                        found_entry.administrative_competencies.clone().unwrap().global_dept,
                        found_entry.administrative_competencies.clone().unwrap().global_national,
                        None,
                        None,
                    )),
                    Some(NumericCompetencies::new(
                        found_entry.numeric_competencies.clone().unwrap().global,
                        found_entry.numeric_competencies.clone().unwrap().global_region,
                        found_entry.numeric_competencies.clone().unwrap().global_dept,
                        found_entry.numeric_competencies.clone().unwrap().global_national,
                        None,
                        None,
                    )),
                )))
            }
            None => Err(StorageError::AnotherError)
        }
    }

    fn create(&self, iris_code: String, entry: Entry) -> StorageResult<()> {
        let tree = self.get_entries_tree();
        match tree.insert(iris_code, to_vec(&entry).unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(StorageError::NotImplemented),
        }
    }
}
