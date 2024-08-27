use crate::{utils, TMP_DIR};
use std::{fs, path::PathBuf, process};

// todo print where it is at in the process of this function

pub fn unpack_and_install_python_version_v1(
    py_version: &str,
    py_version_dir: &PathBuf,
    temp_tarball_path: &PathBuf,
) {
    let temp_python_version_dir = TMP_DIR.join("temp_python_version_download");

    let temp_extract_path_dir = TMP_DIR.join("temp_python_extract");

    if !utils::try_deleting_dir(&temp_python_version_dir, None) {
        eprintln!(
            "Failed to delete {}, exiting",
            temp_python_version_dir.display()
        )
    }

    if !utils::try_deleting_dir(&temp_extract_path_dir, None) {
        eprintln!(
            "Failed to delete {}, exiting",
            temp_extract_path_dir.display()
        )
    }

    if fs::create_dir(&temp_extract_path_dir).is_err() {
        eprintln!("Failed to create temp extract directory");
        process::exit(1);
    }
    println!("Extracting tarball...");
    // Extract the tarball
    if process::Command::new("tar")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("-xzf")
        .arg(&temp_tarball_path)
        .arg("-C")
        .arg(&temp_extract_path_dir)
        .status()
        .expect("Failed to execute tar")
        .success()
        == false
    {
        println!("Failed to extract Python version {}", py_version);
        process::exit(1);
    }

    // Configure and install Python
    let source_name = format!("Python-{}", py_version);
    let source_dir = temp_extract_path_dir.join(PathBuf::from(source_name));
    println!("Configuring Python...");
    if process::Command::new("./configure")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir(&source_dir)
        .arg(format!(
            "--prefix={}",
            temp_python_version_dir.to_str().unwrap()
        ))
        .status()
        .expect("Failed to execute configure")
        .success()
        == false
    {
        println!("Failed to configure Python version {}", py_version);
        process::exit(1);
    }

    println!("Compiling (this might take a few minutes)...");
    if process::Command::new("make")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir(&source_dir)
        .status()
        .expect("Failed to execute make")
        .success()
        == false
    {
        println!("Failed to make Python version {}", py_version);
        process::exit(1);
    }

    println!("Finishing Build...");
    if process::Command::new("make")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .current_dir(&source_dir)
        .arg("install")
        .status()
        .expect("Failed to execute make install")
        .success()
        == false
    {
        println!("Failed to install Python version {}", py_version);
        process::exit(1);
    }

    // todo check the rest of this file
    println!("Moving files...");
    if fs::rename(&temp_python_version_dir, &py_version_dir).is_err() {
        if utils::try_deleting_dir(&py_version_dir, None) {
            eprintln!(
                "Error: Installation of Python version {} failed",
                py_version
            );
        } else {
            eprintln!("catastrophic mesage idk");
        }
        process::exit(1);
    }
}
