use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

use crate::constants::CONFIG_FILE_NAME;
use crate::utils::abort;

// todo docstring
pub fn read_config(project_path: &PathBuf) -> Config {
	let config_path = project_path.join(&*CONFIG_FILE_NAME);
	match fs::read_to_string(&config_path) {
		Ok(contents) => match toml::from_str::<Config>(&contents) {
			Ok(toml) => toml,
			Err(e) => abort(
				&format!("Couldn't parse {}.", config_path.display()),
				Some(&e),
			),
		},
		Err(e) => abort(
			&format!("Couldn't read {}.", config_path.display()),
			Some(&e),
		),
	}
}

// todo docstring
pub fn write_config(project_path: PathBuf, config: Config) {
	match toml::to_string_pretty(&config) {
		Ok(toml) => {
			if let Err(e) = fs::write(project_path.join("pen.toml"), toml) {
				abort(
					&format!(
						"Couldn't write to config file at {}.",
						project_path.display()
					),
					Some(&e),
				);
			}
		}
		Err(e) => abort(
			"Couldn't convert config to valid toml.\nPlease open an issue on Github.",
			Some(&e),
		),
	}
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub python: Version, // todo do we want to have instead a VersionReq?
	pub packages: toml::Table,
}
