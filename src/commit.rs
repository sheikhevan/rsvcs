use crate::utils::Repository;
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

impl Repository {
    fn add_latest_commit(&self, hash: &str) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.rsvcs.join("latest"))?;
        file.write_all(format!("{}", hash).as_bytes())?;
        Ok(())
    }
    pub fn commit(&mut self, message: &str) -> Result<String, Box<dyn Error>> {
        let hash = self.make_tarball()?;
        println!("Commit successful!");
        println!("\"{}\" [{}]", message, hash.to_string());
        self.add_to_log(message, &hash)?;
        self.add_latest_commit(&hash)?;
        fs::remove_dir_all(self.rsvcs.join("staging"))?;
        fs::create_dir_all(self.rsvcs.join("staging"))?;
        Ok(hash)
    }
}
