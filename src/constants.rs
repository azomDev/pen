#![allow(dead_code)]

use std::{path::PathBuf, sync::LazyLock};
use home;
use crate::utils::abort;

pub static ENV_DIR_NAME: &str = ".venv";
// pub static UPDATE_SCRIPT_URL: &str = "todo";

pub static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    match home::home_dir() {
        Some(dir) => dir,
        None => abort("Failed to get home directory", None),
    }
});

pub static PEN_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(".pen"));
pub static TMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("temp"));
pub static PYTHON_VERSIONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("python_versions"));
