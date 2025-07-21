use crate::utils::Repository;
use std::{error::Error, fs::File, io};

impl Repository {
    pub fn pull_latest(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn pull_hash(&self, hash: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
