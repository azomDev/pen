use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};
use toml::Table;

use crate::utils::abort;

pub fn create_config(py_version: Version) -> Config {
    Config {
        python: py_version,
        packages: Table::new(),
    }
}

pub fn find_project() -> PathBuf {
    let mut dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => abort("Failed to get current wroking directory.", Some(&e)),
    };

    loop {
        match fs::exists(dir.join("pen.toml")) {
            Ok(true) => return dir,
            Ok(false) => {
                if !dir.pop() {
                    abort("Couldn't find a pen.toml file.", None);
                }
            }
            Err(e) => abort("Couldn't find a pen.toml file.", Some(&e)),
        };
    }
}

pub fn read_config(project_path: &PathBuf) -> Config {
    let config_path = project_path.join("pen.toml");
    match fs::read_to_string(&config_path) {
        Ok(contents) => match toml::de::from_str::<Config>(&contents) {
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

pub fn write_config(project_path: PathBuf, config: Config) {
    match toml::ser::to_string_pretty(&config) {
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
    pub python: Version,
    pub packages: Table,
}
