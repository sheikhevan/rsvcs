use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct Repository {
    pub working: PathBuf,
    pub rsvcs: PathBuf,
}

impl Repository {
    pub fn new(working: PathBuf) -> Self {
        let rsvcs = working.join(".rsvcs");
        Self { working, rsvcs }
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
}
