use std::path::Path;
use std::fs;

pub fn delete_env(){
    let env_path = Path::new("./env");
    if let Err(e) = fs::remove_dir_all(&env_path) {
        println!("Deletion of directory {} failed: {}", env_path.display(), e);
    } else {
        println!("Deletion of directory {} successful", env_path.display());
    }
}