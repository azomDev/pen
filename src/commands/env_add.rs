use crate::{
	env_utils::{
		download_package, find_config, find_matching_package_version, read_config, write_config,
	},
	utils::{abort, get_package_path},
};
use semver::VersionReq;
use std::fs;

pub fn env_add(name: &str, version: &VersionReq) {
	let projet_path = find_config();
	let mut config = read_config(&projet_path);

	let package = find_matching_package_version(name, version);
	let package_path = get_package_path(&package);
	match fs::exists(&package_path) {
		Ok(exists) => {
			if !exists {
				download_package(&package, &config.python);
			}
			// TODO: Link package in project
		}
		Err(e) => {
			abort("Couldn't see if package is installed", Some(&e));
		}
	}

	config
		.packages
		.insert(package.name, toml::Value::String(version.to_string()));
	write_config(projet_path, config);
}
