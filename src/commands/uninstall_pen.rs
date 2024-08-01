use std::fs;

use crate::PEN_DIR;

pub fn uninstall() {
    println!("Uninstalling pen...");

    // Remove .pen directory if it exists
    if PEN_DIR.exists() {
        if let Err(e) = fs::remove_dir_all(&*PEN_DIR) {
            eprintln!("Failed to remove directory {}: {}", PEN_DIR.display(), e);
            return;
        }
        println!("Removed directory: {}", PEN_DIR.display());
    } else {
        println!("Directory {} does not exist.", PEN_DIR.display());
    }

    // Remove symlink if it exists
    // todo just run what I made in the test shell script where it scans in all possible config files

    println!("Uninstallation complete.");
}