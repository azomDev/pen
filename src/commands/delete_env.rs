use std::fs;
use std::path::PathBuf;

use crate::{ENV_DIR_NAME, TMP_DIR};

pub fn delete_env() {
    let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);
    if !env_dir.exists() || !env_dir.is_dir() {
        eprintln!("Virtual environnement in the current directory does not exist");
        return;
    }

    println!("Deleting the virtual environment in the current directory");

    let temp_version_path = TMP_DIR.join("env_to_delete");

    if let Err(e) = fs::rename(&env_dir, &temp_version_path) {
        eprintln!(
            "Deletion of virual environnement {} failed: {}",
            env_dir.display(),
            e
        );
        return;
    }

    let _ = fs::remove_dir_all(&temp_version_path); // ignore error

    println!(
        "Deletion of virual environnement {} successful",
        env_dir.display()
    );
}
