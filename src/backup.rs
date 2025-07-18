use std::{fs, io, path::PathBuf};
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
) -> Result<(), Box<dyn std::error::Error>> {
    let trimmed_exclusions: Vec<String> = exclusions
        .iter()
        .map(|s| s.strip_suffix('/').unwrap_or(&s).to_string())
        .collect();

    for destination in destinations {
        for source in sources {
            let source_path = PathBuf::from(source);
            let dest_path = PathBuf::from(destination);
            for entry in WalkDir::new(&source_path)
                .into_iter()
                .filter_entry(|e| !is_excluded(e, &trimmed_exclusions))
            {
                let entry = entry?;

                let from = entry.path();
                let to = dest_path
                    .join(source_path.file_name().unwrap())
                    .join(from.strip_prefix(&source_path)?);
                println!("{:?}", source_path);
                if verbose {
                    println!("{} copied to {}", from.display(), to.display())
                }

                // this creates the directories
                if entry.file_type().is_dir() && !is_excluded(&entry, &trimmed_exclusions) {
                    if let Err(e) = fs::create_dir(to) {
                        match e.kind() {
                            io::ErrorKind::AlreadyExists => {}
                            _ => return Err(e.into()),
                        }
                    }
                }
                // copy the files over
                else if entry.file_type().is_file() {
                    fs::copy(from, to)?;
                } else {
                    eprintln!("Ignored {}", from.display());
                }
            }
        }
    }
    Ok(())
}
