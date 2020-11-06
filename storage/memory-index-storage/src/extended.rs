use domain::core::entry::Iris;
use domain::storage::error::*;
use domain::storage::traits::IndexStoragePostalTrait;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::Bound::Included;

pub struct MemoryIndexStoragePostal {
    pub index: BTreeMap<String, Iris>,
}

impl MemoryIndexStoragePostal {
    pub fn new(path: String) -> StorageResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let index: BTreeMap<String, Iris> = serde_json::from_reader(reader)?;

        Ok(MemoryIndexStoragePostal { index: index })
    }
}

impl IndexStoragePostalTrait for MemoryIndexStoragePostal {
    fn search_on_key(
        &self,
        query: String,
        start_with: Option<String>,
    ) -> StorageResult<Vec<String>> {
        let mut results = Vec::new();

        let index = match start_with {
            Some(value) => {
                let start: &String = &value;
                let end: &String = &format!("{}{}", value.to_string(), "z");

                let bound = (Included(start), Included(end));
                self.index.range::<String, _>(bound)
            }
            None => self.index.range("0".to_string()..),
        };

        let query = query.to_uppercase();
        for (key, _) in index {
            if key.contains(&query) {
                results.push(key.to_string());
            }
        }
        Ok(results)
    }

    fn get_index(&self, value: String) -> StorageResult<Option<Iris>> {
        match self.index.get(&value) {
            Some(results) => Ok(Some(results.clone())),
            None => Ok(None),
        }
    }

    fn get_all_values(&self) -> StorageResult<Vec<Iris>> {
        let mut result: Vec<Iris> = Vec::new();
        for index in self.index.iter() {
            result.push(index.1.clone())
        }
        Ok(result)
    }

    fn get_all_keys(&self) -> StorageResult<Vec<String>> {
        let keys: Vec<String> = self.index.keys().cloned().collect();
        Ok(keys)
    }
}

impl MemoryIndexStoragePostal {}
