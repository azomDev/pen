use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::path::Path;

use super::install_python_version;

pub fn create_env(version: &str, python_path: &PathBuf) {
    let env_dir = Path::new("./env");
    if env_dir.exists() && env_dir.is_dir() {
        println!("env directory already exists in current directory");
        return;
    }

    if !install_python_version::install_version(version, python_path) {
        println!("Failed to create virtual environement because Python version install failed");
        return;
    }

    let temp_venv_path = PathBuf::from("/tmp/env");
    let python_bin = python_path.join("bin/python3");

    // Remove any existing temporary environment
    if temp_venv_path.exists() {
        fs::remove_dir_all(&temp_venv_path).expect("Failed to remove existing temporary environment");
    }

    if ProcessCommand::new(python_bin)
        .arg("-m")
        .arg("venv")
        .arg(&temp_venv_path)
        .status()
        .expect("Failed to create virtual environment")
        .success() {
            // Move the temporary environment to the target directory
            fs::rename(&temp_venv_path, &env_dir).expect("Failed to move virtual environment to target directory");
            println!("Virtual environment created at {}", env_dir.display());
    } else {
        println!("Failed to create virtual environment");
    }
}
