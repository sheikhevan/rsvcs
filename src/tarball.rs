use crate::utils::Repository;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::{
    fs::{self, File},
    io::Read,
};

impl Repository {
    pub fn make_tarball(&self) -> Result<String, Box<dyn Error>> {
        fs::create_dir_all(&self.commits)?;

        let tarball_path = self.commits.join("tmp.tar.zst");
        let file_ref = File::create(&tarball_path)?;
        let encoder = zstd::Encoder::new(file_ref, 0)?;
        let mut archive = tar::Builder::new(encoder);

        archive.append_dir_all(".", &self.staging)?;

        archive.into_inner()?.finish()?;
        println!("Sucessfully created the tarball!");

        // Now we must generate the sha256 hash
        let mut file = File::open(&tarball_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let hash = format!("{:x}", hasher.finalize());

        let final_tarball_path = self.commits.join(format!("{}.tar.zst", hash));
        fs::rename(&tarball_path, &final_tarball_path)?;

        Ok(hash)
    }
    pub fn pull_tarball(&self, hash: &str) -> Result<(), Box<dyn Error>> {
        let tarball_path = self.commits.join(format!("{}.tar.zst", hash));
        let file_ref = File::open(&tarball_path)?;
        let decoder = zstd::Decoder::new(file_ref)?;
        let mut archive = tar::Archive::new(decoder);
        archive.unpack(".")?;
        archive.into_inner().finish();

        println!("Sucessfully unpacked the tarball!");

        Ok(())
    }
}
