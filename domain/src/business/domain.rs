use crate::business::error::*;
use crate::business::traits::EntryDomainTrait;
use crate::core::entry::{
    AdministrativeCompetencies, Entry, InformationAccess, NumericCompetencies,
    NumericInterfacesAccess,
};
use crate::storage::traits::{EntryStorageTrait, IndexStorageTrait};
use std::boxed::Box;

pub struct EntryDomain {
    pub idx_regions: Box<dyn IndexStorageTrait>,
    pub idx_departments: Box<dyn IndexStorageTrait>,
    pub idx_cities: Box<dyn IndexStorageTrait>,
    pub idx_insee_coms: Box<dyn IndexStorageTrait>,
    pub entry_datastore: Box<dyn EntryStorageTrait>,
}

impl EntryDomain {
    pub fn new(
        idx_regions: Box<dyn IndexStorageTrait>,
        idx_departments: Box<dyn IndexStorageTrait>,
        idx_cities: Box<dyn IndexStorageTrait>,
        idx_insee_coms: Box<dyn IndexStorageTrait>,
        entry_datastore: Box<dyn EntryStorageTrait>,
    ) -> Self {
        EntryDomain {
            idx_regions,
            idx_departments,
            idx_cities,
            entry_datastore,
            idx_insee_coms,
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

    fn search_cities(&self, department: String, query: String) -> EntryDomainResult<Vec<String>> {
        Ok(self
            .idx_departments
            .search_on_key(query, Some(department))
            .unwrap())
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
