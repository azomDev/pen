use crate::constants::{HOME_DIR, PEN_CONFIG_FILE, PEN_DIR, PYTHON_PACKAGES_DIR, PYTHON_VERSIONS_DIR, TMP_DIR};
use crate::utils::{error, guard, AnyError, Config, Package};
use semver::Version;
use serde_json::Value;
use std::{
	fs,
	io::{self, Write},
	path::PathBuf,
	process,
};

// todo docstring
pub fn user_string_to_version(version: &String) -> Result<Version, AnyError> {
	let version = guard!(Version::parse(version), "Version parsing failed for version {version}");
	return Ok(version);
	// match version {
	// 	Some(version) => {
	// 		let version = guard!(Version::parse(version), "Version parsing failed for version {version}");
	// 		return Ok(version);
	// 	}
	// 	None => {
	// 		todo!("pick the most recent version either from the web or the system wide installation")
	// 	}
	// }
}

// /// Asserts that a given version string adheres to the "major.minor.patch" format.
// ///
// /// # Arguments
// /// - `py_version`: A string slice representing the version number.
// ///
// /// # Output
// /// - None.
// ///
// /// # Termination
// /// - This function terminates if the `py_version` provided is not well formed.
// ///
// /// # Guarantees
// /// - If this function returns, it guarantees that `py_version` adheres to the format.
// ///
// /// # Limitations
// /// - The function does not validate if the given version is a valid Python version
// pub fn assert_major_minor_patch(py_version: &str) -> Result<(), AnyError> {
// 	let parts = py_version.split('.').collect::<Vec<&str>>();

// 	if parts.len() != 3 {
// 		error!();
// 	}

// 	for part in parts {
// 		// debug!("Parsing {part} into u64");
// 		guard!(part.parse::<u64>(), "todo");
// 		// "Version {} does not match the major.minor.patch format : Each part must be a valid integer"
// 	}
// 	return Ok(());
// }

/// Prompts the user to confirm an action.
///
/// # Arguments
/// - `prompt`: A string slice containing the prompt message to display to the user.
///
/// # Output
/// - Returns `true` if the user inputs "y" or "Y"; otherwise, returns `false`.
///
/// # Termination
/// - This function may terminate due to issues with input/output streams.
pub fn confirm_action(prompt: &str) -> Result<bool, AnyError> {
	println!("{}", prompt);

	io::stdout().flush()?;

	let mut user_input = String::new();

	io::stdin().read_line(&mut user_input)?;

	return Ok(user_input.trim().eq_ignore_ascii_case("y"));
}

/// Downloads a file from a specified URL to a given file path. If a file already exists at the specified path, it will be deleted before the new file is downloaded
///
/// # Arguments
/// - `file_url`: A string slice representing the URL of the file to download.
/// - `file_path`: A `PathBuf` specifying where to save the downloaded file.
///
/// # Output
/// - None.
///
/// # Termination
/// - The function will terminate the process if it fails to remove an existing file.
/// - It will also terminate if an error occurs during the download.
/// - Additionally, it will terminate if the downloaded file cannot be found afterward.
///
/// # Guarantees
/// - The function guarantees the downloaded file exists.
///
/// # Limitations
/// - The function does not validate the contents of the downloaded file.
pub fn download_file(file_url: &str, file_path: &PathBuf) -> Result<(), AnyError> {
	match guard!(fs::exists(file_path), "todo") {
		true => guard!(fs::remove_file(file_path), "todo"),
		false => (),
	}

	let response = guard!(minreq::get(file_url).send(), "todo");

	if response.status_code != 200 {
		return error!("todo");
	}
	guard!(fs::write(file_path, response.as_bytes()), "todo");
	return Ok(());
}

