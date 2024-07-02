use std::fs;
use std::path::PathBuf;

pub fn delete_version(path: &PathBuf, pyversion: &str) {
    if let Err(e) = fs::remove_dir_all(&path) {
        println!("Deletion of Python version {} failed: {}", pyversion, e);
    } else {
        println!("Deletion of Python version {} successful", pyversion);
    }
}