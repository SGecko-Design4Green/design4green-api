use crate::core::entry::Entry;
use crate::storage::error::*;

pub trait EntryStorageTrait: Sync + Send {
    fn get_all(&self) -> StorageResult<Vec<Entry>>;
    fn get_entry(&self, iris_code: String) -> StorageResult<Option<Entry>>;
    fn create(&self, entry: Entry) -> StorageResult<Entry>;
    fn update(&self, entry: Entry) -> StorageResult<Entry>;
    fn delete(&self, iris_code: String) -> StorageResult<()>;
}

pub trait IndexStorageTrait: Sync + Send {
    fn get_index(&self, value: String) -> StorageResult<Option<Vec<String>>>;
    fn get_all(&self) -> StorageResult<Vec<String>>;
}
