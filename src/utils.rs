use crate::{HOME_DIR, PEN_DIR, TMP_DIR, PYTHON_VERSIONS_DIR};
use std::{fs, io::{self, Write}, path::PathBuf, process};


/// Asserts that a given version string adheres to the "major.minor.patch" format.
///
/// # Input
/// - `py_version`: A string slice representing the version number.
///
/// # Output
/// - None.
///
/// # Things to Know
/// - This function ensures that the provided version string is in the correct format.
/// - If the format is invalid, the function will print an error message and terminate the process.
pub fn assert_major_minor_patch(py_version: &str) {
    let parts = py_version.split('.').collect::<Vec<&str>>();

    if parts.len() != 3 {
        abort(&format!("Version {} does not match the major.minor.patch format : Version must have exactly three parts", py_version), None);
    }

    for part in parts {
        if part.parse::<u32>().is_err() {
            abort(&format!("Version {} does not match the major.minor.patch format : Each part must be a valid integer", py_version), None);
        }
    }
}


/// Constructs the path to the directory for a specified Python version.
///
/// # Input
/// - `py_version`: A string slice representing the Python version.
///
/// # Output
/// - Returns a `PathBuf` that points to the directory for the specified version.
///
/// # Things to Know
/// - This function guarantees that the path is correctly formed based on the version provided.
/// - No validation is performed on the contents of the path or its existence.
/// - This function will not fail under normal circumstances.
pub fn get_version_path(py_version: &str) -> PathBuf {
    let py_version_dir_name = format!("python_{}", py_version);
    return PYTHON_VERSIONS_DIR.join(py_version_dir_name);
}


/// Prompts the user to confirm an action and returns their response.
///
/// # Input
/// - `prompt`: A string slice containing the prompt message to display to the user.
///
/// # Output
/// - Returns `true` if the user inputs "y" or "Y"; otherwise, returns `false`.
///
/// # Things to Know
/// - This function may fail due to issues with input/output streams. If such errors occur,
/// the function will print an error message and terminate the process.
pub fn confirm_action(prompt: &str) -> bool {
    println!("{}", prompt);

    // Flush stdout to ensure the prompt appears before reading input
    if let Err(e) = io::stdout().flush() {
        abort("Failed to flush standart output", Some(e));
    }

    // Read user input
    let mut user_input = String::new();
    if let Err(e) = io::stdin().read_line(&mut user_input) {
        abort("Failed to read standart input", Some(e));
    }

    return user_input.trim().eq_ignore_ascii_case("y");
}


/// Downloads a file from a specified URL to a given file path.
///
/// # Input
/// - `file_url`: A string slice representing the URL of the file to download.
/// - `file_path`: A `PathBuf` specifying where to save the downloaded file.
///
/// # Output
/// - None.
///
/// # Things to Know
/// - If a file already exists at the specified path, it will be deleted before the new file is downloaded.
/// - This function will terminate the process if the download fails or if the file does not exist after the download.
/// - Although the function ensures the file is downloaded, it does not verify the contents of the file.
pub fn download_file(file_url: &str, file_path: &PathBuf) {
    if let Err(e) = fs::remove_file(file_path) {
        if e.kind() != io::ErrorKind::NotFound {
            abort("Unable to remove file", Some(e));
        }
    }

    match process::Command::new("curl")
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
    {
        Ok(status) if status.success() => (),
        Ok(_) => abort(&format!("Failed to download file from {} to {}", file_url, file_path.display()), None),
        Err(e) => abort(&format!("Failed to extract Python version {} to {}", file_url, file_path.display()), Some(e)),
    }

    if !file_path.exists() || !file_path.is_file() {
        abort("Downloaded file was not found", None);
    }
}


/// Attempts to delete a specified directory.
///
/// # Input
/// - `dir_path`: A `PathBuf` representing the directory to delete.
///
/// # Output
/// - Returns `Ok(())` if the directory was successfully deleted or if it was already empty.
/// - Returns an `Err` if the directory still exists after attempting deletion.
///
/// # Things to Know
/// - If the directory is empty, the function will still return `Ok(())`.
/// - If this function returns `Ok(())`, you can be certain that the directory no longer exists at the specified path.
/// - This function is designed to handle errors and will not fail under normal circumstances.
pub fn try_deleting_dir(dir_path: &PathBuf) -> Result<(), std::io::Error> {
    let delete_path = TMP_DIR.join("delete_path");
    return try_deleting_dir_to_temp(dir_path, &delete_path);
}

pub fn try_deleting_dir_to_temp(dir_path: &PathBuf, temp_dir: &PathBuf) -> Result<(), std::io::Error> {
    if let Ok(exists) = dir_path.try_exists() {
        if !exists {
            return Ok(());
        }
    }
    fs::remove_dir_all(&temp_dir)?;
    fs::rename(&dir_path, &temp_dir)?;
    if dir_path.try_exists()? {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Directory still exists"
        ))
    } else {
        Ok(())
    }
}


