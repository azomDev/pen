use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

use crate::constants::CONFIG_FILE_NAME;
use crate::utils::{guard, AnyError};

// todo docstring
pub fn read_config(project_path: &PathBuf) -> Result<Config, AnyError> {
	let config_path = project_path.join(&*CONFIG_FILE_NAME);
	let contents = guard!(fs::read_to_string(&config_path), "Couldn't read {}.", config_path.display());
	let toml = guard!(toml::from_str::<Config>(&contents), "Couldn't parse {}.", config_path.display());
	return Ok(toml);
}

// todo docstring
pub fn write_config(project_path: PathBuf, config: Config) -> Result<(), AnyError> {
	let toml = guard!(
		toml::to_string_pretty(&config),
		"Couldn't convert config to valid toml.\nPlease open an issue on Github."
	);
	guard!(
		fs::write(project_path.join("pen.toml"), toml),
		"Couldn't write to config file at {}.",
		project_path.display()
	);
	return Ok(());
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub python: Version, // todo do we want to have instead a VersionReq?
	pub packages: toml::Table,
}
