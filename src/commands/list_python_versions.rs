use std::fs;
use std::io;
use std::path::PathBuf;
use std::env;
use std::path::Path;


pub fn list() {
    let home_dir = env::var("HOME").expect("HOME environment variable is not set");
    let version_dir = Path::new(&home_dir).join(".pen/pythonVersions");
    pub fn list_versions(path: &PathBuf) -> io::Result<()> {
        let entries = fs::read_dir(path)?;
        
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            if metadata.is_dir() {
                println!("{}", entry.file_name().into_string().unwrap());
            }
        }

        Ok(())
    }
    
    if let Err(e) = list_versions(&version_dir) {
        eprintln!("Error reading directory: {}", e);
    }
}