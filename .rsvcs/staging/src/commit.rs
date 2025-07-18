use crate::utils::Repository;

impl Repository {
    pub fn commit(&self, message: &str) -> std::io::Result<()> {
        println!("{}", message);
        Ok(())
    }
}
