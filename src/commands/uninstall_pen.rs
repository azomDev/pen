use crate::{utils, HOME_DIR, PEN_DIR};
use std::{fs, path::PathBuf, process};

pub fn uninstall() {
    if !utils::confirm_action("Are you sure you want to uninstall pen? (y/N)") {
        println!("Deletion canceled.");
        process::exit(0);
    }

    println!("Uninstalling pen...");

    let tmp_dir = PathBuf::from("/tmp/pen_backup");

    if !utils::try_deleting_dir(&tmp_dir, None) {
        eprintln!("Error deleting {}, aborting", tmp_dir.display());
        process::exit(1);
    }

    let config_files = [HOME_DIR.join(".bashrc"), HOME_DIR.join(".zshrc")];

    let valid_config_files = get_valid_config_files(&config_files);

    remove_alias_from_all_config_files(valid_config_files);

    if let Err(e) = fs::rename(&*PEN_DIR, &tmp_dir) {
        if fs::remove_dir_all(&*PEN_DIR).is_err() {
            eprintln!(
                "Catastrophic failure: Unable to delete {}. Manual cleanup required. Error: {}",
                PEN_DIR.display(),
                e
            );
            process::exit(1);
        }
    }

    // Try to remove the original PEN_DIR location, ignore any error
    let _ = fs::remove_dir_all(&tmp_dir);

    println!("Uninstallation complete.");
}

fn get_valid_config_files(config_files: &[PathBuf]) -> Vec<&PathBuf> {
    let mut valid_files: Vec<&PathBuf> = Vec::new();

    for file_path in config_files {
        if let Ok(exists) = file_path.try_exists() {
            if !exists {
                continue;
            }
        } else {
            eprintln!("Unable to know if {} exists, aborting", file_path.display());
            process::exit(1);
        }

        match file_path.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    continue;
                }

                let permissions = metadata.permissions();
                if permissions.readonly() {
                    eprintln!(
                        "Unable to get permissions of {}, aborting",
                        file_path.display()
                    );
                    process::exit(1);
                }

                valid_files.push(file_path);
            }
            Err(e) => {
                eprintln!(
                    "Unable to get metadata of {}, aborting",
                    file_path.display()
                );
                process::exit(1);
            }
        }
    }

    return valid_files;
}

fn remove_alias_from_all_config_files(config_files: Vec<&PathBuf>) {
    let mut at_least_one_file_edited = false;

    for config_file in config_files {
        let content = match fs::read_to_string(&config_file) {
            Ok(file_string) => file_string,
            Err(e) => {
                if at_least_one_file_edited {
                    eprintln!(
                        "Catastrophic failure: Unable to read {}. Manual cleanup required. Error: {}",
                        config_file.display(),
                        e
                    );
                    continue;
                } else {
                    eprintln!("Unable to read {}, aborting", config_file.display());
                    process::exit(1);
                }
            }
        };

        // todo check this paragraph to be sure it does what it is supposed to do
        // Remove the specific line
        let new_content: String = content
            .lines()
            .filter(|&line| line.trim() != "alias pen=\". $HOME/.pen/main.sh\"")
            .collect::<Vec<&str>>()
            .join("\n");

        // if data did not change, no need to try writing the data
        if new_content == content {
            continue;
        }

        // Write the new content back to the file
        match fs::write(&config_file, new_content) {
            Ok(()) => {
                at_least_one_file_edited = true;
            }
            Err(e) => {
                if at_least_one_file_edited {
                    eprintln!(
                        "Catastrophic failure: Unable to write to {}. Manual cleanup required. Error: {}",
                        config_file.display(),
                        e
                    );
                    continue;
                } else {
                    // todo check if the data has been altered partially. If yes, notify the user
                    eprint!(
                        "Unable to write new data to {}, aborting",
                        config_file.display()
                    );
                    process::exit(1);
                }
            }
        }
    }
}
