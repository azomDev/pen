use crate::utils::{self, catastrophic_failure};
use std::process;

pub fn py_delete_version(py_version: &String) {
    let py_version = utils::user_string_to_version(Some(py_version));
    let py_version_dir = utils::get_python_path(&py_version);

    if !py_version_dir.exists() || !py_version_dir.is_dir() {
        eprintln!(
            "Error: The Python version {} is not installed.",
            &py_version
        );
        process::exit(0);
    }

    let prompt = format!(
        "Are you sure you want to remove the Python version {} from pen? (y/N)",
        &py_version
    );
    if !utils::confirm_action(&prompt) {
        println!("Removing canceled");
        process::exit(0);
    }

    println!("Deleting Python version {}", &py_version);

    if let Err(e) = utils::try_deleting_dir(&py_version_dir) {
        catastrophic_failure("todo", Some(&e));
    }

    println!("Deletion of Python version {} successful", py_version);
}
