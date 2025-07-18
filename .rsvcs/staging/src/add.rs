use std::fs;
use std::path::Path;

use crate::utils::Repository;

impl Repository {
    fn add_file(&self, source: &Path, staging: &Path) -> std::io::Result<()> {
        if let Some(parent) = staging.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, staging)?;
        Ok(())
    }
    fn add_dir(&self, source: &Path, staging: &Path) -> std::io::Result<()> {
        fs::create_dir_all(staging)?;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let staging_path = staging.join(entry.file_name());

            if source_path.is_dir() {
                self.add_dir(&source_path, &staging_path)?;
            } else {
                self.add_file(&source_path, &staging_path)?;
            }
        }

        Ok(())
    }
    pub fn add(&self, file: &str) -> std::io::Result<()> {
        let source_path = self.working.join(file);
        let staging_path = self.rsvcs.join("staging").join(file);

        if source_path.is_dir() {
            self.add_dir(&source_path, &staging_path)?;
        } else {
            self.add_file(&source_path, &staging_path)?;
        }

        Ok(())
    }
}
