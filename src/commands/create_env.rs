use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use super::install_python_version;
use std::path::Path;

pub fn create_virtual_environment(version: &str, python_path: &PathBuf) {
    let env_dir = Path::new("./env");
    if env_dir.exists() && env_dir.is_dir() {
        println!("env directory already exists in current directory");
        return;
    }

    install_python_version::install(version, python_path);

    let venv_path = PathBuf::from("./env");
    let python_bin = python_path.join("bin/python3");

    if ProcessCommand::new(python_bin)
        .arg("-m")
        .arg("venv")
        .arg(&venv_path)
        .status()
        .expect("Failed to create virtual environment")
        .success() {
            println!("Virtual environment created at {}", venv_path.display());
    } else {
        println!("Failed to create virtual environment");
        // todo handle errors
    }
}