use crate::utils::{error, guard, write_config, AnyError, Config};
use semver::Version;
use std::{env, fs};

pub fn env_init(version: Version) -> Result<(), AnyError> {
	let config = Config {
		python: version,
		packages: toml::Table::new(),
	};

	let project_path = guard!(env::current_dir(), "Failed to get current directory");

	let project_path_exists = guard!(
		fs::exists(project_path.join("pen.toml")),
		"Couldn't see if an existing pen.toml file exists. Do you have permission?"
	);

	if !project_path_exists {
		return error!("This would override an existing pen.toml config file.");
	}
	write_config(project_path, config);
	return Ok(());
}
