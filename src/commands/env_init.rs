use semver::Version;

use crate::utils::{
    abort,
    config::{create_config, write_config},
};
use std::{env, fs};

pub fn env_init(version: Version) {
    let config = create_config(version);

    let project_path = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => abort("Failed to get current directory.", Some(&e)),
    };

    match fs::exists(project_path.join("pen.toml")) {
        Ok(false) => write_config(project_path, config),
        Ok(true) => abort(
            "This would override an existing pen.toml config file.",
            None,
        ),
        Err(e) => abort(
            "Couldn't see if an existing pen.toml file exists. Do you have permission?",
            Some(&e),
        ),
    }
}
