use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

use super::install_python_version;

pub fn create_env(version: &str, python_path: &PathBuf, tmp_dir: &PathBuf, env_dir_name: &str) {
    let env_dir = PathBuf::from(".").join(env_dir_name);

    if env_dir.exists() && env_dir.is_dir() {
        println!("env directory already exists in current directory");
        return;
    }

    if !install_python_version::install_version(version, &python_path, &tmp_dir) {
        println!("Failed to create virtual environement because Python version install failed");
        return;
    }

    let temp_venv_path = tmp_dir.join(env_dir_name);
    let python_bin = python_path.join("bin/python3");

    if ProcessCommand::new(python_bin)
        .arg("-m")
        .arg("venv")
        .arg(&temp_venv_path)
        .status()
        .expect("Failed to create virtual environment")
        .success()
    {
        // Move the temporary environment to the target directory
        fs::rename(&temp_venv_path, &env_dir)
            .expect("Failed to move virtual environment to target directory");
        println!("Virtual environment created at {}", env_dir.display());
    } else {
        println!("Failed to create virtual environment");
    }
}
