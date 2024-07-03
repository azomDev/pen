use std::fs;
use std::path::PathBuf;

pub fn list(python_versions_dir: &PathBuf) {
    let entries = match fs::read_dir(&python_versions_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

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
                Ok(file_name) => println!("{}", file_name),
                Err(_) => eprintln!("Error converting file name to string"),
            }
        }
    }
}
