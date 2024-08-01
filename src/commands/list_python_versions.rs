use std::fs;

use crate::PYTHON_VERSIONS_DIR;

pub fn list() {
    println!("Listing installed Python versions:");

    let entries = match fs::read_dir(&*PYTHON_VERSIONS_DIR) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    let mut python_versions: Vec<String> = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                return;
            }
        };

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                eprintln!("Error reading metadata: {}", e);
                return;
            }
        };

        if metadata.is_dir() {
            match entry.file_name().into_string() {
                Ok(file_name) => python_versions.push(file_name),
                Err(_) => eprintln!("Error converting file name to string"),
            }
        }
    }

    if python_versions.is_empty() {
        println!("No Python versions installed in the specified directory.");
    } else {
        python_versions.sort();  // todo evaluate if this is needed
        for version in python_versions {
            println!("  - {}", version);
        }
    }
}
