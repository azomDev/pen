use clap::{Arg, Command};
use utils::abort;

mod commands;
mod constants;
mod config;
mod py_install_algorithms;
mod utils;

// help_template.rs
// line 1059
// spec_vals.push(format!("[aliases: {all_als}]"));

fn main() {
    let matches = Command::new("pen")
        .bin_name("pen")
        .version("0.5.0")
        .about("pen is a tool for managing Python environments with different Python versions.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .help_template("{about} (v{version})\n\n{usage-heading} {usage}\n\n{all-args}")

        .subcommand(Command::new("init")
            .visible_alias("c")
        //     .styles(clap::builder::styling::Styles::styled()
        //     .header(clap::builder::styling::AnsiColor::Green.on_default() | clap::builder::styling::Effects::BOLD)
        // )
            .about("Create a virtual environment with a Python version")
            .long_about("Create a new virtual environment with the specified Python version in the current directory")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version (ex. pen create 3.11.9)")
                .index(1)))
        .subcommand(Command::new("install")
            .visible_alias("i")
            .about("Install the .venv")
            .long_about("Install a specified Python version"))
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
        .subcommand(Command::new("add")
            .about("Add a package to the current project")
            .long_about("Add a PyPI package to the current project (pip but faster)")
            .arg(Arg::new("name")
                .help("The package to install")
                .required(true)
                .index(1))
            .arg(Arg::new("version")
                .help("The version to install")
                .required(false)
                .index(2)))
        .subcommand(Command::new("activate")
            .about("Activate the virtual environment")
            .long_about("Activate the virtual environment in the current directory")
            .visible_alias("a"))

        .get_matches();

    let dependencies = vec!["curl", "tar", "make"]; // todo goal of having no system dependencies
    utils::assert_dependencies(dependencies);
    utils::assert_global_paths();
    utils::clear_temp();

    match matches.subcommand() {
        // Venv
        Some(("activate", _args)) => {
            commands::activate_env();
        }
        Some(("delete", args)) => {
            if let Some(py_version) = args.get_one::<String>("pyversion") {
                commands::delete_py_version(&py_version);
            } else {
                commands::delete_env();
            }
        }

        // Python
        Some(("list", _args)) => {
            commands::list_py_versions();
        }

        //* Pen
        Some(("init", args)) => {
            let pyversion: Option<&String> = args.get_one("pyversion");
            commands::init(pyversion).unwrap();
        }
        Some(("install", _args)) => {
            commands::install().unwrap();
        }
        Some(("add", args)) => {
            let name: &String = args.get_one("name").expect("required argument");
            let version: Option<&String> = args.get_one("version");
            commands::add_packages(name, version).unwrap();
        }

        // Pen
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
