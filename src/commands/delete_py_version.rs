use crate::{utils, TMP_DIR};
use std::process;

pub fn delete_py_version(py_version: &str) {
    utils::assert_major_minor_patch(&py_version);

    let prompt = format!(
        "Are you sure you want to remove the Python version {} from pen? (y/N)",
        &py_version
    );
    if !utils::confirm_action(&prompt) {
        println!("Removing canceled.");
        process::exit(0);
    }

    let py_version_dir = utils::get_version_path(&py_version);

    if !py_version_dir.exists() {
        eprintln!("The Python version {} is not installed.", &py_version);
        process::exit(0);
    }

    println!("Deleting Python version: {}", &py_version);

    let deleted_py_version_path = TMP_DIR.join("deleted_py_version_temp");

    if !utils::try_deleting_dir(&py_version_dir, Some(&deleted_py_version_path)) {
        eprintln!("catastrophic message idk");
        process::exit(1);
    }

    println!("Deletion of Python version {} successful", py_version);
}
