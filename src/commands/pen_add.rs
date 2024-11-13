use crate::utils::{
    config::{find_project, read_config, write_config},
    package::{download_package, find_matching_package_version},
};
use semver::VersionReq;

pub fn add(name: &str, version: &VersionReq) {
    let projet_path = find_project();
    let mut config = read_config(&projet_path);

    let package = find_matching_package_version(name, version);
    download_package(&package);

    config
        .packages
        .insert(package.name, toml::Value::String(version.to_string()));
    write_config(projet_path, config);
}
