use crate::constants::{PYTHON_PACKAGES_DIR, PYTHON_VERSIONS_DIR, TMP_DIR};
use crate::utils::{error, guard, AnyError, Package};
use semver::Version;
use std::{env, fs, path::PathBuf};

/// Constructs the path to the directory for a specified Python version without validating the format of the version string.
///
/// # Arguments
/// - `py_version`: A string slice representing the Python version, which has been
///   validated to conform to the expected format (major.minor.patch).
///
/// # Output
/// - A `PathBuf` pointing to the directory associated with the specified Python version.
///
/// # Termination
/// - This function does not terminate.
///
/// # Guarantees
/// - The returned path will be correctly formed if `py_version` is well formatted.
///
/// # Limitations
/// - The function does not validate the contents of the constructed path or its existence.
pub fn get_python_path(version: &Version) -> PathBuf {
	PYTHON_VERSIONS_DIR.join(format!("{}.{}.{}", version.major, version.minor, version.patch))
}

// todo docstring
pub fn get_package_path(package: &Package) -> PathBuf {
	PYTHON_PACKAGES_DIR.join(format!(
		"{}_{}.{}.{}",
		package.name, package.version.major, package.version.minor, package.version.patch
	))
}

// todo docstring
pub fn get_project_root() -> Result<PathBuf, AnyError> {
	let mut dir = guard!(env::current_dir(), "Failed to get current working directory.");

	loop {
		match guard!(fs::exists(dir.join("pen.toml")), "Failed to check if pen.toml exists.") {
			true => return Ok(dir),
			false => {
				if !dir.pop() {
					return error!("Couldn't find a pen.toml filein the current directory or any above.");
				}
			}
		}
	}
}

/// Attempts to delete a specified directory.
///
/// # Arguments
/// - `dir_path`: A `PathBuf` representing the directory to delete.
///
/// # Output
/// - Returns `Ok(())` if the directory was successfully deleted or if it was already empty.
/// - Returns an `Err` if the directory still exists after attempting deletion.
///
/// # Termination
/// - This function does not terminate.
///
/// # Guarantees
/// - If this function returns `Ok(())`, it guarantees that the directory no longer exists.
pub fn try_deleting_dir(dir_path: &PathBuf) -> Result<(), AnyError> {
	let delete_path = TMP_DIR.join("delete_path");
	return try_deleting_dir_to_temp(dir_path, &delete_path);
}

pub fn try_deleting_dir_to_temp(dir_path: &PathBuf, temp_dir: &PathBuf) -> Result<(), AnyError> {
	if let Ok(exists) = dir_path.try_exists() {
		if !exists {
			return Ok(());
		}
	}
	if guard!(temp_dir.try_exists(), "Unable to know if {} exists", temp_dir.display()) {
		guard!(fs::remove_dir_all(&temp_dir), "todo")
	}

	fs::rename(&dir_path, &temp_dir)?;

	if guard!(dir_path.try_exists(), "todo") {
		return error!("Directory still exists");
	}

	return Ok(());
}

/// Clears and recreates the temporary directory.
///
/// # Input
/// - None.
///
/// # Output
/// - None.
///
/// # Termination
/// - If either removal or creation operations fail, the function prints an error message and terminates the process.
pub fn clear_temp() -> Result<(), AnyError> {
	let mut dir_entries = guard!((&*TMP_DIR).read_dir(), "Failed to check contents of directory {}", (*TMP_DIR).display());
	let temp_is_empty = dir_entries.next().is_none();

	if temp_is_empty {
		return Ok(());
	}

	guard!(fs::remove_dir_all(&*TMP_DIR), "Failed to clear directory {}", (*TMP_DIR).display());
	guard!(fs::create_dir(&*TMP_DIR), "Failed to create directory {}", (*TMP_DIR).display());
	return Ok(());
}
