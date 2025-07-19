use sha2::{Digest, Sha256};
use std::error::Error;
use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

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

    pub fn make_tarball(&self) -> Result<String, Box<dyn Error>> {
        fs::create_dir_all(&self.commits)?;

        let tarball_path = self.commits.join("tmp.tar.zst");
        let file_ref = File::create(&tarball_path)?;
        let encoder = zstd::Encoder::new(file_ref, 0)?;
        let mut archive = tar::Builder::new(encoder);

        archive.append_dir_all(".", &self.staging)?;

        archive.into_inner()?.finish()?;
        println!("Sucessfully created the tarball.");

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
}
