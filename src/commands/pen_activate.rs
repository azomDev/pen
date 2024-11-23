use crate::constants::ENV_DIR_NAME;
use crate::utils::{abort, get_project_root, read_config};
use std::process;

pub fn pen_activate() {
	let project_path = get_project_root();
	let config = read_config(&project_path);

	let command = format!(
		r#"
            VIRTUAL_ENV="{0}"
            if [ ! -L "$VIRTUAL_ENV/bin/python3" ] || [ ! -f "$(readlink -f "$VIRTUAL_ENV/bin/python3")" ]; then
                echo "python3 not found in $VIRTUAL_ENV/bin"
                exit 1
            fi

            export PATH="$VIRTUAL_ENV/bin:$PATH"
            export PYTHON_VERSION="{1}"

            PROMPT_COMMAND='PS1="($PYTHON_VERSION) [\W]\$ "'
            export PROMPT_COMMAND

            exit() {{
                command exit &> /dev/null
            }}
            deactivate() {{
                command exit &> /dev/null
            }}
            export -f exit
            export -f deactivate

            $SHELL
        "#,
		project_path.join(ENV_DIR_NAME).to_string_lossy(), // todo .display() instead???
		config.python
	);

	// todo make it work with plain sh
	match process::Command::new("bash").arg("-c").arg(command).spawn() {
		Ok(mut child) => child.wait().expect("Child process wasn't running."),
		Err(e) => abort("Failed to start shell.", Some(&e)),
	};
}
