use crate::{
    config::{find_project, read_config},
    constants::PYTHON_PACKAGES_DIR,
    utils::{self, abort},
};
use std::{fs, io, os::unix, path::PathBuf};

use super::add;

pub fn install() {
    let projet_path = find_project();
    let config = read_config(&projet_path);

    let py_version_maj_min = format!("{}.{}", config.python.major, config.python.minor);
    let py_dir = utils::get_python_path(&config.python);

    if let Err(e) = fs::create_dir_all(projet_path.join(".venv/bin")) {
        abort("Couldn't create folder.", Some(&e));
    }
    if let Err(e) = fs::write(
        projet_path.join(".venv/pyvenv.cfg"),
        format!(
            r#"home = {0}/bin
include-system-site-packages = false
version = {1}
executable = {0}/bin/python
command = {0}/bin/python -m venv {2}/.venv
"#,
            py_dir.to_string_lossy(),
            config.python,
            projet_path.to_string_lossy()
        ),
    ) {
        abort("Couldn't write pyenv.cfg.", Some(&e));
    }

    // Bin
    symlink(
        &py_dir.join("bin/python"),
        &projet_path.join(".venv/bin/python"),
        Some(true),
    );
    symlink(
        &projet_path.join(".venv/bin/python"),
        &projet_path.join(".venv/bin/python3"),
        Some(false),
    );
    symlink(
        &projet_path.join(".venv/bin/python"),
        &projet_path.join(format!(".venv/bin/python{}", py_version_maj_min)),
        Some(false),
    );

    // Lib
    let venv_lib_dir = projet_path.join(format!(
        ".venv/lib/python{}/site-packages",
        py_version_maj_min
    ));
    let _ = fs::remove_dir_all(&venv_lib_dir);
    if let Err(e) = fs::create_dir_all(&venv_lib_dir) {
        abort("Couldn't create folder.", Some(&e));
    }
    for (name, version) in config.packages {
        let package_name = format!("{}_{}", name, &version.try_into::<String>().unwrap());
        let package_path: std::path::PathBuf = PYTHON_PACKAGES_DIR.join(&package_name);
        match fs::exists(&package_path) {
            Ok(exists) => {
                if !exists {
                    add(&name, Some(&String::from("value")));
                }

                match fs::read_dir(&package_path) {
                    Ok(entries) => {
                        for directory_entry in entries {
                            let directory_entry = match directory_entry {
                                Ok(entry) => entry,
                                Err(e) => abort("Failed to read directory entry", Some(&e)),
                            };

                            let entry_metadata = match directory_entry.metadata() {
                                Ok(metadata) => metadata,
                                Err(e) => abort("Failed to read metadata", Some(&e)),
                            };

                            if entry_metadata.is_dir() {
                                if let Err(e) = unix::fs::symlink(
                                    directory_entry.path(),
                                    venv_lib_dir.join(directory_entry.file_name()),
                                ) {
                                    if e.kind() != io::ErrorKind::AlreadyExists {
                                        abort(
                                            &format!("Couldn't symlink the package {name}"),
                                            Some(&e),
                                        );
                                    }
                                };
                            }
                        }
                    }
                    Err(e) => abort(
                        &format!("Failed to read {}", package_path.display()),
                        Some(&e),
                    ),
                }
            }
            Err(e) => {
                abort("Couldn't see if package is installed", Some(&e));
            }
        }
    }

    println!("Installation complete!");
}

fn symlink(original: &PathBuf, link: &PathBuf, remove_existing: Option<bool>) {
    match fs::read_link(link) {
        Ok(_) => match remove_existing {
            Some(true) => {
                if let Err(e) = fs::remove_file(link) {
                    abort(&format!("Couldn't remove {}.", link.display()), Some(&e));
                }
            }
            Some(false) => {
                return; // We exit the function gracefully and continue
            }
            None => abort("Symlink already exists", None),
        },
        Err(_) => { /* No conflicts! */ }
    };

    if let Err(e) = unix::fs::symlink(original, link) {
        abort(
            &format!(
                "Couldn't symlink {} to {}.",
                original.display(),
                link.display()
            ),
            Some(&e),
        );
    }
}