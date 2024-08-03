#[macro_use]
extern crate lazy_static;

use clap::{Arg, Command};
use std::{fs, path::PathBuf, process};
// use dirs::home_dir;

mod commands;
mod utils;

// help_template.rs
// line 1059
// spec_vals.push(format!("[aliases: {all_als}]"));

pub static ENV_DIR_NAME: &str = "env";

lazy_static! {
    // let update_script_url = "https://raw.githubusercontent.com/azomDev/pen/main/files/update.sh";
    pub static ref HOME_DIR: PathBuf = dirs::home_dir().expect("Failed to get home directory");
    pub static ref PEN_DIR: PathBuf = {
        let dir = HOME_DIR.join(".pen");
        if !dir.exists() || !dir.is_dir() {
            eprintln!("Error: {} directory does not exist", dir.display());
            process::exit(1);
        }
        return dir;
    };
    pub static ref TMP_DIR: PathBuf = {
        let dir = PEN_DIR.join("temp");
        return dir;
    };

    pub static ref PYTHON_VERSIONS_DIR: PathBuf = {
        let dir = PEN_DIR.join("python_versions");
        if !dir.exists() || !dir.is_dir() {
            eprintln!("Weird, the directory {} does not exist. Creating it...", dir.display());
            fs::create_dir(&dir).expect(&format!("Failed to create {}", dir.display()));
        }
        return dir;
    };
    pub static ref PYTHON_VERSION_INFO_DIR: PathBuf = PEN_DIR.join("python_version_info");
}

// todo pen create with no version means the globally installed version

fn main() {
    let matches = Command::new("pen")
        .bin_name("pen")
        .version("0.2.0")
        .about("pen is a tool for managing Python environments with different Python versions.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .help_template("{about} (v{version})\n\n{usage-heading} {usage}\n\n{all-args}")
        .subcommand(Command::new("create")
            .visible_alias("c")
            .styles(clap::builder::styling::Styles::styled()
            .header(clap::builder::styling::AnsiColor::Green.on_default() | clap::builder::styling::Effects::BOLD)
        )
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

        // activate and deactivate subcommands will never happen in the rust code, so this is used for doc and help messages
        .subcommand(Command::new("activate")
            .about("Activate the virtual environment")
            .long_about("Activate the virtual environment in the current directory")
            .visible_alias("a"))
        .subcommand(Command::new("deactivate")
            .about("Deactivate the virtual environment")
            .visible_alias("d"))

        .subcommand(Command::new("update")
            .about("Update pen")
            .long_about("Update pen to the latest version, if available"))
        .subcommand(Command::new("uninstall")
            .about("Uninstall pen")
            .long_about("Completely uninstall pen from the computer (does not include virtual environements)"))

        .get_matches();

    // clear the temp file each time a command is executed
    let _ = fs::remove_dir_all(&*TMP_DIR).is_err();
    fs::create_dir(&*TMP_DIR).expect("Failed to create temp directory");

    match matches.subcommand() {
        Some(("create", sub_m)) => {
            let pyversion: &String = sub_m.get_one("pyversion").expect("required argument");
            commands::create_env(&pyversion);
        }
        Some(("install", sub_m)) => {
            let pyversion: &String = sub_m.get_one("pyversion").expect("required argument");
            commands::install_python_version(&pyversion);
        }
        Some(("delete", sub_m)) => {
            if let Some(pyversion) = sub_m.get_one::<String>("pyversion") {
                if !utils::is_major_minor_patch(pyversion) {
                    println!("Invalid Python version format. Please use the format 'number.number.number'.");
                    process::exit(1);
                }

                let prompt = format!(
                    "Are you sure you want to remove the Python version {} from pen? (y/N)",
                    pyversion
                );
                if utils::ask_for_confirmation(&prompt) {
                    commands::delete_version(&pyversion);
                } else {
                    println!("Removing canceled.");
                }
            } else {
                if utils::ask_for_confirmation(
                    "Are you sure you want to delete the virtual environment? (y/N)",
                ) {
                    commands::delete_env();
                } else {
                    println!("Deletion canceled.");
                }
            }
        }
        Some(("list", _sub_m)) => {
            commands::list();
        }
        Some(("uninstall", _sub_m)) => {
            println!("Uninstalling pen automatically is not yet implemented.")
            // if utils::ask_for_confirmation("Are you sure you want to uninstall? (y/N)") {
            //     commands::uninstall();
            // } else {
            //     println!("Uninstall canceled.");
            // }
        }
        Some(("update", _sub_m)) => {
            let message = "Updating pen automatically is not yet implemented. For now, uninstall pen with `pen uninstall` and download it again to update it. Updates will be coming in v1.0.0 so keep an eye on the \x1b]8;;https://github.com/azomDev/pen\x1b\\\x1b[34mgithub\x1b[0m\x1b]8;;\x1b\\";
            println!("{}", message);
        
            // println!("Updating pen");
            // commands::update(&tmp_dir, update_script_url);
        }
        _ => {
            eprintln!("Unknown command");
        }
    }
}
