use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || !args[2].starts_with("--pyversion=") {
        eprintln!("Usage: {} --pyversion=<PYVERSION>", args[0]);
        std::process::exit(1);
    }

    let pyversion = &args[2][12..];
    let home_dir = env::var("HOME").expect("HOME environment variable is not set");
    let projects_dir = Path::new(&home_dir).join(".pen/pythonVersions");
    let version_dir_name = format!("python_{}", pyversion);
    let version_path = projects_dir.join(&version_dir_name);

    if version_path.exists() {
        println!("The folder for Python version {} already exists, no installing required.", pyversion);
    } else {
        println!("The folder for Python version {} does not exist. Installing...", pyversion);
        if install_python_version(pyversion, &version_path) {
            println!("Successfully installed Python version {}", pyversion);
        } else {
            println!("Failed to install Python version {}", pyversion);

            if version_path.exists() {
                if let Err(e) = fs::remove_dir_all(&version_path) {
                    println!("Failed to remove directory {}: {}", version_path.display(), e);
                } else {
                    println!("Removed directory {} after failed installation", version_path.display());
                }
            }
            std::process::exit(1);
        }
    }
    create_virtual_environment(&version_path);
}

fn install_python_version(version: &str, path: &PathBuf) -> bool {
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

    // Cleanup
    fs::remove_file(tarball_path).expect("Failed to remove tarball");
    fs::remove_dir_all(source_dir).expect("Failed to remove source directory");

    true
}

fn create_virtual_environment(python_path: &PathBuf) {
    let venv_path = PathBuf::from("./env");
    let python_bin = python_path.join("bin/python3");

    if ProcessCommand::new(python_bin)
        .arg("-m")
        .arg("venv")
        .arg(&venv_path)
        .status()
        .expect("Failed to create virtual environment")
        .success() {
            println!("Virtual environment created at {}", venv_path.display());
    } else {
        println!("Failed to create virtual environment");
    }
}
