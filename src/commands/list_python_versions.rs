use std::fs;
use std::env;
use std::path::Path;

pub fn list() {
    let home_dir = match env::var("HOME") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting HOME environment variable: {}", e);
            return;
        }
    };

    let version_dir = Path::new(&home_dir).join(".pen/pythonVersions");

    let entries = match fs::read_dir(&version_dir) {
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
