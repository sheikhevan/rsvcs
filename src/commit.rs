use crate::utils::Repository;

impl Repository {
    pub fn commit(&self, message: &str) -> std::io::Result<()> {
        let hash = self.make_tarball();
        println!("[{:?}] {}", &hash.unwrap(), message);
        Ok(())
    }
}
