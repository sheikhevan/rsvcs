use std::{fs, io, path::Path};
use walkdir::{DirEntry, WalkDir};

fn is_excluded(entry: &DirEntry, exclusions: &Vec<String>) -> bool {
    entry
        .path()
        .to_str()
        .map(|s| exclusions.iter().any(|exclusion| s.contains(exclusion)))
        .unwrap_or(false)
}

pub fn backup(
    verbose: bool,
    destinations: &Vec<String>,
    sources: &Vec<String>,
    exclusions: &Vec<String>,
) -> Result<(), io::Error> {
    for destination in destinations {
        for source in sources {
            let walker = WalkDir::new(source).into_iter();
            for entry in walker.filter_entry(|e| !is_excluded(e, exclusions)) {
                let entry = entry?;
                let path = entry.path();
                let destination_path = Path::new(destination).join(path.file_name().unwrap());

                if path.is_file() {
                    fs::copy(path, &destination_path)?;
                    if verbose {
                        println!("{} copied to {}", path.display(), destination)
                    }
                }
            }
        }
    }
    Ok(())
}
