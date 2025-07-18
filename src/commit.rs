use crate::utils::Repository;
use std::{fs, io::Write};

impl Repository {
    pub fn init(&self, message: &str) -> std::io::Result<()> {
        fs::create_dir_all(&self.rsvcs)?;
        fs::create_dir_all(self.rsvcs.join("commits"))?;
        fs::create_dir_all(self.rsvcs.join("staging"))?;

        fs::File::create(self.rsvcs.join("config.toml"))?;
        fs::File::create(self.rsvcs.join("log"))?;
        let mut description = fs::File::create(self.rsvcs.join("desciption"))?;
        description
            .write_all(b"This repository is unnamed! Edit this file to name the repository.")?;
        Ok(())
    }
}
