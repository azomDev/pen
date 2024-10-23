use std::process;
use crate::ENV_DIR_NAME;

pub fn activate_env() {

    let command = format!(r#"
        VIRTUAL_ENV="{}"
        export PATH="{}/bin:$PATH"
        PYTHON_VERSION="$("$VIRTUAL_ENV/bin/python3" --version | awk '{{print $2}}')"
        PS1="($PYTHON_VERSION) [\W]$ "
        export PS1
        exec bash --norc --noprofile
    "#, ENV_DIR_NAME, ENV_DIR_NAME); // todo instead of bash use default shell

    let mut child = process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Failed to start shell");

    let _ = child.wait().expect("Child process wasn't running");
}
