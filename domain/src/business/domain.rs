use crate::business::error::*;
use crate::business::traits::EntryDomainTrait;
use crate::core::entry::*;
use crate::storage::traits::{EntryStorageTrait, IndexStoragePostalTrait, IndexStorageTrait};
use std::boxed::Box;
use std::collections::HashMap;

pub struct EntryDomain {
    pub idx_regions: Box<dyn IndexStorageTrait>,
    pub idx_departments: Box<dyn IndexStorageTrait>,
    pub idx_cities: Box<dyn IndexStoragePostalTrait>,
    pub idx_departments_by_region: Box<dyn IndexStorageTrait>,
    pub idx_insee_coms: Box<dyn IndexStorageTrait>,
    pub entry_datastore: Box<dyn EntryStorageTrait>,
}

impl EntryDomain {
    pub fn new(
        idx_regions: Box<dyn IndexStorageTrait>,
        idx_departments: Box<dyn IndexStorageTrait>,
        idx_cities: Box<dyn IndexStoragePostalTrait>,
        idx_insee_coms: Box<dyn IndexStorageTrait>,
        idx_departments_by_region: Box<dyn IndexStorageTrait>,
        entry_datastore: Box<dyn EntryStorageTrait>,
    ) -> Self {
        EntryDomain {
            idx_regions,
            idx_departments,
            idx_cities,
            idx_insee_coms,
            idx_departments_by_region,
            entry_datastore,
        }
    }
}

impl EntryDomainTrait for EntryDomain {
    fn get_all(&self) -> EntryDomainResult<Vec<Entry>> {
        Ok(Vec::new())
    }

    fn get_regions(&self) -> EntryDomainResult<Vec<String>> {
        Ok(self.idx_regions.get_all_keys().unwrap())
    }

    fn get_departments(&self) -> EntryDomainResult<Vec<String>> {
        Ok(self.idx_departments.get_all_keys().unwrap())
    }

    fn get_cities(&self) -> EntryDomainResult<Vec<String>> {
        Ok(self.idx_cities.get_all_keys().unwrap())
    }

    fn search_cities(
        &self,
        department: String,
        query: String,
    ) -> EntryDomainResult<HashMap<String, CityDetail>> {
        let mut results: HashMap<String, CityDetail> = HashMap::new();
        let cities = self
            .idx_cities
            .search_on_key(query, Some(department))
            .unwrap();

        for city in cities.iter() {
            results.insert(
                city.to_string(),
                CityDetail {
                    code_insee: None,
                    districts: None,
                },
            );
        }
        Ok(results)
    }

    fn get_national_index(&self) -> EntryDomainResult<Entry> {
        match self.entry_datastore.get_national_entry().unwrap() {
            Some(national_entry) => Ok(national_entry),
            None => Err(EntryDomainError::NotFoundError),
        }
    }

    fn get_regional_index(&self, region: String) -> EntryDomainResult<Entry> {
        let iris_code = match self.idx_regions.get_index(region).unwrap() {
            Some(codes) => Ok(codes.first().unwrap().clone()),
            None => Err(EntryDomainError::NotFoundError),
        };

        match self
            .entry_datastore
            .get_region_entry(iris_code?.to_string())
            .unwrap()
        {
            Some(regional_entry) => Ok(regional_entry),
            None => Err(EntryDomainError::NotFoundError),
        }
    }

    fn get_in_regional_index(&self, region: String) -> EntryDomainResult<HashMap<String, Entry>> {
        let depts = match self.idx_departments_by_region.get_index(region) {
            Ok(option_dept) => match option_dept {
                Some(depts) => depts,
                None => Vec::new(),
            },
            Err(_) => Vec::new(),
        };
        let mut res: HashMap<String, Entry> = HashMap::new();
        for dept in depts {
            res.insert(
                dept.to_string(),
                self.get_departmental_index(dept.to_string()).unwrap(),
            );
        }

        Ok(res)
    }

    fn get_all_regions_index(&self) -> EntryDomainResult<HashMap<String, Entry>> {
        let regions = match self.idx_departments_by_region.get_all_keys() {
            Ok(regs) => regs,
            Err(_) => Vec::new(),
        };
        let mut res: HashMap<String, Entry> = HashMap::new();
        for region in regions {
            res.insert(
                region.to_string(),
                self.get_regional_index(region.to_string()).unwrap(),
            );
        }

        Ok(res)
    }

    fn get_departmental_index(&self, department: String) -> EntryDomainResult<Entry> {
        let iris_code = match self.idx_departments.get_index(department).unwrap() {
            Some(codes) => Ok(codes.first().unwrap().clone()),
            None => Err(EntryDomainError::NotFoundError),
        };

        match self
            .entry_datastore
            .get_department_entry(iris_code?.to_string())
            .unwrap()
        {
            Some(departmental_entry) => Ok(departmental_entry),
            None => Err(EntryDomainError::NotFoundError),
        }
    }

