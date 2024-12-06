use crate::constants::PYTHON_VERSIONS_DIR;
use crate::utils::{error, guard, AnyError};
use std::fs;

pub fn py_list_versions() -> Result<(), AnyError> {
	println!("Listing installed Python versions:");
	let directory_entries = guard!(fs::read_dir(&*PYTHON_VERSIONS_DIR), "Failed to read {}", PYTHON_VERSIONS_DIR.display());

	let mut installed_versions: Vec<String> = Vec::new();

	for directory_entry in directory_entries {
		let directory_entry = guard!(directory_entry, "Failed to read directory entry");
		let entry_metadata = guard!(directory_entry.file_type(), "Failed to read metadata");

		if entry_metadata.is_dir() {
			match directory_entry.file_name().into_string() {
				Ok(directory_name) => installed_versions.push(directory_name),
				Err(_) => return error!("Failed to convert file name to string"),
			}
		}
	}

	if installed_versions.is_empty() {
		println!("No Python versions installed with pen.");
	} else {
		installed_versions.sort_unstable();
		for version in installed_versions {
			println!("  - {}", version);
		}
	}
	return Ok(());
}
