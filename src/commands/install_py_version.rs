use crate::{py_install_algorithms, utils::{self, abort}, TMP_DIR};
use std::process;

// todo add docs here since it is used by create_env.rs
pub fn install_py_version(py_version: &str) {
    utils::assert_major_minor_patch(&py_version);

    let py_version_dir = utils::get_version_path(&py_version);

    match py_version_dir.try_exists() {
        Ok(true) => {
            println!("{} already exists", py_version_dir.display());
            return;
        },
        Ok(false) => {},
        Err(e) => {
            abort(&format!("Failed to check if {} already exists", py_version_dir.display()), Some(e));
        }
    }

    println!("Installing Python version: {}", &py_version);

    let temp_tarball_path = TMP_DIR.join("temp_tarball.tgz");
    let python_tarball_url = format!("https://www.python.org/ftp/python/{}/Python-{}.tgz", &py_version, &py_version);

    println!("Downloading Python installation files.");
    utils::download_file(&python_tarball_url, &temp_tarball_path);

    println!("Building Python from source.");
    py_install_algorithms::unpack_and_install_python_version_v1(
        &py_version,
        &py_version_dir,
        &temp_tarball_path,
    );

    println!("Verifying Python install.");
    let python_bin = py_version_dir.join("bin/python3");
    match process::Command::new(python_bin)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .arg("--version")
        .status()
    {
        Ok(status) if status.success() => {
            println!("Python version {} installed successfully.", &py_version);
            return;
        },
        Ok(_) => {},
        Err(e) => eprintln!("Error: Failed to verify if Python version is installed: {}", e)
    }
    // todo if it fails, we should try deleting what has been done. If that works nice, if it does not then call catastrophic_failure
    abort("Failed to install python version", None);
}
