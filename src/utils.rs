use std::{fs, io, path::PathBuf};

#[derive(Debug)]
pub struct Repository {
    pub working: PathBuf,
    pub rsvcs: PathBuf,
    pub staging: PathBuf,
    pub commits: PathBuf,
}

impl Repository {
    pub fn new(working: PathBuf) -> Self {
        let rsvcs = working.join(".rsvcs");
        let staging = rsvcs.join("staging");
        let commits = rsvcs.join("commits");
        Self {
            working,
            rsvcs,
            staging,
            commits,
        }
    }

    pub fn open() -> io::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let repo = Self::new(current_dir);

        if !repo.rsvcs.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No rsvcs repository found in the current directory. Run `rsvcs init` first.",
            ));
        }
        Ok(repo)
    }

    pub fn is_staging_empty(&self) -> io::Result<bool> {
        Ok(fs::read_dir(&self.staging)?.next().is_none())
    }
}
