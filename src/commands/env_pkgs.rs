use crate::env_utils::{find_config, read_config};

pub fn env_pkgs() {
	let project_path = find_config();

	let config = read_config(&project_path);

	println!("Packages listed in pen.toml:");
	for (package_name, _) in &config.packages {
		println!("- {}", package_name);
	}
}
