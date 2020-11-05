use crate::core::entry::{Entry, Iris};
use crate::storage::error::*;

pub trait EntryStorageTrait: Sync + Send {
    fn get_all(&self) -> StorageResult<Vec<Entry>>;
    fn get_entry(&self, iris_code: String) -> StorageResult<Option<Entry>>;
    fn get_national_entry(&self) -> StorageResult<Option<Entry>>;
    fn get_region_entry(&self, iris_code: String) -> StorageResult<Option<Entry>>;
    fn get_department_entry(&self, iris_code: String) -> StorageResult<Option<Entry>>;
    fn get_district_entry(&self, iris_code: String) -> StorageResult<Option<Entry>>;
    fn create(&self, iris_code: String, entry: Entry) -> StorageResult<()>;
}

pub trait IndexStorageTrait: Sync + Send {
    fn search_on_key(
        &self,
        contains: String,
        start_with: Option<String>,
    ) -> StorageResult<Vec<String>>;
    fn get_index(&self, value: String) -> StorageResult<Option<Vec<String>>>;
    fn get_all_values(&self) -> StorageResult<Vec<String>>;
    fn get_all_keys(&self) -> StorageResult<Vec<String>>;
}

pub trait IndexStoragePostalTrait: Sync + Send {
    fn search_on_key(
        &self,
        contains: String,
        start_with: Option<String>,
    ) -> StorageResult<Vec<String>>;
    fn get_index(&self, value: String) -> StorageResult<Option<Iris>>;
    fn get_all_values(&self) -> StorageResult<Vec<Iris>>;
    fn get_all_keys(&self) -> StorageResult<Vec<String>>;
}
