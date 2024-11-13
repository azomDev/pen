use semver::{Version, VersionReq};

use crate::utils::{
    self, abort,
    package::{download_package, find_matching_package_version},
};
use std::{fs, os::unix, path::PathBuf};

use super::config::Config;
use super::package::Package;
use super::python::install_python;

pub fn create_virtual_env(config: Config, destination_path: &PathBuf) {
    let py_dir = utils::get_python_path(&config.python);
    let py_version_short = format!("{}.{}", config.python.major, config.python.minor);

    if let Err(e) = fs::create_dir_all(destination_path.join("bin")) {
        abort("Couldn't create folder.", Some(&e));
    }
    if let Err(e) = fs::write(
        destination_path.join("pyvenv.cfg"),
        format!(
            r#" # Created using pen
                home = {0}/bin
                include-system-site-packages = false
                version = {1}
                executable = {0}/bin/python
                command = {0}/bin/python -m venv {2}
            "#,
            py_dir.to_string_lossy(),
            config.python,
            destination_path.to_string_lossy()
        ),
    ) {
        abort("Couldn't write pyenv.cfg.", Some(&e));
    }

    // Bin
    link_python(
        &config.python,
        destination_path.join("bin"),
        &py_version_short,
    );

    // Lib
    let site_packages_path =
        destination_path.join(format!("lib/python{}/site-packages", py_version_short));
    let _ = fs::remove_dir_all(&site_packages_path);
    if let Err(e) = fs::create_dir_all(&site_packages_path) {
        abort("Couldn't create folder.", Some(&e));
    }

    // TODO: split this in another function
    for (name, version) in config.packages {
        let version = version
            .try_into::<String>()
            .ok()
            .map(|v| match VersionReq::parse(&v) {
                Ok(version) => version,
                Err(e) => abort(
                    &format!("Couldn't read version of {} in config", name),
                    Some(&e),
                ),
            })
            .unwrap();

        // TODO: use lockfile to cache version
        let package = find_matching_package_version(&name, &version);
        link_package(&package, &site_packages_path, &config.python);
    }
}

pub fn link_python(version: &Version, destination_path: PathBuf, py_version_short: &String) {
    let python_path = utils::get_python_path(&version);

    match fs::exists(&python_path) {
        Ok(exists) => {
            if !exists {
                install_python(&version);
            }

            symlink(
                python_path.join("bin/python"),
                destination_path.join("python"),
                Some(true),
            );
            symlink(
                destination_path.join("python"),
                destination_path.join("python3"),
                Some(false),
            );
            symlink(
                destination_path.join("python"),
                destination_path.join(format!("python{}", py_version_short)),
                Some(false),
            );
        }
        Err(e) => {
            abort("Couldn't see if package is installed", Some(&e));
        }
    }
}

pub fn link_package(package: &Package, site_packages_path: &PathBuf, py_version: &Version) {
    let package_path = utils::get_package_path(&package);

    match fs::exists(&package_path) {
        Ok(exists) => {
            if !exists {
                download_package(&package, py_version);
            }

            match fs::read_dir(&package_path) {
                Ok(entries) => {
                    for directory_entry in entries {
                        let directory_entry = match directory_entry {
                            Ok(entry) => entry,
                            Err(e) => abort("Failed to read directory entry", Some(&e)),
                        };

                        symlink(
                            directory_entry.path(),
                            site_packages_path.join(directory_entry.file_name()),
                            None,
                        );
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

fn symlink(original: PathBuf, link: PathBuf, remove_existing: Option<bool>) {
    match fs::read_link(&link) {
        Ok(_) => match remove_existing {
            Some(true) => {
                if let Err(e) = fs::remove_file(&link) {
                    abort(&format!("Couldn't remove {}.", &link.display()), Some(&e));
                }
            }
            Some(false) => {
                return; // We exit the function gracefully and continue
            }
            None => abort("Symlink already exists", None),
        },
        Err(_) => { /* No conflicts! */ }
    };

    if let Err(e) = unix::fs::symlink(&original, &link) {
        abort(
            &format!(
                "Couldn't symlink {} to {}",
                &original.display(),
                &link.display()
            ),
            Some(&e),
        );
    }
}
