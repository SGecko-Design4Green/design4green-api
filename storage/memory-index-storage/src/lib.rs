use dashmap::DashMap;
use domain::storage::error::*;
use domain::storage::traits::IndexStorageTrait;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

pub struct MemoryIndexStorage {
    pub index: BTreeMap<String, Vec<String>>,
}

impl MemoryIndexStorage {
    pub fn new(path: String) -> StorageResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: BTreeMap<String, Vec<String>> = serde_json::from_reader(reader)?;

        Ok(MemoryIndexStorage { index: index })
    }
}

impl IndexStorageTrait for MemoryIndexStorage {
    fn get_index(&self, value: String) -> StorageResult<Option<Vec<String>>> {
        match &self.index.get(&value) {
            Some(results) => Ok(Some(results.to_vec())),
            None => Ok(None),
        }
    }

    fn get_all(&self) -> StorageResult<Vec<String>> {
        //let result: Vec<String> = self.index.into_values().collect();
        Ok(Vec::new())
    }
}

impl MemoryIndexStorage {
    pub fn load_index(&self) -> StorageResult<()> {
        Ok(())
    }
}
