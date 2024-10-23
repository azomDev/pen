use std::process;
use crate::ENV_DIR_NAME;

pub fn activate_env() {
    let command = format!(r#"
        VIRTUAL_ENV="{}"
        export PATH="$VIRTUAL_ENV/bin:$PATH"
        export PYTHON_VERSION="$("$VIRTUAL_ENV/bin/python3" --version | awk '{{print $2}}')"
        exit() {{
            command exit &> /dev/null
        }}
        export -f exit
        PROMPT_COMMAND='PS1="($PYTHON_VERSION) [\W]\$ "'
        export PROMPT_COMMAND
        $SHELL
    "#, ENV_DIR_NAME);

    let mut child = process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to start shell");

    let _ = child.wait().expect("Child process wasn't running");
}
