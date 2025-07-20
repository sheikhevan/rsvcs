use crate::utils::Repository;
use std::{error::Error, fs};

impl Repository {
    pub fn pull_latest(&self) -> Result<(), Box<dyn Error>> {
        println!("pp");
        Ok(()) // Need to return Ok(()) for Result type
    }

    pub fn pull_hash(&self, hash: &str) -> Result<(), Box<dyn Error>> {
        println!("pulling the hash");
        println!("{}", hash);
        Ok(()) // Need to return Ok(()) for Result type
    }
}
