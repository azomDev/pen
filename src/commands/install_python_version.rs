use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::fs;

pub fn install_version(version: &str, path: &PathBuf) -> bool {
    if path.exists() {
        println!("The folder for Python version {} already exists, no installing required.", version);
        return true;
    }
    println!("The folder for Python version {} does not exist. Installing...", version);

    let url = format!("https://www.python.org/ftp/python/{}/Python-{}.tgz", version, version);
    let tarball_path = format!("/tmp/Python-{}.tgz", version);

    // Create the directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(path) {
        println!("Failed to create directory: {}", e);
        return false;
    }

    // Convert relative path to absolute path
    let absolute_path = fs::canonicalize(path).expect("Failed to get absolute path");

    // Download the Python tarball
    if ProcessCommand::new("curl")
        .arg("-o")
        .arg(&tarball_path)
        .arg(&url)
        .status()
        .expect("Failed to execute curl")
        .success() == false
        {
            println!("Failed to download Python version {}", version);
            return false;
        }

    // Extract the tarball
    if ProcessCommand::new("tar")
        .arg("-xzf")
        .arg(&tarball_path)
        .arg("-C")
        .arg("/tmp")
        .status()
        .expect("Failed to execute tar")
        .success() == false
        {
            println!("Failed to extract Python version {}", version);
            return false;
        }

    // Configure and install Python
    let source_dir = format!("/tmp/Python-{}", version);
    if ProcessCommand::new("./configure")
        .current_dir(&source_dir)
        .arg(format!("--prefix={}", absolute_path.to_str().unwrap()))
        .status()
        .expect("Failed to execute configure")
        .success() == false
        {
            println!("Failed to configure Python version {}", version);
            return false;
        }
    
    if ProcessCommand::new("make")
        .current_dir(&source_dir)
        .status()
        .expect("Failed to execute make")
        .success() == false
        {
            println!("Failed to make Python version {}", version);
            return false;
        }

    if ProcessCommand::new("make")
        .current_dir(&source_dir)
        .arg("install")
        .status()
        .expect("Failed to execute make install")
        .success() == false
        {
            println!("Failed to install Python version {}", version);
            return false;
        }

    // Verify the installation
    let python_bin = absolute_path.join("bin/python3");
    if ProcessCommand::new(python_bin)
        .arg("--version")
        .status()
        .expect("Failed to execute installed Python")
        .success() == false
        {
            println!("Failed to verify Python version {}", version);
            return false;
        }

    println!("Finishing install");

    // Cleanup
    fs::remove_file(tarball_path).expect("Failed to remove tarball");
    fs::remove_dir_all(source_dir).expect("Failed to remove source directory");

    true
}