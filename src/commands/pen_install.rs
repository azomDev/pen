use semver::VersionReq;

use crate::constants::{ENV_DIR_NAME, PYTHON_PACKAGES_DIR};
use crate::utils::{
    config::{find_project, read_config},
    virtual_env::create_virtual_env,
};
use std::{fs, os::unix, path::PathBuf};

pub fn install() {
    let projet_path = find_project();
    let config = read_config(&projet_path);

    create_virtual_env(config, &projet_path.join(ENV_DIR_NAME));

    println!("Installation complete!");
}
