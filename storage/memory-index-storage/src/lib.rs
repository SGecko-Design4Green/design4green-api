use dashmap::DashMap;
use domain::storage::error::*;
use domain::storage::traits::IndexStorageTrait;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub struct MemoryIndexStorage {
    pub index: Arc<DashMap<String, Vec<String>>>,
}

impl MemoryIndexStorage {
    pub fn new(path: String) -> StorageResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: DashMap<String, Vec<String>> = serde_json::from_reader(reader)?;

        Ok(MemoryIndexStorage {
            index: Arc::new(index),
        })
    }
}

impl IndexStorageTrait for MemoryIndexStorage {
    fn get_index(&self, value: String) -> StorageResult<Option<Vec<String>>> {
        match self.index.get_mut(&value) {
            Some(results) => Ok(Some(results.to_vec())),
            None => Ok(None),
        }
    }

    fn get_all(&self) -> StorageResult<Vec<String>> {
        let size = self.index.iter().fold(0, |acc, index| acc + index.len());
        let mut result: Vec<String> = Vec::with_capacity(size);

        for index in self.index.iter() {
            result.extend_from_slice(&index);
        }
        Ok(result)
    }
}

impl MemoryIndexStorage {
    pub fn load_index(&self) -> StorageResult<()> {
        Ok(())
    }
}
