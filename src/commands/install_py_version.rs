use crate::{py_install_algorithms, utils, TMP_DIR};
use std::{path::PathBuf, process};

pub fn install_py_version(py_version: &str) {
    utils::assert_major_minor_patch(&py_version);

    let py_version_dir = utils::get_version_path(&py_version);

    if is_py_version_installed(&py_version_dir) {
        println!("Directory {} already exists", py_version_dir.display());
        return;
    }

    println!("Installing Python version: {}", &py_version);

    let temp_tarball_path = TMP_DIR.join("temp_tarball.tgz");

    let python_tarball_url = format!(
        "https://www.python.org/ftp/python/{}/Python-{}.tgz",
        &py_version, &py_version
    );

    println!("Downloading Python installation files.");
    utils::download_file(&python_tarball_url, &temp_tarball_path);

    println!("Building Python from source.");
    py_install_algorithms::unpack_and_install_python_version_v1(
        &py_version,
        &py_version_dir,
        &temp_tarball_path,
    );

    println!("Verifying Python install.");
    if !is_py_version_installed(&py_version_dir) {
        if utils::try_deleting_dir(&py_version_dir, Some(&TMP_DIR.join("deleted_python_temp"))) {
            eprintln!("Failed to install python version");
        } else {
            eprintln!("some catastrophic message idk");
        }
        process::exit(1);
    }

    println!("Python version {} installed successfully.", &py_version);
}

fn is_py_version_installed(py_version_dir: &PathBuf) -> bool {
    let python_bin = py_version_dir.join("bin/python3");
    match process::Command::new(python_bin)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("--version")
        .status()
    {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!(
                "There might be a problem with {}: {}",
                py_version_dir.display(),
                e
            );
            process::exit(1);
        }
    }
}
