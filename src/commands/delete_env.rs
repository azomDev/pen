use crate::{utils::{self, abort}, ENV_DIR_NAME};
use std::{path::PathBuf, process};

pub fn delete_env() {
    if !utils::confirm_action("Are you sure you want to delete the virtual environment? (y/N)") {
        println!("Deletion canceled");
        process::exit(0);
    }

    let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);
    if !env_dir.exists() || !env_dir.is_dir() {
        abort(&format!("Directory {} does not exist in current directory", env_dir.display()), None);
    }

    println!("Deleting the virtual environment in the current directory");

    if let Err(e) = utils::try_deleting_dir(&env_dir) {
        abort(&format!("catastrophic failure or smth {}", env_dir.display()), Some(e));
    }

    println!("Deletion of virual environnement {} successful", &env_dir.display());
}
