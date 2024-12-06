use semver::Version;

use crate::constants::TMP_DIR;
use crate::utils::{self, error, guard, AnyError};
use std::{fs, path::PathBuf, process};

// todo put a file in the py_utils dir with the job of selecting the file with the function with the appropriate python install algo version. I changed the name of this function but it was a mistake, remove the v1
pub fn py_install_algo_v1(version: &Version) -> Result<(), AnyError> {
	let version_dir = utils::get_python_path(&version);

	if guard!(version_dir.try_exists(), "todo") {
		println!("{} is already installed", version_dir.display());
		return Ok(());
	}

	println!("Installing Python version: {}", &version);

	let temp_tarball_path = TMP_DIR.join("temp_tarball.tgz"); // todo remove hardcoded value
	let python_tarball_url = format!(
		"https://www.python.org/ftp/python/{}/Python-{}.tgz", // todo remove hardcoded value
		&version, &version
	);

	println!("Downloading Python installation files.");
	utils::download_file(&python_tarball_url, &temp_tarball_path);

	println!("Building Python from source.");
	unpack_and_install_python_version_v1(&version, &version_dir, &temp_tarball_path);

	println!("Verifying Python install.");
	let temp_python_version_dir = TMP_DIR.join("temp_python_version_download");
	let temp_python_bin = temp_python_version_dir.join("bin/python3"); // todo remove hardcoded value
	let python_process = process::Command::new(temp_python_bin)
		.stdin(process::Stdio::null())
		.stdout(process::Stdio::null())
		.stderr(process::Stdio::null())
		.arg("--version")
		.status();

	let python_status = guard!(python_process, "Failed to verify if Python version is installed.");

	if !python_status.success() {
		return error!("Failed to verify if Python version is installed.");
	}

	println!("Moving files...");

	if let Err(e1) = fs::rename(&temp_python_version_dir, &version_dir) {
		if let Err(e2) = utils::try_deleting_dir(&version_dir) {
			eprintln!("todo catastrophic message with both e1 and e2");
		} else {
			return error!("Failed to move Python version {}", version);
		}
	}

	println!("Python version {} installed successfully.", &version);
	return Ok(());
}

fn unpack_and_install_python_version_v1(py_version: &Version, py_version_dir: &PathBuf, temp_tarball_path: &PathBuf) -> Result<(), AnyError> {
	// Preparing directories
	let temp_extract_path_dir = TMP_DIR.join("temp_python_extract");
	let temp_python_version_dir = TMP_DIR.join("temp_python_version_download");
	guard!(
		utils::try_deleting_dir(&temp_python_version_dir),
		"Failed to delete {}",
		temp_python_version_dir.display()
	);
	guard!(
		utils::try_deleting_dir(&temp_extract_path_dir),
		"Failed to delete {}",
		temp_extract_path_dir.display()
	);
	guard!(fs::create_dir(&temp_extract_path_dir), "Failed to create temp extract directory");

	println!("Extracting tarball...");

	let tar_process = process::Command::new("tar")
		.stdin(process::Stdio::null())
		.stdout(process::Stdio::null())
		.stderr(process::Stdio::null())
		.arg("-xzf")
		.arg(&temp_tarball_path)
		.arg("-C")
		.arg(&temp_extract_path_dir)
		.status();

	let tar_status = guard!(tar_process, "Failed to extract Python version {}", py_version);
	if !tar_status.success() {
		return error!("Failed to extract Python version {}", py_version);
	}

	println!("Configuring Python...");

	let source_name = format!("Python-{}", py_version);
	let source_dir = temp_extract_path_dir.join(PathBuf::from(source_name));

	let configure_process = process::Command::new("./configure")
		.stdin(process::Stdio::null())
		.stdout(process::Stdio::null())
		.stderr(process::Stdio::null())
		.current_dir(&source_dir)
		.arg(format!("--prefix={}", temp_python_version_dir.display()))
		.status();

	let configure_status = guard!(configure_process, "Failed to configure Python version {}", py_version);
	if !configure_status.success() {
		return error!("Failed to configure Python version {}", py_version);
	}

	println!("Compiling (this might take a few minutes)...");

	let make_process1 = process::Command::new("make")
		.stdin(process::Stdio::null())
		.stdout(process::Stdio::null())
		.stderr(process::Stdio::null())
		.current_dir(&source_dir)
		.status();

	let make_status = guard!(make_process1, "Failed to make Python version {}", py_version);
	if !make_status.success() {
		return error!("Failed to make Python version {}", py_version);
	}

	println!("Finishing Build...");

	let make_process2 = process::Command::new("make")
		.stdin(process::Stdio::null())
		.stdout(process::Stdio::null())
		.stderr(process::Stdio::null())
		.current_dir(&source_dir)
		.arg("install")
		.status();
	let make_status = guard!(make_process2, "Failed to install Python version {}", py_version);
	if !make_status.success() {
		return error!("Failed to install Python version {}", py_version);
	}

	return Ok(());
}
