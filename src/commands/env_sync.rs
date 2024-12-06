use std::error::Error;
use std::fs;
use std::os::unix;

use crate::constants::ENV_DIR_NAME;
use crate::utils::{self, get_project_root, guard, read_config, AnyError};

// todo in all of this file remove hardcoded strings so that they can be the same in the entire program

pub fn env_sync() -> Result<(), AnyError> {
	let projet_path = get_project_root()?;
	let config = read_config(&projet_path)?;
	let venv_path = projet_path.join(ENV_DIR_NAME);

	// missing compared to actual .venv: lib64, share, include
	// no need to implement them now, but it might be useful to know if something happens
	let dirs_to_check = vec![venv_path.join("bin"), venv_path.join("lib").join("python3.12").join("site-packages")];

	for dir_path in dirs_to_check {
		utils::create_dir_if_missing(&dir_path, true)?
	}

	let python_path = utils::get_python_path(&config.python);

	let mut symlinks_to_check = vec![
		(venv_path.join("bin").join("python"), python_path.join("bin").join("python3")),
		(venv_path.join("bin").join("python3"), venv_path.join("bin").join("python")),
		(venv_path.join("bin").join("python3.12"), venv_path.join("bin").join("python")),
	];

	// todo check if the python version is the same as in the config (because the user can manually edit it, like the packages in the config)

	let dependencies = utils::get_recursive_dependencies(&config);

	for dep in dependencies {
		let dep_path = utils::get_package_path(&dep);

		let dep_dir_name = dep_path.file_name().unwrap(); // we can unwrap since get_package_path always add a name at the end

		symlinks_to_check.push((
			venv_path.join("lib").join("python3.12").join("site-packages").join(dep_dir_name),
			dep_path,
		));

		utils::download_dep_if_missing(&dep);
	}

	// todo the logic here is good, only needs refactoring so it is easier to understand
	for (symlink_path, expected_target) in symlinks_to_check {
		let symlink_path_exists = guard!(symlink_path.try_exists(), "todo");
		if symlink_path_exists {
			if symlink_path.is_symlink() {
				let target = guard!(fs::read_link(&symlink_path), "todo");
				if target != expected_target {
					guard!(fs::remove_file(&symlink_path), "todo");

					guard!(
						unix::fs::symlink(&expected_target, &symlink_path),
						"Couldn't symlink {} to {}",
						&expected_target.display(),
						&symlink_path.display()
					)
				}
			} else {
				let path_to_delete = &symlink_path;
				let metadata = guard!(fs::symlink_metadata(path_to_delete), "todo");
				if metadata.is_dir() {
					guard!(fs::remove_dir_all(path_to_delete), "todo");
				} else {
					guard!(fs::remove_file(path_to_delete), "todo");
				}

				guard!(
					unix::fs::symlink(&expected_target, &symlink_path),
					"Couldn't symlink {} to {}",
					expected_target.display(),
					symlink_path.display()
				);
			}
		} else {
			// this fails when symlink_path already exists
			guard!(
				unix::fs::symlink(&expected_target, &symlink_path),
				"Couldn't symlink {} to {}",
				&expected_target.display(),
				&symlink_path.display()
			);
		}
	}

	let pyvenv_text = format!(
		"# Created using pen\n\
			home = {0}/bin\n\
			include-system-site-packages = false\n\
			version = {1}\n\
			executable = {0}/bin/python\n\
			command = {0}/bin/python -m venv {2}\n\
		",
		python_path.to_string_lossy(),
		config.python,
		venv_path.to_string_lossy()
	);

	guard!(fs::write(venv_path.join("pyvenv.cfg"), pyvenv_text), "Couldn't write pyvenv.cfg.");

	println!("Installation complete!");
	return Ok(());
}
