use clap::{Arg, Command};
use std::process;

mod commands;
mod utils;

fn main() {
    let matches = Command::new("pen")
        .bin_name("pen")
        .version("0.1.0")
        .author("azomDev, azom.developer@gmail.com")
        .about("This tool helps with managing Python environments with different Python versions.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("create")
            .visible_alias("c")
            .about("Create a new virtual environment with the specified Python version in the current directory")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version (ex. pen create 3.11.9)")
                .required(true)
                .index(1)))
        .subcommand(Command::new("install")
            .visible_alias("i")
            .about("Install a specified Python version")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version (ex. pen install 3.11.9)")
                .required(true)
                .index(1)))
        .subcommand(Command::new("delete")
            .about("Delete the virtual environment in the current directory or delete a specific Python version")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version to delete (to delete the virtual environement, run the command without an argument")
                .required(false)
                .index(1)))
        .subcommand(Command::new("list")
            .visible_alias("l")
            .about("Lists the installed Python versions from pen"))
        .subcommand(Command::new("update")
            .about("Updates pen to the latest version, if available"))
        .subcommand(Command::new("uninstall")
            .about("Completely uninstalls pen from the computer (does not include virtual environements)"))
        // activate and deactivate subcommands will never happen in the rust code, so this is used for doc and help messages
        .subcommand(Command::new("activate")
            .about("Activates the virtual environment in the current directory")
            .alias("a"))
            .subcommand(Command::new("deactivate")
            .about("Deactivates the virtual environment")
            .alias("d"))

        .get_matches();

        if !utils::does_pen_dir_exists() {
            println!("Error: .pen directory does not exist in home directory, exiting.");
            process::exit(1);
        }

        match matches.subcommand() {
            Some(("create", sub_m)) => {
                let pyversion: &String = sub_m.get_one("pyversion").expect("required argument");
                println!("Installing Python version: {}", pyversion);

                if utils::check_version_format(pyversion) {
                    println!("Installing Python version: {}", pyversion);

                    let version_path = utils::get_version_path(pyversion);

                    commands::create_env(pyversion, &version_path);
                } else {
                    println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
                }
            }
            Some(("install", sub_m)) => {
                let pyversion: &String = sub_m.get_one("pyversion").expect("required argument");

                if utils::check_version_format(pyversion) {
                    println!("Installing Python version: {}", pyversion);

                    let version_path = utils::get_version_path(pyversion);

                    commands::install_version(pyversion, &version_path);
                } else {
                    println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
                }
            }
            Some(("delete", sub_m)) => {
                if let Some(pyversion) = sub_m.get_one::<String>("pyversion") {
                    // todo add confirmation
                    if utils::check_version_format(pyversion) {
                        println!("Deleting Python version: {}", pyversion);
                        let version_path = utils::get_version_path(pyversion);
                        commands::delete_version(&version_path, pyversion)
                    } else {
                        println!("Invalid Python version format. Please use the format 'number.number' or 'number.number.number'.");
                    }
                } else {
                    // todo add confirmation
                    println!("Deleting the virtual environment in the current directory");
                    commands::delete_env();
                }
            }
            Some(("list", _sub_m)) => {
                println!("Listing installed Python versions:");
                commands::list();
            }
            Some(("uninstall", _sub_m)) => {
                // todo add confirmation
                println!("Uninstalling pen...");

                commands::uninstall();
            }
            Some(("update", _sub_m)) => {
                // todo add confirmation
                println!("Updating pen");
                commands::update();
            }
            _ => {
                eprintln!("Unknown command");
            }
        }
}
