use crate::PYTHON_VERSIONS_DIR;
use std::{fs, process};

pub fn list_py_versions() {
    println!("Listing installed Python versions:");

    let directory_entries = match fs::read_dir(&*PYTHON_VERSIONS_DIR) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            process::exit(1);
        }
    };

    let mut installed_versions: Vec<String> = Vec::new();

    for directory_entry in directory_entries {
        let directory_entry = match directory_entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                process::exit(1);
            }
        };

        let entry_metadata = match directory_entry.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                eprintln!("Error reading metadata: {}", e);
                process::exit(1);
            }
        };

        if entry_metadata.is_dir() {
            match directory_entry.file_name().into_string() {
                Ok(directory_name) => installed_versions.push(directory_name),
                Err(_) => {
                    eprintln!("Error converting file name to string");
                    process::exit(1);
                }
            }
        }
    }

    if installed_versions.is_empty() {
        println!("No Python versions installed using pen.");
    } else {
        installed_versions.sort();
        for version in installed_versions {
            println!("  - {}", version);
        }
    }
}
