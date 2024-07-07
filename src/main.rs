use clap::{Arg, Command};
use std::process;
use dirs::home_dir;

mod commands;
mod utils;

// help_template.rs
// line 1059
// spec_vals.push(format!("[aliases: {all_als}]"));

fn main() {
    let matches = Command::new("pen")
        .bin_name("pen")
        .version("0.1.0")
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
            .alias("a"))
        .subcommand(Command::new("deactivate")
            .about("Deactivate the virtual environment")
            .alias("d"))

        .subcommand(Command::new("update")
            .about("Update pen")
            .long_about("Update pen to the latest version, if available"))
        .subcommand(Command::new("uninstall")
            .about("Uninstall pen")
            .long_about("Completely uninstall pen from the computer (does not include virtual environements)"))

        .get_matches();

    
    let home_dir = home_dir().expect("Failed to get home directory");
    let pen_dir = home_dir.join(".pen");

    if !pen_dir.exists() || !pen_dir.is_dir() {
        eprintln!("Error: {} directory does not exist", pen_dir.display()); 
        process::exit(1);
    }

    let tmp_dir = pen_dir.join("temp");
    let python_versions_dir = pen_dir.join("pythonVersions");
    
    std::fs::remove_dir_all(&tmp_dir).expect("Failed to remove temp directory");
    std::fs::create_dir(&tmp_dir).expect("Failed to create temp directory");

    if !python_versions_dir.exists() || !python_versions_dir.is_dir() {
        eprintln!("Weird, the directory {} does not exist. Creating it...", python_versions_dir.display());
        std::fs::create_dir(&tmp_dir).expect(&format!("Failed to create {}", tmp_dir.display()));
    }

    let bashrc_file = home_dir.join(".bashrc");
    let update_script_url = "https://raw.githubusercontent.com/azomDev/pen/main/files/update.sh";

    let env_dir_name = "env";

    match matches.subcommand() {
        Some(("create", sub_m)) => {
            let pyversion: &String = sub_m.get_one("pyversion").expect("required argument");
            println!("Installing Python version: {}", pyversion);

            if utils::check_version_format(pyversion) {
                println!("Installing Python version: {}", pyversion);

                let version_path = utils::get_version_path(pyversion, &python_versions_dir);

                commands::create_env(pyversion, &version_path, &tmp_dir, &env_dir_name);
            } else {
                println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
            }
        }
        Some(("install", sub_m)) => {
            // todo atomic
            let pyversion: &String = sub_m.get_one("pyversion").expect("required argument");

            if utils::check_version_format(pyversion) {
                println!("Installing Python version: {}", pyversion);

                let version_path = utils::get_version_path(pyversion, &python_versions_dir);

                commands::install_version(pyversion, &version_path, &tmp_dir);
            } else {
                println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
            }
        }
        Some(("delete", sub_m)) => {
            if let Some(pyversion) = sub_m.get_one::<String>("pyversion") {
                // todo add confirmation
                if utils::check_version_format(&pyversion) {
                    println!("Deleting Python version: {}", &pyversion);
                    let version_path = utils::get_version_path(pyversion, &python_versions_dir);
                    commands::delete_version(&version_path, &pyversion, &tmp_dir);
                } else {
                    println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
                }
            } else {
                // todo add confirmation
                println!("Deleting the virtual environment in the current directory");
                commands::delete_env(&env_dir_name, &tmp_dir);
            }
        }
        Some(("list", _sub_m)) => {
            println!("Listing installed Python versions:");
            commands::list(&python_versions_dir);
        }
        Some(("uninstall", _sub_m)) => {
            // todo atomic
            // todo add confirmation
            println!("Uninstalling pen...");
            commands::uninstall(&pen_dir, &bashrc_file);
        }
        Some(("update", _sub_m)) => {
            // todo atomic
            println!("Updating pen");
            commands::update(&tmp_dir, update_script_url);
        }
        _ => {
            eprintln!("Unknown command");
        }
    }
}
