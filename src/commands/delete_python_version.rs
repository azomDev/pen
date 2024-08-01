use std::fs;
use std::process;

use crate::{utils, TMP_DIR};

pub fn delete_version(pyversion: &str) {
    println!("Deleting Python version: {}", &pyversion);
    let version_path = utils::get_version_path(pyversion);

    if !version_path.exists() || !version_path.is_dir() {
        eprintln!("The Python version {} is not installed.", pyversion);
        return;
    }

    let temp_version_path = TMP_DIR.join("to_delete");

    if let Err(e) = fs::rename(&version_path, &temp_version_path) {
        eprintln!("Deletion of Python version {} failed: {}", pyversion, e);
        return;
    }

    let _ = fs::remove_dir_all(&temp_version_path); // ignore error
    println!("Deletion of Python version {} successful", pyversion);
}
