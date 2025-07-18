use std::path::PathBuf;

#[derive(Debug)]
pub struct Repository {
    working: PathBuf,
    rsvcs: PathBuf,
}

impl Repository {
    pub fn new(working: PathBuf) -> Self {
        let rsvcs = working.join(".rsvcs");
        Self { working, rsvcs }
    }
}
