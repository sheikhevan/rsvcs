use crate::utils::Repository;
use std::{error::Error, fs};

impl Repository {
    pub fn commit(&mut self, message: &str) -> Result<String, Box<dyn Error>> {
        let hash = self.make_tarball()?;
        println!("Commit successful!");
        println!("\"{}\" [{}]", message, hash.to_string());
        self.add_to_log(message, &hash)?;
        fs::remove_dir_all(self.rsvcs.join("staging"))?;
        fs::create_dir_all(self.rsvcs.join("staging"))?;
        Ok(hash)
    }
}
