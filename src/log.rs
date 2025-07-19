use crate::utils::Repository;
use chrono::{DateTime, Utc};
use std::{
    error::Error,
    fs::OpenOptions,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

impl Repository {
    pub fn add_to_log(&mut self, message: &str, hash: &str) -> Result<(), Box<dyn Error>> {
        let log_path = self.rsvcs.join("log");

        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let datetime: DateTime<Utc> =
            DateTime::from_timestamp(since_epoch.as_secs() as i64, 0).unwrap_or_else(|| Utc::now());
        let timestamp = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut log_file = OpenOptions::new().append(true).open(log_path)?;
        log_file.write_all(format!("{} - [{}] {}\n", timestamp, hash, message).as_bytes())?;
        Ok(())
    }
}
