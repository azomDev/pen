use semver::Version;

use crate::constants::TMP_DIR;
use crate::utils::{self, abort, catastrophic_failure};
use std::{fs, path::PathBuf, process};

pub fn py_install_algo_v1(version: &Version) {
    let version_dir = utils::get_python_path(&version);

    match version_dir.try_exists() {
        Ok(true) => {
            println!("{} is already installed", version_dir.display());
            return;
        }
        Ok(false) => {}
        Err(e) => {
            abort(
                &format!(
                    "Failed to check if {} already exists",
                    version_dir.display()
                ),
                Some(&e),
            );
        }
    }

    println!("Installing Python version: {}", &version);

    let temp_tarball_path = TMP_DIR.join("temp_tarball.tgz"); // todo remove hardcoded value
    let python_tarball_url = format!(
        "https://www.python.org/ftp/python/{}/Python-{}.tgz", // todo remove hardcoded value
        &version, &version
    );

    println!("Downloading Python installation files.");
    utils::download_file(&python_tarball_url, &temp_tarball_path);

    println!("Building Python from source.");
    unpack_and_install_python_version_v1(&version, &version_dir, &temp_tarball_path);

    println!("Verifying Python install.");
    let python_bin = version_dir.join("bin/python3"); // todo remove hardcoded value
    match process::Command::new(python_bin)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("--version")
        .status()
    {
        Ok(status) if status.success() => {
            println!("Python version {} installed successfully.", &version);
            return;
        }
        Ok(_) => eprintln!("Error: Failed to verify if Python version is installed"),
        Err(e) => eprintln!(
            "Error: Failed to verify if Python version is installed: {}",
            e
        ),
    }
    match utils::try_deleting_dir(&version_dir) {
        Ok(()) => process::exit(1),
        Err(e) => catastrophic_failure("todo", Some(&e)),
    }
}

fn unpack_and_install_python_version_v1(
    py_version: &Version,
    py_version_dir: &PathBuf,
    temp_tarball_path: &PathBuf,
) {
    // Preparing directories
    let temp_extract_path_dir = TMP_DIR.join("temp_python_extract");
    let temp_python_version_dir = TMP_DIR.join("temp_python_version_download");
    if let Err(e) = utils::try_deleting_dir(&temp_python_version_dir) {
        abort(
            &format!("Failed to delete {}", temp_python_version_dir.display()),
            Some(&e),
        );
    }
    if let Err(e) = utils::try_deleting_dir(&temp_extract_path_dir) {
        abort(
            &format!("Failed to delete {}", temp_extract_path_dir.display()),
            Some(&e),
        );
    }
    if let Err(e) = fs::create_dir(&temp_extract_path_dir) {
        abort("Failed to create temp extract directory", Some(&e));
    }

    println!("Extracting tarball...");

    match process::Command::new("tar")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("-xzf")
        .arg(&temp_tarball_path)
        .arg("-C")
        .arg(&temp_extract_path_dir)
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(_) => abort(
            &format!("Failed to extract Python version {}", py_version),
            None,
        ),
        Err(e) => abort(
            &format!("Failed to extract Python version {}", py_version),
            Some(&e),
        ),
    }

    println!("Configuring Python...");

    let source_name = format!("Python-{}", py_version);
    let source_dir = temp_extract_path_dir.join(PathBuf::from(source_name));
    match process::Command::new("./configure")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir(&source_dir)
        .arg(format!("--prefix={}", temp_python_version_dir.display()))
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(_) => abort(
            &format!("Failed to configure Python version {}", py_version),
            None,
        ),
        Err(e) => abort(
            &format!("Failed to configure Python version {}", py_version),
            Some(&e),
        ),
    }

    println!("Compiling (this might take a few minutes)...");

    match process::Command::new("make")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir(&source_dir)
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(_) => abort(
            &format!("Failed to make Python version {}", py_version),
            None,
        ),
        Err(e) => abort(
            &format!("Failed to make Python version {}", py_version),
            Some(&e),
        ),
    }

    println!("Finishing Build...");

    match process::Command::new("make")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir(&source_dir)
        .arg("install")
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(_) => abort(
            &format!("Failed to install Python version {}", py_version),
            None,
        ),
        Err(e) => abort(
            &format!("Failed to install Python version {}", py_version),
            Some(&e),
        ),
    }

    println!("Moving files...");

    if fs::rename(&temp_python_version_dir, &py_version_dir).is_err() {
        if let Err(e) = utils::try_deleting_dir(&py_version_dir) {
            catastrophic_failure("catastrophic message idk", Some(&e));
        } else {
            abort(
                &format!("Failed to move Python version {}", py_version),
                None,
            );
        }
    }
}
