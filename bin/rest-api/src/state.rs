use crate::configuration::Configuration;
use domain::business::domain::EntryDomain;
use domain::business::traits::EntryDomainTrait;
use std::boxed::Box;

pub struct AppState {
    entry_domain: Box<dyn EntryDomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            entry_domain: Box::new(EntryDomain::new()),
        }
    }

    pub fn get_domain(&self) -> &Box<dyn EntryDomainTrait + Send> {
        &self.entry_domain
    }
}
