use crate::{utils, TMP_DIR, UPDATE_SCRIPT_URL};
use std::os::unix::fs::PermissionsExt;
use std::{fs, path::PathBuf, process};

// todo everywhere in the project streamline how the prinln and the eprinln are written

pub fn update_pen() {
    println!("Updating pen...");

    // todo what the below comments says might actually be deletable if curl overwrites the file at the destination
    // Check if there is already a file at this path and delete it if it exists. Exit on failure.
    let temp_update_script_path = TMP_DIR.join("update_script");
    if temp_update_script_path.exists() {
        if fs::remove_file(&temp_update_script_path).is_err() {
            eprintln!("Error: Failed to delete existing update script.");
            process::exit(1);
        }
    }
    utils::download_file(UPDATE_SCRIPT_URL, &temp_update_script_path);
    run_update_script(&temp_update_script_path);

    println!("Update successful.");
}

// check everything in this function
fn run_update_script(file_path: &PathBuf) {
    // Check if the file exists and is a regular file
    let metadata = match fs::metadata(file_path) {
        Ok(metadata) if metadata.is_file() => metadata,
        _ => {
            eprintln!("Error: File does not exist or is not a regular file.");
            process::exit(1);
        }
    };

    // Set the file permissions to make it executable
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755); // rwxr-xr-x

    if fs::set_permissions(file_path, permissions).is_err() {
        eprintln!("Error: Failed to set file permissions.");
        process::exit(1);
    }

    // Try to run the script with /bin/sh
    let status = process::Command::new("/bin/sh").arg(file_path).status();

    // Check if the command executed successfully
    match status {
        Ok(status) if status.success() => (),
        Ok(_) => {
            eprintln!("Error: Script execution failed with non-zero process::exit code.");
            process::exit(1);
        }
        Err(_) => {
            eprintln!("Error: Failed to execute script.");
            process::exit(1);
        }
    }
}
