use crate::constants::ENV_DIR_NAME;
use crate::utils::{get_project_root, guard, read_config, AnyError};
use std::process;

pub fn pen_activate() -> Result<(), AnyError> {
	let project_path = guard!(get_project_root(), "todo");
	let config = guard!(read_config(&project_path), "todo");

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
		project_path.join(ENV_DIR_NAME).to_string_lossy(), // todo .display() instead?
		config.python
	);

	// todo make it work with plain sh
	let activate_process = process::Command::new("bash").arg("-c").arg(command).spawn();
	let mut child = guard!(activate_process, "Failed to start shell");
	child.wait().expect("Child process wasn't running.");
	return Ok(());
}
