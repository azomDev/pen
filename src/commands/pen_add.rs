use std::fs;

use crate::utils::{
    abort,
    config::{find_project, read_config, write_config},
    get_package_path,
    package::{download_package, find_matching_package_version},
};
use semver::VersionReq;

pub fn add(name: &str, version: &VersionReq) {
    let projet_path = find_project();
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
