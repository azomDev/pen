use crate::utils::{self, abort};
use std::process;

pub fn delete_py_version(py_version: &str) {
    utils::assert_major_minor_patch(&py_version);

    let py_version_dir = utils::get_version_path(&py_version);

    if !py_version_dir.exists() || !py_version_dir.is_dir() {
        eprintln!("The Python version {} is not installed.", &py_version);
        process::exit(0);
    }

    let prompt = format!("Are you sure you want to remove the Python version {} from pen? (y/N)", &py_version);
    if !utils::confirm_action(&prompt) {
        println!("Removing canceled");
        process::exit(0);
    }

    println!("Deleting Python version {}", &py_version);

    if let Err(e) = utils::try_deleting_dir(&py_version_dir) {
        abort(&format!("catastrophic failure or smth {}", py_version_dir.display()), Some(e));
    }

    println!("Deletion of Python version {} successful", py_version);
}
