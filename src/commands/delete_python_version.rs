use std::fs;
use std::path::PathBuf;

pub fn delete_version(version_path: &PathBuf, pyversion: &str, temp_dir: &PathBuf) {
    let temp_version_path = temp_dir.join("to_delete");

    if let Err(e) = fs::rename(&version_path, &temp_version_path) {
        eprintln!(
            "Error 1: Deletion of Python version {} failed: {}",
            pyversion, e
        );
        return;
    }

    if let Err(e) = fs::remove_dir_all(&temp_version_path) {
        println!(
            "Error 2: Deletion of Python version {} failed: {}",
            pyversion, e
        );
    } else {
        println!("Deletion of Python version {} successful", pyversion);
    }
}
