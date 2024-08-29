use super::install_py_version;
use crate::{utils, ENV_DIR_NAME, TMP_DIR};
use std::{fs, path::PathBuf, process};

pub fn create_env(py_version: &str) {
    utils::assert_major_minor_patch(&py_version);

    let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);

    match fs::metadata(&env_dir) {
        Ok(metadata) => {
            // Check if the metadata corresponds to a directory
            if metadata.is_dir() {
                eprintln!(
                    "env directory {} already exists in current directory",
                    &env_dir.display()
                );
                process::exit(1);
            } else {
                eprintln!(
                    "There is already a file named {} in the current directory, aborting",
                    ENV_DIR_NAME
                );
                process::exit(1);
            }
        }
        Err(e) => {
            // If there was an error, check if it is a "not found" error (which means the directory does not exist)
            if e.kind() == std::io::ErrorKind::NotFound {
                // Directory does not exist, proceed
            } else {
                // Handle other potential errors (e.g., permission denied)
                eprintln!("Error checking directory: {}", e);
                process::exit(1);
            }
        }
    }

    println!(
        "Creating Python virtual environnement with version: {}",
        &py_version
    );

    install_py_version(&py_version);

    let py_version_dir = utils::get_version_path(&py_version);
    let py_binary = py_version_dir.join("bin/python3");

    // Attempt to create the virtual environment
    let status = process::Command::new(&py_binary)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("-m")
        .arg("venv")
        .arg(&env_dir)
        .status();

    fn handle_failure(env_dir: &PathBuf) {
        if !utils::try_deleting_dir(&env_dir, Some(&TMP_DIR.join("deleted_env_temp"))) {
            eprintln!(
                "Catastrophic failure: Unable to delete {}. Manual cleanup required",
                &env_dir.display()
            );
            process::exit(1);
        }
    }

    match status {
        Ok(status) => {
            if status.success() {
                println!("Virtual environment created at {}", &env_dir.display());
            } else {
                // Status is Ok but the command was not successful.
                eprintln!("Command was not successful");
                handle_failure(&env_dir);
            }
        }
        Err(e) => {
            // Error occurred when trying to run the command.
            eprintln!("Failed to execute command: {}", e);
            handle_failure(&env_dir);
        }
    }
}
