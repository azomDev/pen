use crate::{
    config::read_config,
    constants::PYTHON_PACKAGES_DIR,
    utils::{self, abort},
};
use std::{env::current_dir, error::Error, fs, io, os::unix};

use super::add_packages;

pub fn install() -> Result<(), Box<dyn Error>> {
    let working_directory = current_dir().expect("Impossible to get current working directory");
    let config = read_config()?; // TODO: Handle no config

    let py_dir = utils::get_version_path(&config.python);

    let py_version_maj_min = format!("{}.{}", config.python.major, config.python.minor);

    fs::create_dir_all(".venv")?;
    fs::write(
        ".venv/pyvenv.cfg",
        format!(
            r#"home = {0}/bin
include-system-site-packages = false
version = {1}
executable = {0}/bin/python
command = {0}/bin/python -m venv {2}/.venv
"#,
            py_dir.to_string_lossy(),
            config.python,
            working_directory.to_string_lossy()
        ),
    )?;

    // Bin
    fs::create_dir_all(".venv/bin")?;
    if let Err(e) = unix::fs::symlink(
        py_dir.join("bin/python"),
        working_directory.join(".venv/bin/python"),
    ) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            abort("Couldn't symlink python", Some(e));
        }
    };
    if let Err(e) = unix::fs::symlink(
        working_directory.join(".venv/bin/python"),
        working_directory.join(".venv/bin/python3"),
    ) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            abort("Couldn't symlink python", Some(e));
        }
    };
    if let Err(e) = unix::fs::symlink(
        working_directory.join(".venv/bin/python"),
        working_directory.join(format!(".venv/bin/python{}", py_version_maj_min)),
    ) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            abort("Couldn't symlink python", Some(e));
        }
    };

    // Lib
    let venv_lib_dir = working_directory.join(format!(
        ".venv/lib/python{}/site-packages",
        py_version_maj_min
    ));
    fs::create_dir_all(&venv_lib_dir)?;
    for (name, version) in config.packages {
        let package_name = format!("{}_{}", name, &version.try_into::<String>().unwrap());
        let package_path: std::path::PathBuf = PYTHON_PACKAGES_DIR.join(&package_name);
        match fs::exists(&package_path) {
            Ok(exists) => {
                if !exists {
                    add_packages(&name, Some(&String::from("value")))?;
                }

                match fs::read_dir(&package_path) {
                    Ok(entries) => {
                        for directory_entry in entries {
                            let directory_entry = match directory_entry {
                                Ok(entry) => entry,
                                Err(e) => abort("Failed to read directory entry", Some(e)),
                            };

                            let entry_metadata = match directory_entry.metadata() {
                                Ok(metadata) => metadata,
                                Err(e) => abort("Failed to read metadata", Some(e)),
                            };

                            if entry_metadata.is_dir() {
                                if let Err(e) = unix::fs::symlink(
                                    directory_entry.path(),
                                    venv_lib_dir.join(directory_entry.file_name()),
                                ) {
                                    if e.kind() != io::ErrorKind::AlreadyExists {
                                        abort(
                                            &format!("Couldn't symlink the package {name}"),
                                            Some(e),
                                        );
                                    }
                                };
                            }
                        }
                    }
                    Err(e) => abort(
                        &format!("Failed to read {}", package_path.display()),
                        Some(e),
                    ),
                }
            }
            Err(e) => {
                abort("Couldn't see if package is installed", Some(e));
            }
        }
    }

    println!("Done!");
    Ok(())
}
