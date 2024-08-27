use crate::PYTHON_VERSIONS_DIR;
use regex::Regex;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process,
};

/// Asserts that a Python version string matches the major.minor.patch format.
///
/// This function checks if the provided `py_version` string adheres to the format
/// `major.minor.patch` (e.g., "3.9.1"). It uses a regular expression to validate
/// the format. If the version string does not match the expected format, it prints
/// an error message and terminates the program with an exit code of `1`.
///
/// # Arguments
///
/// * `py_version` - A `&str` representing the Python version to be validated.
///
/// # Error Handling
///
/// The function will exit the program with an exit code of `1` in the following scenarios:
///
/// * **Invalid Format**: If the `py_version` does not match the `major.minor.patch` format,
///   as validated by the regular expression. An error message will be printed to
///   the standard error output.
pub fn assert_major_minor_patch(py_version: &str) {
    let re = Regex::new(r"^\d+\.\d+\.\d+$").expect("Invalid regex pattern");
    if !re.is_match(py_version) {
        eprintln!(
            "Version '{}' does not match the major.minor.patch format",
            py_version
        );
        process::exit(1);
    }
}

/// Constructs the path to the directory for a specified Python version.
///
/// This function generates a directory path by combining a predefined base directory
/// (`PYTHON_VERSIONS_DIR`) with a directory name that is formatted based on the provided
/// Python version string. The resulting path corresponds to where the Python version's
/// files are expected to be located.
///
/// # Arguments
///
/// * `py_version` - A `&str` representing the Python version (e.g., "3.9.1").
///
/// # Returns
///
/// * `PathBuf` - The constructed path to the directory for the specified Python version.
///   This path is derived by appending a directory name of the form `python_<py_version>`
///   to the base directory `PYTHON_VERSIONS_DIR`.
pub fn get_version_path(py_version: &str) -> PathBuf {
    let py_version_dir_name = format!("python_{}", py_version);
    return PYTHON_VERSIONS_DIR.join(py_version_dir_name);
}

/// Prompts the user with a message and returns a boolean based on their input.
///
/// This function displays a prompt message to the user and reads their response from
/// the standard input. It then returns `true` if the user’s input, after trimming
/// and case-insensitivity check, is "y" (or "Y"). It returns `false` for any other input.
///
/// # Arguments
///
/// * `prompt` - A `&str` representing the message to be displayed to the user.
///
/// # Returns
///
/// * `true` if the user input (case-insensitive) is "y" or "Y".
/// * `false` for any other input.
pub fn confirm_action(prompt: &str) -> bool {
    println!("{}", prompt);

    // Flush stdout to ensure the prompt appears before reading input
    io::stdout().flush().unwrap();

    // Read user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    return user_input.trim().eq_ignore_ascii_case("y");
}

/// Downloads a file from a specified URL and saves it to a given path.
///
/// This function uses the `curl` command-line tool to download a file from `file_url`
/// and saves it to `file_path`. The download will use IPv4 and suppress progress output.
///
/// # Arguments
///
/// * `file_url` - A `&str` representing the URL of the file to be downloaded.
/// * `file_path` - A `PathBuf` representing the path where the downloaded file should be saved.
///
/// # Error Handling
///
/// The function may exit in the following scenarios:
///
/// * **`curl` Command Failure**: If the `curl` command fails to execute (e.g., `curl` is not installed or an execution error occurs).
/// * **File Not Found**: After the `curl` command completes, if the downloaded file is not found at `file_path` (i.e., the file does not exist or is not a regular file).
pub fn download_file(file_url: &str, file_path: &PathBuf) {
    if fs::remove_file(file_path).is_err() {
        eprintln!("Unable to remove file to download the new one, exiting");
        process::exit(1);
    }
    let curl_status = process::Command::new("curl")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("-4")
        .arg("-s")
        .arg("-o")
        .arg(file_path)
        .arg("-L")
        .arg(file_url)
        .status()
        .expect("Error executing curl command");

    if !curl_status.success() {
        eprintln!("Curl command failed with status: {}", curl_status);
        process::exit(1);
    }

    if !file_path.exists() || !file_path.is_file() {
        eprintln!("Downloaded file was not found.");
        process::exit(1);
    }
}

/// Attempts to delete a specified directory, with an optional temporary move.
///
/// This function attempts to delete the directory at `dir_path`. If `delete_path`
/// is provided, the function first renames the directory to the temporary path and
/// then attempts to delete the directory from that temporary location. If renaming
/// fails, it tries to remove the directory directly from `dir_path`. If `delete_path`
/// is `None`, it directly attempts to remove the directory from `dir_path`.
///
/// # Arguments
///
/// * `dir_path` - A `&PathBuf` representing the path to the directory that is to be deleted.
/// * `delete_path` - An `Option<&PathBuf>` representing an optional temporary path to move
///   the directory before deletion. If `Some(temp_path)` is provided, the directory will
///   be renamed to `temp_path` before deletion. If `None`, the directory will be deleted directly.
///
/// # Returns
///
/// * `true` if `dir_path` no longer exists.
/// * `false` if `dir_path` still exists after attempting to delete it, indicating
///   that the deletion was unsuccessful.
///
/// # Error Handling
///
/// The function is not supposed to exit the program.
pub fn try_deleting_dir(dir_path: &PathBuf, delete_path: Option<&PathBuf>) -> bool {
    if let Some(temp_path) = delete_path {
        // todo check if rename overwrites even when renaming dirs. If no, delete the temp_path dir at the beginning and return false on failure
        if fs::rename(dir_path, &temp_path).is_ok() {
            let _ = fs::remove_dir_all(&temp_path);
        } else {
            let _ = fs::remove_dir_all(&dir_path);
        }
    } else {
        let _ = fs::remove_dir_all(dir_path);
    }

    match dir_path.try_exists() {
        Ok(exists) => !exists,
        Err(_) => false,
    }
}
