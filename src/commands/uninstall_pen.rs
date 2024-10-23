use crate::{utils::{self, abort, catastrophic_failure}, PEN_DIR};
use std::{fs, process};

pub fn uninstall() {
    if !utils::confirm_action("Are you sure you want to uninstall pen? (y/N)") {
        println!("Deletion canceled.");
        process::exit(0);
    }

    println!("Uninstalling pen...");

    println!("Deleting a symbolic link at /usr/local/bin/pen requires elevated permissions. Please enter your password.");
    match process::Command::new("sudo")
        .arg("rm")
        .arg("/usr/local/bin/pen")
        .status()
    {
        Ok(status) if status.success() => {},
        Ok(_) => abort("Failed to remove the symlink.", None),
        Err(e) => abort("Error executing command", Some(e))
    }

    if let Err(e) = fs::remove_dir_all(&*PEN_DIR) {
        let message = format!("Failed to remove directory '{}'. Please manually delete it to finish uninstalling.", PEN_DIR.display());
        catastrophic_failure(&message, Some(e));
    }

    println!("\x1b[32mUninstall complete.\x1b[0m");
}
