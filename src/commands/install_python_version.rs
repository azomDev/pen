use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};
use std::{fs, process};

use crate::{utils, TMP_DIR};

const TOTAL_STEPS: u8 = 7;

pub fn install_python_version(pyversion: &str) {
    let full_version: String;
    if utils::is_major_minor(pyversion) {
        full_version = utils::get_latest_patch_version(&pyversion);
    } else if utils::is_major_minor_patch(pyversion) {
        full_version = pyversion.to_string();
    } else {
        println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
        process::exit(1);
    }

    let version_path = utils::get_version_path(&full_version);

    if version_path.exists() {
        println!(
            "The directory for Python version {} already exists, no installing required.",
            full_version
        );
        return;
    }

    println!("Installing Python version: {}", &full_version);

    let temp_tarball_path = TMP_DIR.join("temp_tarball.tgz");

    download_things(&full_version, &temp_tarball_path);

    unpack_build_and_install(&full_version, &version_path, &temp_tarball_path);

    check_install(&version_path, &full_version);

    println!("Python version {} installed successfully.", pyversion);
}

fn check_install(version_path: &PathBuf, pyversion: &str) {
    println!("Verifying install... (7/{})", TOTAL_STEPS);
    let python_bin = version_path.join("bin/python3");
    if ProcessCommand::new(python_bin)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("--version")
        .status()
        .expect("Failed to execute installed Python")
        .success()
        == false
    {
        eprintln!("Failed to verify Python version {}", pyversion);
        fs::remove_dir_all(version_path)
            .expect("FATAL: Failed to cleanup partially installed directory");
        process::exit(1);
    }
}

fn unpack_build_and_install(pyversion: &str, version_path: &PathBuf, temp_tarball_path: &PathBuf) {
    let temp_python_version_dir = TMP_DIR.join("temp_python_version_download");

    let temp_extract_path_dir = TMP_DIR.join("temp_python_extract");

    if let Err(e) = fs::create_dir(&temp_extract_path_dir) {
        eprintln!("Failed to create temp extract directory: {}", e);
        process::exit(1);
    }
    println!("Extracting tarball... (2/{})", TOTAL_STEPS);
    // Extract the tarball
    if ProcessCommand::new("tar")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-xzf")
        .arg(&temp_tarball_path)
        .arg("-C")
        .arg(&temp_extract_path_dir)
        .status()
        .expect("Failed to execute tar")
        .success()
        == false
    {
        println!("Failed to extract Python version {}", pyversion);
        process::exit(1);
    }

    // Configure and install Python
    let source_name = format!("Python-{}", pyversion);
    let source_dir = temp_extract_path_dir.join(PathBuf::from(source_name));
    println!("Configuring Python... (3/{})", TOTAL_STEPS);
    if ProcessCommand::new("./configure")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
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
        println!("Failed to configure Python version {}", pyversion);
        process::exit(1);
    }

    println!(
        "Compiling (this might take a few minutes)... (4/{})",
        TOTAL_STEPS
    );
    if ProcessCommand::new("make")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(&source_dir)
        .status()
        .expect("Failed to execute make")
        .success()
        == false
    {
        println!("Failed to make Python version {}", pyversion);
        process::exit(1);
    }

    println!("Finishing Build... (5/{})", TOTAL_STEPS);
    if ProcessCommand::new("make")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(&source_dir)
        .arg("install")
        .status()
        .expect("Failed to execute make install")
        .success()
        == false
    {
        println!("Failed to install Python version {}", pyversion);
        process::exit(1);
    }

    println!("Moving files... (6/{})", TOTAL_STEPS);
    if let Err(e) = fs::rename(&temp_python_version_dir, &version_path) {
        eprintln!(
            "Error: Installation of Python version {} failed: {}",
            pyversion, e
        );
        // todo mabye try to delete if there is something that still got into path
        process::exit(1);
    }
}

fn download_things(full_version: &str, temp_tarball_path: &PathBuf) {
    println!("Downloading files... (1/{})", TOTAL_STEPS);
    let url = format!(
        "https://www.python.org/ftp/python/{}/Python-{}.tgz",
        full_version, full_version
    );

    // Download the Python tarball
    if !download_file(&url, &temp_tarball_path) {
        eprintln!("Failed to download Python version {}", full_version);
        process::exit(1);
    }
}

fn download_file(url: &str, file_path: &PathBuf) -> bool {
    // Execute curl command to fetch HTTP status code
    let status_code_output = ProcessCommand::new("curl")
        .arg("-s") // silent mode
        .arg("-I")
        .arg("-o")
        .arg("%{http_code}")
        .arg("-w")
        .arg("%{http_code}") // print HTTP status code
        .arg(url)
        .output();

    // Check if fetching HTTP status code was successful
    match status_code_output {
        Ok(output) => {
            // Convert the output bytes to a string
            let status_code_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Parse status code as integer
            if let Ok(status_code) = status_code_str.parse::<u16>() {
                if status_code != 200 {
                    eprintln!(
                        "Error: HTTP status code {} received. File download failed.",
                        status_code
                    );
                    return false;
                }
            } else {
                eprintln!("Error: Failed to parse HTTP status code.");
                return false;
            }
        }
        Err(e) => {
            eprintln!(
                "Error executing curl command to fetch HTTP status code: {}",
                e
            );
            return false;
        }
    }

    // Continue with the actual download
    let download_output = ProcessCommand::new("curl")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-o")
        .arg(file_path)
        .arg("-L")
        .arg(url)
        .output();

    // Check if download was successful
    match download_output {
        Ok(_) => {
            if file_path.exists() {
                return true;
            } else {
                eprintln!("Error: File was not downloaded successfully.");
                return false;
            }
        }
        Err(e) => {
            eprintln!("Error executing curl command to download file: {}", e);
            return false;
        }
    }
}
