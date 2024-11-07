use std::{process, env};
use crate::abort;
use crate::constants::ENV_DIR_NAME;


pub fn activate_env() {
    let env_dir = match env::current_dir() {
        Ok(dir) => dir.join(ENV_DIR_NAME),
        Err(e) => abort("Failed to get current directory.", Some(e)),
    };

    if !env_dir.exists() {
        abort(
            &format!(
                "{} does not exit or cannot be verified to exist.",
                env_dir.display()
            ),
            None,
        );
    }

    let command = format!(
        r#"
            VIRTUAL_ENV="{}"
            if [ ! -f "$VIRTUAL_ENV/bin/python3" ]; then
                echo "python3 not found in $VIRTUAL_ENV/bin"
                exit 1
            fi

            export PATH="$VIRTUAL_ENV/bin:$PATH"
            export PYTHON_VERSION="$("$VIRTUAL_ENV/bin/python3" --version | awk '{{print $2}}')"

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
        env_dir.to_string_lossy()
    );

    let mut child = process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to start shell");

    let _ = child.wait().expect("Child process wasn't running");
}
