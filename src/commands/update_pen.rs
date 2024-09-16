use crate::utils::abort;
use crate::{utils, TMP_DIR, UPDATE_SCRIPT_URL};
use std::os::unix::fs::PermissionsExt;
use std::{fs, path::PathBuf, process};

pub fn update_pen() {
    println!("Updating pen...");

    let temp_update_script_path = TMP_DIR.join("update_script");
    utils::download_file(UPDATE_SCRIPT_URL, &temp_update_script_path);
    run_update_script(&temp_update_script_path);

    println!("Update successful.");
}

fn run_update_script(file_path: &PathBuf) {
    // Check if the file exists and is a regular file
    let metadata = match fs::metadata(file_path) {
        Ok(metadata) => metadata,
        Err(e) => abort(&format!("Failed to get metadata from file {}", file_path.display()), Some(e))
    };

    // Set the file permissions to make it executable
    let mut permissions = metadata.permissions();
    // todo double check that this is the permission we need
    permissions.set_mode(0o755); // rwxr-xr-x

    if let Err(e) = fs::set_permissions(file_path, permissions) {
        abort("Failed to set file permissions", Some(e));
    }

    // Try to run the script with /bin/sh
    let status = process::Command::new("bin/sh")
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg(file_path)
        .status();

    // Check if the command executed successfully
    match status {
        Ok(status) if status.success() => (),
        Ok(_) => abort("Script execution failed with non-zero exit code", None),
        Err(e) => abort("Failed to execute script", Some(e))
    }
}
