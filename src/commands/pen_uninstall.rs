use crate::constants::{PEN_BIN_FILE, PEN_CONFIG_FILE, PEN_DIR};
use crate::utils::{self, guard, AnyError};
use std::fs;

pub fn pen_uninstall() -> Result<(), AnyError> {
	let user_said_yes = guard!(utils::confirm_action("Are you sure you want to uninstall pen? (y/N)"), "todo");
	if !user_said_yes {
		println!("Deletion canceled.");
		return Ok(());
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
		let path_exists = guard!(fs::exists(path), "Error checking path {}:", path.display());
		if path_exists {
			existing_paths.push((path, is_dir))
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
		eprintln!("{}Catastrophic failure: {}: {}{}", RED_BOLD, message, e, RESET);
	}

	println!("\x1b[32mUninstall complete.\x1b[0m");
	return Ok(());
}
