use crate::constants::ENV_DIR_NAME;
use crate::env_utils::{create_virtual_env, find_config, read_config};

pub fn env_sync() {
	let projet_path = find_config();
	let config = read_config(&projet_path);

	create_virtual_env(config, &projet_path.join(ENV_DIR_NAME));

	println!("Installation complete!");
}
