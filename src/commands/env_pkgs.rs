use crate::utils::{get_project_root, guard, read_config, AnyError};

pub fn env_pkgs() -> Result<(), AnyError> {
	let project_path = guard!(get_project_root(), "todo");

	let config = guard!(read_config(&project_path), "todo");

	println!("Packages listed in pen.toml:");
	for (package_name, _) in &config.packages {
		println!("- {}", package_name);
	}
	return Ok(());
}
