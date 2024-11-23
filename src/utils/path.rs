use crate::constants::{PYTHON_PACKAGES_DIR, PYTHON_VERSIONS_DIR, TMP_DIR};
use crate::utils::{abort, Package};
use semver::Version;
use std::{env, fs, io, path::PathBuf};

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
	PYTHON_VERSIONS_DIR.join(format!(
		"{}.{}.{}",
		version.major, version.minor, version.patch
	))
}

// todo docstring
pub fn get_package_path(package: &Package) -> PathBuf {
	PYTHON_PACKAGES_DIR.join(format!(
		"{}_{}.{}.{}",
		package.name, package.version.major, package.version.minor, package.version.patch
	))
}

// todo docstring
pub fn get_project_root() -> PathBuf {
	// todo this should probably be in a more global utils file
	let mut dir = match env::current_dir() {
		Ok(dir) => dir,
		Err(e) => abort("Failed to get current working directory.", Some(&e)),
	};

	loop {
		match fs::exists(dir.join("pen.toml")) {
			Ok(true) => return dir,
			Ok(false) => {
				if !dir.pop() {
					abort("Couldn't find a pen.toml file.", None);
				}
			}
			Err(e) => abort("Failed to find a pen.toml file.", Some(&e)),
		};
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
pub fn try_deleting_dir(dir_path: &PathBuf) -> Result<(), std::io::Error> {
	let delete_path = TMP_DIR.join("delete_path");
	return try_deleting_dir_to_temp(dir_path, &delete_path);
}

pub fn try_deleting_dir_to_temp(
	dir_path: &PathBuf,
	temp_dir: &PathBuf,
) -> Result<(), std::io::Error> {
	if let Ok(exists) = dir_path.try_exists() {
		if !exists {
			return Ok(());
		}
	}
	match temp_dir.try_exists() {
		Ok(true) => fs::remove_dir_all(&temp_dir)?,
		Ok(false) => (),
		Err(e) => abort(
			&format!("Unable to know if {} exists", temp_dir.display()),
			Some(&e),
		),
	}
	fs::rename(&dir_path, &temp_dir)?;
	if dir_path.try_exists()? {
		Err(io::Error::new(
			io::ErrorKind::Other,
			"Directory still exists",
		))
	} else {
		Ok(())
	}
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
pub fn clear_temp() {
	let temp_is_empty = match (&*TMP_DIR).read_dir() {
		Ok(mut read_dir) => read_dir.next().is_none(),
		Err(e) => abort(
			&format!(
				"Failed to check contents of directory {}",
				(*TMP_DIR).display()
			),
			Some(&e),
		),
	};

	if temp_is_empty {
		return;
	}

	if let Err(e) = fs::remove_dir_all(&*TMP_DIR) {
		abort(
			&format!("Failed to clear directory {}", (*TMP_DIR).display()),
			Some(&e),
		)
	}

	if let Err(e) = fs::create_dir(&*TMP_DIR) {
		abort(
			&format!("Failed to create directory {}", (*TMP_DIR).display()),
			Some(&e),
		)
	}
}
