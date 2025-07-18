use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

fn is_excluded(entry: &DirEntry, exclusions: &Vec<String>) -> bool {
    entry
        .path()
        .to_str()
        .map(|s| exclusions.iter().any(|exclusion| s.contains(exclusion)))
        .unwrap_or(false)
}

pub fn backup(
    destinations: &Vec<String>,
    sources: &Vec<String>,
    exclusions: &Vec<String>,
) -> Result<String, io::Error> {
    for destination in destinations {
        for source in sources {
            let walker = WalkDir::new(source).into_iter();
            for entry in walker.filter_entry(|e| !is_excluded(e, exclusions)) {
                println!("{} copied to {}", entry?.path().display(), destination)
            }
        }
    }
    Ok("The backup was successful!".to_string())
}
