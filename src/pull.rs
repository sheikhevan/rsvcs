use crate::utils::Repository;
use std::{error::Error, fs::File, io::Read};

impl Repository {
    pub fn pull_latest(&self) -> Result<(), Box<dyn Error>> {
        let mut file = match File::open(self.rsvcs.join("latest")) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("There was an error getting the latest commit: {}", e);
                return Err(Box::new(e));
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.pull_tarball(&contents)?;
        Ok(())
    }

    pub fn pull_hash(&self, hash: &str) -> Result<(), Box<dyn Error>> {
        self.pull_tarball(hash)?;
        Ok(())
    }
}
