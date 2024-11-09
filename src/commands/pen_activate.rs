use crate::{
    abort,
    config::{find_project, read_config},
    constants::ENV_DIR_NAME,
};
use std::process;

pub fn activate() {
    let project_path = find_project();
    let config = read_config(&project_path);

    let command = format!(
        r#"
            VIRTUAL_ENV="{0}"
            if [ ! -f "$VIRTUAL_ENV/bin/python3" ]; then
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
        project_path.join(ENV_DIR_NAME).to_string_lossy(),
        config.python
    );

    match process::Command::new("bash").arg("-c").arg(command).spawn() {
        Ok(mut child) => child.wait().expect("Child process wasn't running."),
        Err(e) => abort("Failed to start shell.", Some(&e)),
    };
}