/// Takes the major and minor version and returns the full version using https://endoflife.date/api/python.json
///
/// # Arguments
/// - `major_minor_version` : a string representing the "x.y" part of the release
///
/// # Output
/// - Will output the full version from the provided major & minor
///
/// #Termination
/// - An error should be thrown if the file for checking is not found, or if the version is not found.
///
/// # Limitations
/// - The function assumes that all data passed to it is in the correct format
pub fn get_full_python_version(major_minor_version: &str) -> Result<Option<String>, AnyError> {
	let request = minreq::get("https://endoflife.date/api/python.json").with_header("Accept", "application/json");
	let response = guard!(request.send(), "todo");
	if response.status_code != 200 {
		return error!("todo");
	}

	let json = guard!(response.json::<Value>(), "Failed to parse response into json.");

	if let Some(json) = json.as_array() {
		for item in json {
			match item.as_object() {
				Some(items) => {
					for (key, value) in items {
						if (key == "cycle") && (value == major_minor_version) {
							// returns the latest
							return Ok(Some(item["latest"].to_string()));
						}
					}
				}
				None => return error!("todo"),
			}
		}
	}
	return Ok(None);
}

/// Checks if the specified dependencies are installed by running their `--help` command.
///
/// # Arguments
/// - `dependencies`: A vector of string slices representing the names of the dependencies to check.
///
/// # Output
/// - None.
///
/// # Termination
/// -If, for any dependencies, the `--help` command fails, this function terminates.
///
/// # Guarantees
/// - If the function returns, the dependencies are considered installed.
///
/// # Limitations
/// - The function only checks the result of the `--help` command for each dependencies.
pub fn assert_dependencies(dependencies: Vec<&'static str>) -> Result<(), AnyError> {
	for dep in dependencies {
		let command = process::Command::new("sh")
			.arg("-c")
			.arg(format!("command -v {}", dep))
			.stdin(process::Stdio::null())
			.stdout(process::Stdio::null())
			.stderr(process::Stdio::null())
			.status();

		let sucess = guard!(command, "todo");

		if sucess.success() {
			continue;
		} else {
			return error!("{} is not installed", dep);
		}
	}
	return Ok(());
}

/// Checks if the paths used in pen exists.
///
/// # Arguments
/// - None
///
/// # Output
/// - None.
///
/// # Termination
/// -If, for any paths, checking the existence fails or returns false, this function exits. Only exeption is if `PYTHON_VERSIONS_DIR` does not exist, then it it created.
///
/// # Guarantees
/// - If the function returns, the paths are considered to be existing.
pub fn assert_global_paths() -> Result<(), AnyError> {
	let home_dir_exists = HOME_DIR.try_exists()?;

	if home_dir_exists {
		if !HOME_DIR.is_dir() {
			return error!("Error: {} is not a directory.", HOME_DIR.display());
		}
	} else {
		return error!("Error: {} does not exist.", HOME_DIR.display());
	}

	// No need to check for PEN_BIN since it is only used when uninstalling, where it is checked for existence.
	let dirs_to_check = vec![(&*PEN_DIR), (&*PYTHON_PACKAGES_DIR), (&*TMP_DIR), (&*PYTHON_VERSIONS_DIR)];

	for path in dirs_to_check {
		guard!(create_dir_if_missing(path, false), "todo");
	}

	let pen_config_file_exists = guard!(PEN_CONFIG_FILE.try_exists(), "todo");

	if !pen_config_file_exists {
		guard!(fs::File::create_new(&*PEN_CONFIG_FILE), "todo");
	}

	return Ok(());
}

// todo docstring
pub fn create_dir_if_missing(dir_path: &PathBuf, overwrite: bool) -> Result<(), AnyError> {
	let dir_path_exists = guard!(dir_path.try_exists(), "todo");
	if dir_path_exists {
		if dir_path.is_file() && overwrite {
			if overwrite {
				guard!(fs::remove_file(dir_path), "todo");
				guard!(fs::create_dir_all(dir_path), "todo");
			} else {
				return error!("Error: Path is a file, expected directory at {}", dir_path.display());
			}
		}
	} else {
		guard!(fs::create_dir_all(dir_path), "todo")
	}

	return Ok(());
}

pub fn get_recursive_dependencies(config: &Config) -> Vec<Package> {
	todo!()
}

pub fn download_dep_if_missing(dependency: &Package) -> () {
	todo!()
}

pub fn get_info_of_package(package: &Package) {
	// since the json returned from the url is very complicated, it is expected to add new information to return when needed since we do not want to specify a json structure.
}
