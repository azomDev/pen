use std::fs;
use std::path::PathBuf;

pub fn delete_env(env_dir_name: &str, temp_dir: &PathBuf) {
    let env_dir = PathBuf::from(".").join(env_dir_name);

    if !env_dir.exists() || !env_dir.is_dir() {
        eprintln!("Virtual environnement in the current directory does not exist");
        return;
    }

    let temp_version_path = temp_dir.join("to_delete");

    if let Err(e) = fs::rename(&env_dir, &temp_version_path) {
        eprintln!(
            "Error 1: Deletion of virual environnement {} failed: {}",
            env_dir.display(),
            e
        );
        return;
    }

    if let Err(e) = fs::remove_dir_all(&temp_version_path) {
        println!(
            "Error 2: Deletion of virual environnement {} failed: {}",
            temp_version_path.display(),
            e
        );
    } else {
        println!(
            "Deletion of virual environnement {} successful",
            env_dir.display()
        );
    }
}
