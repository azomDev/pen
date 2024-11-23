use crate::constants::ENV_DIR_NAME;
use crate::utils::{create_or_update_virtual_env, get_project_root, read_config};

pub fn env_sync() {
	let projet_path = get_project_root();
	let config = read_config(&projet_path);

	create_or_update_virtual_env(config, &projet_path.join(ENV_DIR_NAME));

	println!("Installation complete!");
}
