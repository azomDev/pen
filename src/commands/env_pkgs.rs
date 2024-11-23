use crate::utils::{get_project_root, read_config};

pub fn env_pkgs() {
	let project_path = get_project_root();

	let config = read_config(&project_path);

	println!("Packages listed in pen.toml:");
	for (package_name, _) in &config.packages {
		println!("- {}", package_name);
	}
}
