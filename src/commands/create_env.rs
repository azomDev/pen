use super::install_py_version;
use crate::{utils::{self, abort, catastrophic_failure}, ENV_DIR_NAME};
use std::{path::PathBuf, process};

pub fn create_env(py_version: &str) {
    utils::assert_major_minor_patch(&py_version);

    let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);

    match env_dir.try_exists() {
        Ok(true) => abort(&format!("{} already exists", env_dir.display()), None),
        Ok(false) => { /* Directory does not exist, proceed */ },
        Err(e) => abort(&format!("Failed to check if {} already exists", env_dir.display()), Some(e))
    }

    println!("Creating Python virtual environnement with version {}", py_version);

    install_py_version(&py_version);

    let py_binary = utils::get_version_path(&py_version).join("bin/python3");

    // Attempt to create the virtual environment
    let status = process::Command::new(&py_binary)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("-m")
        .arg("venv")
        .arg(&env_dir)
        .status();

    fn handle_failure(env_dir: &PathBuf, err_msg: &str) -> ! {
        eprintln!("Error: Failed to create virtual environement, cleaning up : {}", err_msg);
        if let Err(e) = utils::try_deleting_dir(env_dir) { // todo this could fail if on different filesystem
            catastrophic_failure(&format!("Unable to delete {}", env_dir.display()), Some(e));
        }
        process::exit(1);
    }

    match status {
        Ok(status) if status.success() => println!("Virtual environment created at {}", env_dir.display()),
        Ok(_) => handle_failure(&env_dir, "Command was not successful"),
        Err(e) => handle_failure(&env_dir, &e.to_string()),
    }
}
