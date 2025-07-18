use std::fs;

pub fn backup(destinations: &Vec<String>, sources: &Vec<String>) {
    for destination in destinations {
        for source in sources {
            println!("{} -> {}", source, destination)
        }
    }
}
