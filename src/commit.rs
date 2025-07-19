use crate::utils::Repository;
use std::error::Error;

impl Repository {
    pub fn commit(&mut self, message: &str) -> Result<String, Box<dyn Error>> {
        let hash = self.make_tarball()?;
        self.latest_commit = Some(hash.clone());
        println!("[{:?}] {}", hash, message);
        Ok(hash)
    }
}
