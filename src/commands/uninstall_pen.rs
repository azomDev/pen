use crate::{utils::{self, abort, catastrophic_failure}, HOME_DIR, PEN_DIR};
use std::{fs, io, path::{Path, PathBuf}, process};

// todo check all this file

pub fn uninstall() {
    if !utils::confirm_action("Are you sure you want to uninstall pen? (y/N)") {
        println!("Deletion canceled.");
        process::exit(0);
    }

    println!("Uninstalling pen...");

    // let tmp_dir = PathBuf::from("/tmp/pen_backup");

    // if let Err(e) = utils::try_deleting_dir(&tmp_dir) {
    //     abort(&format!("Error deleting {}", tmp_dir.display()), Some(e))
    // }

    let config_files = [HOME_DIR.join(".bashrc"), HOME_DIR.join(".zshrc")];

    let valid_config_files = get_existing_config_files(&config_files);

    // todo check permissions and mabye other stuff to reduce the chance of "try_deleting_dir_to_temp" failing

    remove_alias_from_config_files(&valid_config_files);

    // if let Err(e1) = utils::try_deleting_dir_to_temp(&*PEN_DIR, &tmp_dir) {
    //     // if delete using rename fails, just try to directly delete the directory
    //     if let Err(e2) = fs::remove_dir_all(&*PEN_DIR) {
    //         if e2.kind() != io::ErrorKind::NotFound {
    //             catastrophic_failure(&format!("Was not blablabla, initial error was {} but failed to recover", e1), Some(e2));
    //         }
    //     }
    // }

    // // Try to remove the temp directory, ignore any error
    // let _ = fs::remove_dir_all(&tmp_dir);

    println!("Last step to uninstall pen is to delete the directory at the path {}", PEN_DIR.display());
}

fn get_existing_config_files(config_files: &[PathBuf]) -> Vec<(PathBuf, PathBuf)> {
    let mut existing_files = Vec::new();

    for file_path in config_files {
        // todo this first match is probably not needed because metadata fetch returns Err if the path does not exist
        match file_path.try_exists() {
            Ok(true) => (),
            Ok(false) => continue,
            Err(e) => abort(&format!("Unable to know if {} exists", file_path.display()), Some(e))
        }

        match file_path.metadata() {
            Ok(metadata) if metadata.is_file() => {
                let permissions = metadata.permissions();
                if permissions.readonly() {
                    abort(&format!("{} is readonly", file_path.display()), None);
                }
                if let Some(temp_path) = get_temp_edited_config(file_path) {
                    let tuple = (file_path.clone(), temp_path);
                    existing_files.push(tuple);
                } else {
                    continue; // if temp_path is None, it means that there is no change to do with that file
                }
            },
            Ok(_) => continue, // continue if path is a directory
            Err(e) => {
                abort(&format!("Unable to get metadata of {}", file_path.display()), Some(e));
            }
        }
    }

    return existing_files;
}

fn get_temp_edited_config(config_file: &PathBuf) -> Option<PathBuf> {
    let content = match fs::read_to_string(&config_file) {
        Ok(file_string) => file_string,
        Err(e) =>  abort(&format!("Failed to read {}", config_file.display()), Some(e))
    };

    // Remove the specific line
    let mut new_content = String::new();
    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line != "alias pen=\". $HOME/.pen/main.sh\"" {
            if !new_content.is_empty() {
                new_content.push('\n');
            }
            new_content.push_str(line);
        }
    }

    // if data did not change, no need to try changing the data
    if new_content == content {
        return None;
    }

    // todo here, it is trying to use /tmp, which we know is not a good idea because possible separate filesystem
    // since the config things are happening before the deleting of .pen, we can use the temp dir in .pen here.
    // we don't need to delete the file if it already exists as fs::write will overwrite all contents if it does exist.
    let temp_file = match config_file.file_name() {
        Some(file_name) => PathBuf::from("/tmp").join(file_name),
        None => abort(&format!("{} has no file name", config_file.display()), None)
    };

    // Write the new content to a temp file
    match fs::write(&temp_file, &new_content) {
        Ok(()) => return Some(temp_file),
        Err(e) => abort(&format!("Failed to write contents of {} to {}", config_file.display(), temp_file.display()), Some(e)),
    }
}

fn remove_alias_from_config_files(config_files: &Vec<(PathBuf, PathBuf)>) {
    let mut at_least_one_file_edited = false;

    let mut files_with_fatal_write: Vec<&PathBuf> = Vec::new();

    for (config_file, temp_file) in config_files {
        match fs::rename(temp_file, config_file) {
            Ok(()) => {
                at_least_one_file_edited = true;
                continue;
            },
            Err(e) => {
                if at_least_one_file_edited {
                    files_with_fatal_write.push(config_file);
                    continue;
                } else {
                    // we are assuming that if fs::rename is Err, then nothing changed in the file system
                    abort(&format!("Failed to move {} to {}", temp_file.display(), config_file.display()), Some(e));
                }
            }
        }
    }

    if !files_with_fatal_write.is_empty() {
        // todo custom catastrophic message that says what is wrong, tells what to do and then lists all the files that has to be manually checked
    }
}
