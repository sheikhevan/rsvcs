use crate::utils::Repository;
use std::{fs, io::Write};

impl Repository {
    pub fn init(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.rsvcs)?;
        fs::create_dir_all(&self.commits)?;
        fs::create_dir_all(&self.staging)?;

        fs::File::create(self.rsvcs.join("log"))?;
        if let Err(e) = self.write_default_config() {
            println!("There was an error writing `config.toml`: {}", e);
        };
        let mut description = fs::File::create(self.rsvcs.join("description"))?;
        description
            .write_all(b"This repository is unnamed! Edit this file to name the repository.")?;
        Ok(())
    }
}
