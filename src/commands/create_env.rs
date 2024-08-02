use std::path::PathBuf;
use std::process;
use std::process::Command as ProcessCommand;

use crate::{utils, ENV_DIR_NAME};

use super::install_python_version;

pub fn create_env(pyversion: &str) {
    let full_version: String;
    if utils::is_major_minor(pyversion) {
        full_version = utils::get_latest_patch_version(&pyversion);
    } else if utils::is_major_minor_patch(pyversion) {
        full_version = pyversion.to_string();
    } else {
        println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
        process::exit(1);
    }

    println!(
        "Creating Python virtual environnement with version: {}",
        full_version
    );

    let python_path: std::path::PathBuf = utils::get_version_path(&full_version);
    let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);

    if env_dir.exists() && env_dir.is_dir() {
        eprintln!(
            "env directory {} already exists in current directory",
            &env_dir.display()
        );
        return;
    }

    install_python_version::install_python_version(&full_version);

    let python_bin = python_path.join("bin/python3");

    // todo apparently "expect" will panic on error. If there is an error try to delete the folder if it was partially created. If nothing has been created, exit program with error message.
    if ProcessCommand::new(python_bin)
        .arg("-m")
        .arg("venv")
        .arg(&env_dir)
        .status()
        .expect("Failed to create virtual environment")
        .success()
    // todo there is the .expect and the else to say it failed. Mabye put them together
    {
        println!("Virtual environment created at {}", &env_dir.display());
    } else {
        eprintln!(
            "Failed to create virtual environment at {}",
            &env_dir.display()
        );
    }
}
