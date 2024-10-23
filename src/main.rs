use clap::{Arg, Command};
use home;
use utils::abort;
use std::{path::PathBuf, sync::LazyLock};

mod commands;
mod py_install_algorithms;
mod utils;

// help_template.rs
// line 1059
// spec_vals.push(format!("[aliases: {all_als}]"));

// global constants
pub static ENV_DIR_NAME: &str = ".venv";
pub static UPDATE_SCRIPT_URL: &str = "todo";

pub static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    // hom_dir can return an empty string, but assert_global_paths handles that case
    return match home::home_dir() {
        Some(dir) => dir,
        None => abort("Failed to get home directory", None)
    };
});

pub static PEN_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    return HOME_DIR.join(".pen");
});

pub static TMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    return PEN_DIR.join("temp");
});

pub static PYTHON_VERSIONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    return PEN_DIR.join("python_versions");
});

fn main() {
    let matches = Command::new("pen")
        .bin_name("pen")
        .version("0.4.0")
        .about("pen is a tool for managing Python environments with different Python versions.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .help_template("{about} (v{version})\n\n{usage-heading} {usage}\n\n{all-args}")

        .subcommand(Command::new("create")
            .visible_alias("c")
        //     .styles(clap::builder::styling::Styles::styled()
        //     .header(clap::builder::styling::AnsiColor::Green.on_default() | clap::builder::styling::Effects::BOLD)
        // )
            .about("Create a virtual environment with a Python version")
            .long_about("Create a new virtual environment with the specified Python version in the current directory")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version (ex. pen create 3.11.9)")
                .required(true)
                .index(1)))
        .subcommand(Command::new("install")
            .visible_alias("i")
            .about("Install a Python version")
            .long_about("Install a specified Python version")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version (ex. pen install 3.11.9)")
                .required(true)
                .index(1)))
        .subcommand(Command::new("list")
            .visible_alias("l")
            .about("List Python versions")
            .long_about("List the installed Python versions from pen"))
        .subcommand(Command::new("delete")
            .about("Delete the virtual environment or a Python version")
            .long_about("Delete the virtual environment in the current directory or delete a specific Python version")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version to delete (to delete the virtual environement, run the command without an argument")
                .required(false)
                .index(1)))
        .subcommand(Command::new("update")
            .about("Update pen")
            .long_about("Update pen to the latest version, if available"))
        .subcommand(Command::new("uninstall")
            .about("Uninstall pen")
            .long_about("Completely uninstall pen from the computer (does not include virtual environements)"))
        .subcommand(Command::new("activate")
            .about("Activate the virtual environment")
            .long_about("Activate the virtual environment in the current directory")
            .visible_alias("a"))

        .get_matches();

    let dependencies = vec!["curl", "tar", "make", "sh"];
    utils::assert_dependencies(dependencies);
    utils::assert_global_paths();
    utils::clear_temp();

    match matches.subcommand() {
        Some(("activate", _args)) => {
            commands::activate_env();
        }
        Some(("create", args)) => {
            let py_version: &String = args.get_one("pyversion").expect("required argument");
            commands::create_env(&py_version);
        }
        Some(("install", args)) => {
            let pyversion: &String = args.get_one("pyversion").expect("required argument");
            commands::install_py_version(&pyversion);
        }
        Some(("delete", args)) => {
            if let Some(py_version) = args.get_one::<String>("pyversion") {
                commands::delete_py_version(&py_version);
            } else {
                commands::delete_env();
            }
        }
        Some(("list", _args)) => {
            commands::list_py_versions();
        }
        Some(("uninstall", _args)) => {
            commands::uninstall();
        }
        Some(("update", _args)) => {
            let message = "Updating pen automatically is not yet implemented. For now, uninstall pen with `pen uninstall` and download it again to update it. Updates will be coming in v1.0.0 so keep an eye on the \x1b]8;;https://github.com/azomDev/pen\x1b\\\x1b[34mgithub\x1b[0m\x1b]8;;\x1b\\";
            println!("{}", message);
        }
        _ => {
            abort("Unknown command", None);
        }
    }
}
