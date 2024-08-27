use crate::{utils, ENV_DIR_NAME, TMP_DIR};
use std::{path::PathBuf, process};

pub fn delete_env() {
    if !utils::confirm_action("Are you sure you want to delete the virtual environment? (y/N)") {
        println!("Deletion canceled.");
        process::exit(0);
    }

    let env_dir = PathBuf::from(".").join(ENV_DIR_NAME);
    if !env_dir.exists() || !env_dir.is_dir() {
        eprintln!("Virtual environnement in the current directory does not exist");
        process::exit(1);
    }

    println!("Deleting the virtual environment in the current directory");

    let delete_env_path = TMP_DIR.join("deleted_env_temp");

    if !utils::try_deleting_dir(&env_dir, Some(&delete_env_path)) {
        eprintln!(
            "Deletion of virual environnement {} failed",
            &env_dir.display()
        );
        process::exit(1);
    }

    println!(
        "Deletion of virual environnement {} successful",
        &env_dir.display()
    );
}
