use crate::{utils::{self, abort, catastrophic_failure}, ENV_DIR_NAME};
use std::{process, env};

pub fn delete_env() {
    if !utils::confirm_action("Are you sure you want to delete the virtual environment? (y/N)") {
        println!("Deletion canceled");
        process::exit(0);
    }

    // let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);
    let env_dir = match env::current_dir() {
        Ok(dir) => dir.join(ENV_DIR_NAME),
        Err(e) => abort("todo", Some(e)),
    };

    if !env_dir.exists() || !env_dir.is_dir() {
        abort(&format!("Directory {} does not exist in current directory", env_dir.display()), None);
    }

    println!("Deleting the virtual environment in the current directory");

    if let Err(e) = utils::try_deleting_dir(&env_dir) {
        catastrophic_failure(&format!("idk yet todo {}", env_dir.display()), Some(e));
    }

    println!("Deletion of virual environnement {} successful", &env_dir.display());
}
