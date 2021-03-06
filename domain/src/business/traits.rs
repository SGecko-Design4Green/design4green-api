use crate::business::error::EntryDomainResult;
use crate::core::entry::*;
use std::collections::{HashMap, BTreeMap};

pub trait EntryDomainTrait: Sync + Send {
    fn get_all(&self) -> EntryDomainResult<Vec<Entry>>;
    fn get_regions(&self) -> EntryDomainResult<Vec<String>>;
    fn get_departments(&self) -> EntryDomainResult<Vec<String>>;
    fn get_cities(&self) -> EntryDomainResult<Vec<String>>;
    fn search_cities(
        &self,
        department: String,
        query: String,
    ) -> EntryDomainResult<BTreeMap<String, CityDetail>>;
    fn get_national_index(&self) -> EntryDomainResult<Entry>;
    fn get_regional_index(&self, region: String) -> EntryDomainResult<Entry>;
    fn get_in_regional_index(&self, region: String) -> EntryDomainResult<HashMap<String, Entry>>;
    fn get_departmental_index(&self, department: String) -> EntryDomainResult<Entry>;
    fn get_in_departmental_index(&self, department: String, page: i32) -> EntryDomainResult<BTreeMap<String, Entry>>;
    fn get_city_index(&self, code_insee: String) -> EntryDomainResult<Entry>;
    fn get_city_districts_index(&self, code_insee: String) -> EntryDomainResult<HashMap<String, Entry>>;
    fn get_all_regions_index(&self) -> EntryDomainResult<HashMap<String, Entry>>;
    fn get_district_index(&self, iriscode: String) -> EntryDomainResult<Entry>;
}