/// Checks if the specified dependencies are installed by running their `--help` command.
///
/// # Input
/// - `dependencies`: A vector of string slices representing the names of the dependencies to check.
///
/// # Output
/// - None.
///
/// # Things to Know
/// - This function verifies if each dependency is installed by executing its `--help` command. If the command succeeds, the dependency is considered installed.
/// - If any dependency is not installed, the function prints an error message and terminates the process.
pub fn assert_dependencies(dependencies: Vec<&'static str>) {
    for dep in dependencies {
        match process::Command::new(dep)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .arg("--help")
            .status()
        {
            Ok(status) if status.success() => continue,
            Ok(_) => abort(&format!("{} is not installed", dep), None),
            Err(e) => abort(&format!("Failed to check if {} is installed", dep), Some(e))
        }
    }
}


/// Prints an error message and terminates the process.
///
/// # Input
/// - `message`: A string slice containing the error message to display.
/// - `e`: An optional `io::Error` that, if provided, will be included in the error message for additional context.
///
/// # Output
/// - This function does not return. It terminates the process with an exit status of 1.
///
/// # Things to Know
/// - If an `io::Error` is provided, it will be appended to the error message for additional context.
/// - The process will exit immediately with status code 1, indicating an error.
/// - This function always exit, it will never return
pub fn abort(message: &str, e: Option<io::Error>) -> ! {
    if let Some(error) = e {
        eprintln!("Error: {}: {}", message, error);
    } else {
        eprintln!("Error: {}", message);
    }
    process::exit(1);
}

/// Prints a critical error message and terminates the process with a status code of 1.
///
/// # Input
/// - `message`: A string slice containing the critical error message to display.
/// - `e`: An optional `io::Error` that, if provided, will be included in the error message for additional context.
///
/// # Output
/// - This function does not return. It terminates the process with an exit status of 1.
///
/// # Things to Know
/// - The error message is prefixed with "Catastrophic failure: " and is highlighted in bold red text to emphasize the severity.
/// - If an `io::Error` is provided, it will be appended to the error message for additional detail.
/// - The process will exit immediately with status code 1, indicating an error.
/// - This function always exits and will never return.
pub fn catastrophic_failure(message: &str, e: Option<io::Error>) -> ! {
    const RED_BOLD: &str = "\x1b[1;31m"; // Bold red text
    const RESET: &str = "\x1b[0m"; // Reset formatting
    if let Some(error) = e {
        eprintln!("{}Catastrophic failure: {}: {}{}", RED_BOLD, message, error, RESET);
    } else {
        eprintln!("{}Catastrophic failure: {}{}", RED_BOLD, message, RESET);
    }
    process::exit(1);
}


/// Clears and recreates the temporary directory.
///
/// # Input
/// - None.
///
/// # Output
/// - None.
///
/// # Things to Know
/// - If the removal fails, it will attempt to create the directory anew to prevent error loops.
/// - If either removal or creation operations fail, the function prints an error message and terminates the process.
pub fn clear_temp() {
    if let Err(e) = fs::remove_dir_all(&*TMP_DIR) {
        let _ = fs::create_dir(&*TMP_DIR); // this is to prevent an error loop if TMP_DIR does not exist
        abort(&format!("Failed to clear directory {}", (*TMP_DIR).display()), Some(e))
    }

    if let Err(e) = fs::create_dir(&*TMP_DIR) {
        abort(&format!("Failed to create directory {}", (*TMP_DIR).display()), Some(e))
    }
}


// todo this function and also the docs for this function
// todo cases where these are files and not actually directories
pub fn assert_global_paths() {
    match HOME_DIR.try_exists() {
        Ok(true) => (),
        Ok(false) => abort(&format!("Failed to check if {} exists", HOME_DIR.display()), None),
        Err(e) => abort(&format!("Failed to check if {} exists", HOME_DIR.display()), Some(e))
    }

    match PEN_DIR.try_exists() {
        Ok(true) => (),
        Ok(false) => abort(&format!("Failed to check if {} exists", PEN_DIR.display()), None),
        Err(e) => abort(&format!("Failed to check if {} exists", PEN_DIR.display()), Some(e))
    }

    match TMP_DIR.try_exists() {
        Ok(true) => (),
        Ok(false) => abort(&format!("Failed to check if {} exists", TMP_DIR.display()), None),
        Err(e) => abort(&format!("Failed to check if {} exists", TMP_DIR.display()), Some(e))
    }

    match PYTHON_VERSIONS_DIR.try_exists() {
        Ok(true) => (),
        Ok(false) => {
            if let Err(e) = fs::create_dir(&*PYTHON_VERSIONS_DIR) {
                abort(&format!("Failed to create directory {}", PYTHON_VERSIONS_DIR.display()), Some(e));
            }
        },
        Err(e) => abort(&format!("Failed to check if {} exists", PYTHON_VERSIONS_DIR.display()), Some(e))
    }
}
