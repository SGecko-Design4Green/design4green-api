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
    fn search_on_key(&self, query: String) -> StorageResult<Vec<String>> {
        let mut results = Vec::new();
        for (key, _) in self.index.iter() {
            if key.contains(&query) {
                results.push(key.to_string());
            }
        }
        Ok(results)
    }

    fn get_index(&self, value: String) -> StorageResult<Option<Vec<String>>> {
        match self.index.get(&value) {
            Some(results) => Ok(Some(results.to_vec())),
            None => Ok(None),
        }
    }

    fn get_all_values(&self) -> StorageResult<Vec<String>> {
        let size = self.index.iter().fold(0, |acc, index| acc + index.1.len());

        let mut result: Vec<String> = Vec::with_capacity(size);
        for index in self.index.iter() {
            result.extend_from_slice(&index.1);
        }
        Ok(result)
    }

    fn get_all_keys(&self) -> StorageResult<Vec<String>> {
        let keys: Vec<String> = self.index.keys().cloned().collect();
        Ok(keys)
    }
}

impl MemoryIndexStorage {
    pub fn load_index(&self) -> StorageResult<()> {
        Ok(())
    }
}
