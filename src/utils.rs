use std::env;
use std::path::Path;
use std::path::PathBuf;
use regex::Regex;

pub fn check_version_format(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+(\.\d+)?$").expect("Invalid regex");
    return re.is_match(version);
}

pub fn get_version_path(pyversion: &str, python_versions_dir: &PathBuf) -> PathBuf {
    let version_dir_name = format!("python_{}", pyversion);
    return python_versions_dir.join(version_dir_name);
}

pub fn does_pen_dir_exists() -> bool{
    let home_dir = env::var("HOME").expect("HOME environment variable is not set");
    let pen_dir = Path::new(&home_dir).join(".pen");
    return pen_dir.exists() && pen_dir.is_dir()
}