    fn get_city_index(&self, code_insee: String) -> EntryDomainResult<Entry> {
        println!("get city {:?}", code_insee);
        let iris_codes_res = match self.idx_insee_coms.get_index(code_insee) {
            Ok(optional_code) => match optional_code {
                Some(codes) => Ok(codes.clone()),
                None => Err(EntryDomainError::NotFoundError),
            },
            Err(_) => Err(EntryDomainError::NotFoundError),
        };

        match iris_codes_res {
            Ok(iris_code) => {
                println!("{:?} iris retrieved", iris_code);
                let mut cityEntries: Vec<Entry> = Vec::new();
                for iris_code in iris_code.iter() {
                    match self
                        .entry_datastore
                        .get_entry(iris_code.to_string())
                        .unwrap()
                    {
                        Some(neighbor_entry) => {
                            cityEntries.push(neighbor_entry);
                        }
                        None => {}
                    };
                }

                let num_of_neighbors = cityEntries.len() as f64;
                println!("{:?} neighbors", num_of_neighbors);
                let sum_of_global: f64 =
                    cityEntries.iter().map(|entry| entry.global.unwrap()).sum();
                let sum_of_global_numeric_competencies: f64 = cityEntries
                    .iter()
                    .map(|entry| entry.numeric_competencies.clone().unwrap().global.unwrap())
                    .sum();
                let sum_of_global_administrative_competencies: f64 = cityEntries
                    .iter()
                    .map(|entry| {
                        entry
                            .administrative_competencies
                            .clone()
                            .unwrap()
                            .global
                            .unwrap()
                    })
                    .sum();
                let sum_of_global_numeric_interfaces_access: f64 = cityEntries
                    .iter()
                    .map(|entry| {
                        entry
                            .numeric_interfaces_access
                            .clone()
                            .unwrap()
                            .global
                            .unwrap()
                    })
                    .sum();
                let sum_of_global_information_access: f64 = cityEntries
                    .iter()
                    .map(|entry| entry.information_access.clone().unwrap().global.unwrap())
                    .sum();

                let avg_global = sum_of_global / num_of_neighbors;
                let avg_global_numeric_competencies =
                    sum_of_global_numeric_competencies / num_of_neighbors;
                let avg_global_administrative_competencies =
                    sum_of_global_administrative_competencies / num_of_neighbors;
                let avg_global_numeric_interfaces_access =
                    sum_of_global_numeric_interfaces_access / num_of_neighbors;
                let avg_global_information_access =
                    sum_of_global_information_access / num_of_neighbors;

                let found_entry = cityEntries.get(0).unwrap();

                Ok(Entry::new(
                    Some(avg_global),
                    found_entry.global_region,
                    found_entry.global_dept,
                    found_entry.global_national,
                    None,
                    Some(InformationAccess::new(
                        Some(avg_global_information_access),
                        found_entry
                            .information_access
                            .clone()
                            .unwrap()
                            .global_region,
                        found_entry.information_access.clone().unwrap().global_dept,
                        found_entry
                            .information_access
                            .clone()
                            .unwrap()
                            .global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(NumericInterfacesAccess::new(
                        Some(avg_global_numeric_interfaces_access),
                        found_entry
                            .numeric_interfaces_access
                            .clone()
                            .unwrap()
                            .global_region,
                        found_entry
                            .numeric_interfaces_access
                            .clone()
                            .unwrap()
                            .global_dept,
                        found_entry
                            .numeric_interfaces_access
                            .clone()
                            .unwrap()
                            .global_national,
                        None,
                        None,
                        None,
                        None,
                    )),
                    Some(AdministrativeCompetencies::new(
                        Some(avg_global_administrative_competencies),
                        found_entry
                            .administrative_competencies
                            .clone()
                            .unwrap()
                            .global_region,
                        found_entry
                            .administrative_competencies
                            .clone()
                            .unwrap()
                            .global_dept,
                        found_entry
                            .administrative_competencies
                            .clone()
                            .unwrap()
                            .global_national,
                        None,
                        None,
                    )),
                    Some(NumericCompetencies::new(
                        Some(avg_global_numeric_competencies),
                        found_entry
                            .numeric_competencies
                            .clone()
                            .unwrap()
                            .global_region,
                        found_entry
                            .numeric_competencies
                            .clone()
                            .unwrap()
                            .global_dept,
                        found_entry
                            .numeric_competencies
                            .clone()
                            .unwrap()
                            .global_national,
                        None,
                        None,
                    )),
                ))
            }
            Err(_) => Err(EntryDomainError::NotFoundError),
        }
    }

    fn get_district_index(&self, iriscode: String) -> EntryDomainResult<Entry> {
        match self.entry_datastore.get_entry(iriscode)? {
            Some(district_entry) => Ok(district_entry),
            None => Err(EntryDomainError::NotFoundError),
        }
    }
}
