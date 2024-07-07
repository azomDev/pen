use regex::Regex;
use std::path::PathBuf;

pub fn check_version_format(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+(\.\d+)?$").expect("Invalid regex");
    return re.is_match(version);
}

pub fn get_version_path(pyversion: &str, python_versions_dir: &PathBuf) -> PathBuf {
    let version_dir_name = format!("python_{}", pyversion);
    return python_versions_dir.join(version_dir_name);
}
