use crate::{utils::abort, PYTHON_VERSIONS_DIR};
use std::fs;

pub fn list_py_versions() {
    println!("Listing installed Python versions:");

    let directory_entries = match fs::read_dir(&*PYTHON_VERSIONS_DIR) {
        Ok(entries) => entries,
        Err(e) => abort(&format!("Failed to read {}", &(*PYTHON_VERSIONS_DIR).display()), Some(e))
    };

    let mut installed_versions: Vec<String> = Vec::new();

    for directory_entry in directory_entries {
        let directory_entry = match directory_entry {
            Ok(entry) => entry,
            Err(e) => abort("Failed to read directory entry", Some(e))
        };

        let entry_metadata = match directory_entry.metadata() {
            Ok(metadata) => metadata,
            Err(e) => abort("Failed to read metadata", Some(e))
        };

        if entry_metadata.is_dir() {
            match directory_entry.file_name().into_string() {
                Ok(directory_name) => installed_versions.push(directory_name),
                Err(_) => abort("Failed to convert file name to string", None)
            }
        }
    }

    if installed_versions.is_empty() {
        println!("No Python versions installed with pen.");
    } else {
        installed_versions.sort_unstable();
        for version in installed_versions {
            println!("  - {}", version);
        }
    }
}
