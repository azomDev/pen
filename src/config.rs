use semver::Version;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use toml::Table;

pub fn create_config(py_version: Version) -> Config {
    Config {
        python: py_version,
        packages: Table::new(),
    }
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    // TODO: find file in parent folders (new function if possible)
    match fs::read_to_string("pen.toml") {
        Ok(toml) => {
            let config: Config = toml::de::from_str(&toml)?;
            Ok(config)
        }
        Err(_) => Ok(Config {
            python: Version::parse("3.12.1").unwrap(),
            packages: Table::new(),
        }),
    }
}

pub fn write_config(config: Config) -> Result<(), Box<dyn Error>> {
    let toml = toml::ser::to_string_pretty(&config)?;
    fs::write("pen.toml", toml)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub python: Version,
    pub packages: Table,
}
