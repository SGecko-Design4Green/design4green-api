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
                        None
                    )),
                    Some(NumericInterfacesAccess::new(
                        entry.numeric_interfaces_access.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(AdministrativeCompetencies::new(
                        entry.administrative_competencies.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(NumericCompetencies::new(
                        entry.numeric_competencies.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None
                    ))
                );

                Ok(Some(national_entry))
            },
            Err(_) => Err(StorageError::AnotherError)
        }
    }

    fn get_region_entry(&self, region: String) -> StorageResult<Option<Entry>> {
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
                        None
                    )),
                    Some(NumericInterfacesAccess::new(
                        entry.numeric_interfaces_access.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(AdministrativeCompetencies::new(
                        entry.administrative_competencies.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(NumericCompetencies::new(
                        entry.numeric_competencies.unwrap().global_national,
                        None,
                        None,
                        None,
                        None,
                        None
                    ))
                );

                Ok(Some(national_entry))
            },
            Err(_) => Err(StorageError::AnotherError)
        }
    }

    fn get_department_entry(&self, department: String) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();

        match tree.first() {
            Ok(wrap_cbor_entry) => {
                let entry: Entry = from_slice(&wrap_cbor_entry.unwrap().1).unwrap();

                let national_entry = Entry::new(
                    entry.global_dept,
                    None,
                    None,
                    None,
                    None,
                    Some(InformationAccess::new(
                        entry.information_access.unwrap().global_dept,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(NumericInterfacesAccess::new(
                        entry.numeric_interfaces_access.unwrap().global_dept,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(AdministrativeCompetencies::new(
                        entry.administrative_competencies.unwrap().global_dept,
                        None,
                        None,
                        None,
                        None,
                        None
                    )),
                    Some(NumericCompetencies::new(
                        entry.numeric_competencies.unwrap().global_dept,
                        None,
                        None,
                        None,
                        None,
                        None
                    ))
                );

                Ok(Some(national_entry))
            },
            Err(_) => Err(StorageError::AnotherError)
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
