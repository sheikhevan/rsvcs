use std::{
    fs, io,
    path::{Path, PathBuf},
};
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
    for destination in destinations {
        for source in sources {
            let source_path = PathBuf::from(source);
            let dest_path = PathBuf::from(destination);
            for entry in WalkDir::new(&source_path)
                .into_iter()
                .filter_entry(|e| !is_excluded(e, exclusions))
            {
                let entry = entry?;

                let from = entry.path();
                let to = dest_path.join(from.strip_prefix(&source_path)?);
                if verbose {
                    println!("{} copied to => {}", from.display(), to.display())
                }

                // this creates the directories
                if entry.file_type().is_dir() {
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
                    eprintln!("ignored {}", from.display());
                }
            }
        }
    }
    Ok(())
}
