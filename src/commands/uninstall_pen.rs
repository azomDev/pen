use crate::{
    constants::{PEN_BIN_FILE, PEN_CONFIG_FILE, PEN_DIR},
    utils::{self, abort},
};
use std::{fs, process};

pub fn uninstall() {
    if !utils::confirm_action("Are you sure you want to uninstall pen? (y/N)") {
        println!("Deletion canceled.");
        process::exit(0);
    }

    println!("Uninstalling pen...");

    let paths_to_check = vec![
        (&*PEN_DIR, true),          // true means it's a directory
        (&*PEN_BIN_FILE, false),    // false means it's a file
        (&*PEN_CONFIG_FILE, false), // false means it's a file
    ];

    let mut existing_paths = Vec::new();

    // Check if paths exist
    for (path, is_dir) in paths_to_check {
        match fs::exists(path) {
            Ok(true) => existing_paths.push((path, is_dir)),
            Ok(false) => (),
            Err(e) => abort(&format!("Error checking path {}:", path.display()), Some(e)),
        }
    }

    // POINT OF NO RETURN

    let mut catastrophic_failure_messages = Vec::new();

    // Attempt to remove the paths
    for (path, is_dir) in existing_paths {
        if is_dir {
            if let Err(e) = fs::remove_dir_all(path) {
                let message = format!(
                    "Failed to remove directory '{}'. Please manually delete it to finish uninstalling.",
                    path.display()
                );
                catastrophic_failure_messages.push((message, e));
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                let message = format!(
                    "Failed to remove file '{}'. Please manually delete it to finish uninstalling.",
                    path.display()
                );
                catastrophic_failure_messages.push((message, e));
            }
        }
    }

    for (message, e) in catastrophic_failure_messages {
        const RED_BOLD: &str = "\x1b[1;31m"; // Bold red text
        const RESET: &str = "\x1b[0m"; // Reset formatting
        eprintln!(
            "{}Catastrophic failure: {}: {}{}",
            RED_BOLD, message, e, RESET
        );
    }

    println!("\x1b[32mUninstall complete.\x1b[0m");
}
