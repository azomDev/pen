use crate::utils::{self, error, guard, AnyError};

pub fn py_delete_version(py_version: &String) -> Result<(), AnyError> {
	let py_version = guard!(utils::user_string_to_version(py_version), "todo");
	let py_version_dir = utils::get_python_path(&py_version);

	if !py_version_dir.exists() || !py_version_dir.is_dir() {
		return error!("Error: The Python version {} is not installed.", &py_version);
	}

	let prompt = format!("Are you sure you want to remove the Python version {} from pen? (y/N)", &py_version);
	let user_said_yes = guard!(utils::confirm_action(&prompt), "todo");
	if !user_said_yes {
		println!("Removing canceled");
		return Ok(());
	}

	println!("Deleting Python version {}", &py_version);

	guard!(utils::try_deleting_dir(&py_version_dir), "todo");

	println!("Deletion of Python version {} successful", py_version);
	return Ok(());
}
