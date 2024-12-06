use crate::utils::{self, download_package, error, find_matching_package_version, guard, py_install_algo_v1, AnyError, Config, Package};
use semver::{Version, VersionReq};
use std::{fs, os::unix, path::PathBuf};

pub fn create_or_update_virtual_env(config: Config, destination_path: &PathBuf) -> Result<(), AnyError> {
	let py_dir = utils::get_python_path(&config.python);
	let py_version_short = format!("{}.{}", config.python.major, config.python.minor);

	guard!(fs::create_dir_all(destination_path.join("bin")), "Couldn't create folder");

	let temp = format!(
		// todo this pyenv.cfg file, when writen, has some spacing beforeeach line of the paragraph, needs fixing
		r#" # Created using pen
		home = {0}/bin
		include-system-site-packages = false
		version = {1}
		executable = {0}/bin/python
		command = {0}/bin/python -m venv {2}
		"#,
		py_dir.to_string_lossy(),
		config.python,
		destination_path.to_string_lossy()
	);

	guard!(fs::write(destination_path.join("pyvenv.cfg"), temp), "Couldn't write pyenv.cfg");

	// Bin
	link_python(&config.python, destination_path.join("bin"), &py_version_short);

	// Lib
	let site_packages_path = destination_path.join(format!("lib/python{}/site-packages", py_version_short));
	let _ = fs::remove_dir_all(&site_packages_path);

	guard!(fs::create_dir_all(&site_packages_path), "Couldn't create folder");

	// TODO: split this in another function
	for (name, version) in config.packages {
		let version = version
			.try_into::<String>()
			.ok()
			.map(|v| match VersionReq::parse(&v) {
				Ok(version) => version,
				Err(e) => todo!("Couldn't read version of {} in config", name),
			})
			.unwrap();

		// TODO: use lockfile to cache version
		let package = find_matching_package_version(&name, &version)?;
		link_package(&package, &site_packages_path, &config.python);
	}
	return Ok(());
}

pub fn link_python(version: &Version, destination_path: PathBuf, py_version_short: &String) -> Result<(), AnyError> {
	let python_path = utils::get_python_path(&version);

	let python_path_exists = guard!(fs::exists(&python_path), "Couldn't see if package is installed");
	if !python_path_exists {
		py_install_algo_v1(&version)?;
	}

	symlink(
		python_path.join("bin/python3"), // this is a little cursed since it is dependent on python3 so idk what to do
		destination_path.join("python"),
		Some(true),
	);
	symlink(destination_path.join("python"), destination_path.join("python3"), Some(false));
	symlink(
		destination_path.join("python"),
		destination_path.join(format!("python{}", py_version_short)),
		Some(false),
	);
	return Ok(());
}

pub fn link_package(package: &Package, site_packages_path: &PathBuf, py_version: &Version) -> Result<(), AnyError> {
	let package_path = utils::get_package_path(&package);

	let package_path_exists = guard!(fs::exists(&package_path), "Couldn't see if package is installed");
	if !package_path_exists {
		download_package(&package, py_version);
	}

	let entries = guard!(fs::read_dir(&package_path), "Failed to read {}", package_path.display());
	for directory_entry_result in entries {
		let directory_entry = guard!(directory_entry_result, "Failed to read directory entry");
		symlink(directory_entry.path(), site_packages_path.join(directory_entry.file_name()), None);
	}
	return Ok(());
}

fn symlink(original: PathBuf, link: PathBuf, remove_existing: Option<bool>) -> Result<(), AnyError> {
	match fs::read_link(&link) {
		Ok(_) => match remove_existing {
			Some(true) => {
				guard!(fs::remove_file(&link), "Couldn't remove {}.", &link.display());
			}
			Some(false) => {
				return Ok(()); // We exit the function gracefully and continue
			}
			None => return error!("Symlink already exists"),
		},
		Err(_) => { /* No conflicts! */ }
	};

	guard!(
		unix::fs::symlink(&original, &link),
		"Couldn't symlink {} to {}",
		&original.display(),
		&link.display()
	);
	return Ok(());
}
