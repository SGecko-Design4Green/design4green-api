use crate::business::error::*;
use crate::business::traits::EntryDomainTrait;
use crate::core::entry::Entry;

pub struct EntryDomain {}

impl EntryDomain {
    pub fn new() -> Self {
        EntryDomain {}
    }
}

impl EntryDomainTrait for EntryDomain {
    fn get_all(&self) -> EntryDomainResult<Vec<Entry>> {
        Ok(Vec::new())
    }
}
