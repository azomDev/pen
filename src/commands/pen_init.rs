use crate::{
    config::{create_config, write_config},
    utils,
};
use std::error::Error;

pub fn init(version: Option<&String>) -> Result<(), Box<dyn Error>> {
    let version = utils::user_string_to_version(version);

    // TODO: Maybe don't nuke the existing config?
    write_config(create_config(version))
}
