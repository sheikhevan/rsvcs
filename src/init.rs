use std::{fs, io::Write};

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(".rsvcs/commits")?;
    fs::create_dir_all(".rsvcs/staging")?;
    fs::File::create(".rsvcs/config.toml")?;
    fs::File::create(".rsvcs/log")?;
    let mut description = fs::File::create(".rsvcs/description")?;
    description.write_all(b"This repository is unnamed! Edit this file to name the repository.")?;
    Ok(())
}
