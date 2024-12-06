use crate::utils::{download_package, find_matching_package_version, get_package_path, get_project_root, guard, read_config, write_config, AnyError};
use semver::VersionReq;
use std::fs;

pub fn env_add(name: &str, version: &VersionReq) -> Result<(), AnyError> {
	let projet_path = get_project_root()?;
	let mut config = read_config(&projet_path)?;

	let package = find_matching_package_version(name, version)?;
	let package_path = get_package_path(&package);
	let package_path_exists = guard!(fs::exists(&package_path), "Couldn't see if package is installed");
	if !package_path_exists {
		download_package(&package, &config.python);
	}

	config.packages.insert(package.name, toml::Value::String(version.to_string()));
	write_config(projet_path, config);
	return Ok(());
}
