use std::fs;

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(".rsvcs/commits")?;
    fs::create_dir_all(".rsvcs/logs")?;
    fs::File::create(".rsvcs/config.toml")?;
    Ok(())
}
