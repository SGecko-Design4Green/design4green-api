use domain::core::entry::Entry;
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
        Err(StorageError::NotImplemented)
    }
    fn get_entry(&self, iris_code: String) -> StorageResult<Option<Entry>> {
        let tree = self.get_entries_tree();
        match tree.get(iris_code.to_string()) {
            Ok(wrap_cbor_entry) => Ok(Some(from_slice(&wrap_cbor_entry.unwrap()).unwrap())),
            Err(_) => Err(StorageError::NotImplemented),
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
