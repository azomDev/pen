#![allow(dead_code)]

use crate::utils::abort;
use home;
use std::{path::PathBuf, sync::LazyLock};

pub static ENV_DIR_NAME: &str = ".venv";
// pub static UPDATE_SCRIPT_URL: &str = "todo";

pub static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| match home::home_dir() {
    Some(dir) => dir,
    None => abort("Failed to get home directory", None),
});

// pub static PEN_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(".pen"));
// pub static TMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("temp"));
// pub static PYTHON_VERSIONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("python_versions"));

///////////////////
pub static BIN_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(".local/bin"));
pub static PEN_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(".cache/pen"));
pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| HOME_DIR.join(".config"));

pub static TMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("temp"));
pub static PYTHON_VERSIONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("python"));
pub static PYTHON_PACKAGES_DIR: LazyLock<PathBuf> = LazyLock::new(|| PEN_DIR.join("packages"));

pub static PEN_BIN_FILE: LazyLock<PathBuf> = LazyLock::new(|| BIN_DIR.join("pen"));
pub static PEN_CONFIG_FILE: LazyLock<PathBuf> = LazyLock::new(|| CONFIG_DIR.join("pen"));
