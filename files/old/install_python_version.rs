use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

pub fn install_version(version: &str, path: &PathBuf, tmp_dir: &PathBuf) -> bool {
    if path.exists() {
        println!(
            "The folder for Python version {} already exists, no installing required.",
            version
        );
        return true;
    }
    println!(
        "The folder for Python version {} does not exist. Installing...",
        version
    );

    let url = format!(
        "https://www.python.org/ftp/python/{}/Python-{}.tgz",
        version, version
    );
    let tarball_path = format!("Python-{}.tgz", version);
    let tarball_file = tmp_dir.join(PathBuf::from(&tarball_path));
    println!("{}", tmp_dir.display());
    println!("{}", tarball_file.display());

    // Create the directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(path) {
        println!("Failed to create directory: {}", e);
        return false;
    }

    // Convert relative path to absolute path
    let absolute_path = fs::canonicalize(path).expect("Failed to get absolute path");

    // Download the Python tarball
    if !download_file(&url, &tarball_file) {
        eprintln!("Failed to download Python version {}", version);
        fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
        return false;
    }
    // if ProcessCommand::new("curl")
    //     .arg("-o")
    //     .arg(&tarball_path)
    //     .arg(&url)
    //     .status()
    //     .expect("Failed to execute curl")
    //     .success()
    //     == false
    // {
    //     println!("Failed to download Python version {}", version);
    //     fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
    //     return false;
    // }

    // Extract the tarball
    if ProcessCommand::new("tar")
        .arg("-xzf")
        .arg(&tarball_file)
        .arg("-C")
        .arg(tmp_dir)
        .status()
        .expect("Failed to execute tar")
        .success()
        == false
    {
        println!("Failed to extract Python version {}", version);
        fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
        return false;
    }

    // Configure and install Python
    let source_name = format!("Python-{}", version);
    let source_dir = tmp_dir.join(PathBuf::from(source_name));
    if ProcessCommand::new("./configure")
        .current_dir(&source_dir)
        .arg(format!("--prefix={}", absolute_path.to_str().unwrap()))
        .status()
        .expect("Failed to execute configure")
        .success()
        == false
    {
        println!("Failed to configure Python version {}", version);
        fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
        return false;
    }

    if ProcessCommand::new("make")
        .current_dir(&source_dir)
        .status()
        .expect("Failed to execute make")
        .success()
        == false
    {
        println!("Failed to make Python version {}", version);
        fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
        return false;
    }

    if ProcessCommand::new("make")
        .current_dir(&source_dir)
        .arg("install")
        .status()
        .expect("Failed to execute make install")
        .success()
        == false
    {
        println!("Failed to install Python version {}", version);
        fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
        return false;
    }

    // Verify the installation
    let python_bin = absolute_path.join("bin/python3");
    if ProcessCommand::new(python_bin)
        .arg("--version")
        .status()
        .expect("Failed to execute installed Python")
        .success()
        == false
    {
        println!("Failed to verify Python version {}", version);
        fs::remove_dir_all(path).expect("Failed to cleanup partially installed directory");
        return false;
    }

    println!("Finishing install");

    // Cleanup
    fs::remove_file(tarball_file).expect("Failed to remove tarball");
    fs::remove_dir_all(source_dir).expect("Failed to remove source directory");

    true
}

fn linux_python_install(path: &PathBuf, tmp_dir: &PathBuf) {}

fn download_file(url: &str, file_path: &PathBuf) -> bool {
    let client = match Client::new()
        .get(url)
        .header(USER_AGENT, "Rust reqwest")
        .send()
    {
        Ok(response) => match response.error_for_status() {
            Ok(valid_response) => valid_response,
            Err(_) => {
                eprintln!("Error 1");
                return false;
            }
        },
        Err(_) => {
            eprintln!("Error 2");
            return false;
        }
    };

    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error 3: {}", e);
            return false;
        }
    };

    let content = match client.bytes() {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("Error 4");
            return false;
        }
    };

    if copy(&mut content.as_ref(), &mut file).is_err() {
        eprintln!("Error 5");
        return false;
    }

    return true;
}
