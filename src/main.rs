use clap::{Arg, Command};
use semver::VersionReq;
use utils::abort;

mod commands;
mod constants;
mod env_utils;
mod py_utils;
mod utils;

// todo a big question i have is should we not use abort and instead return errors and not exit whenever in the program?

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
            .arg(Arg::new("pyversion") // todo do we need required(true)
                .help("Specify the Python version (ex. pen init 3.11.9)")
                .index(1)))
        .subcommand(Command::new("sync")
            .visible_alias("s")
            .about("Syncs the installed packages and the .venv with the pen.toml file")
            .long_about("Creates the .venv according to the config"))
        .subcommand(Command::new("pkgs")
            .about("Lists packages todo")
            .long_about("todo"))
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
            .visible_alias("a"))
            .long_about("Activate the virtual environment in the current directory")
        .subcommand(Command::new("list")
            .visible_alias("l")
            .about("List Python versions")
            .long_about("List the installed Python versions from pen"))
        .subcommand(Command::new("delete")
            .about("Delete a Python version")
            .long_about("Delete a specific Python version")
            .arg(Arg::new("pyversion")
                .help("Specify the Python version to delete (this might break installed .venv until the version is installed again)")
                .required(true)
                .index(1)))
        .subcommand(Command::new("update")
            .about("Update pen")
            .long_about("Update pen to the latest version, if available"))
        .subcommand(Command::new("uninstall")
            .about("Uninstall pen")
            .long_about("Completely uninstall pen from the computer (does not include virtual environements)"))

        .get_matches();

	let dependencies = vec!["curl", "tar", "make"]; // todo goal of having no system dependencies
	utils::assert_dependencies(dependencies);
	utils::assert_global_paths();
	utils::clear_temp();

	match matches.subcommand() {
		// Python
		Some(("list", _args)) => {
			commands::py_list_versions();
		}
		Some(("delete", args)) => {
			let py_version: &String = args.get_one("pyversion").expect("required argument");
			commands::py_delete_version(py_version);
		}

		//* Pen
		Some(("init", args)) => {
			let version = utils::user_string_to_version(args.get_one::<String>("pyversion"));

			commands::env_init(version);
		}
		Some(("sync", _args)) => {
			commands::env_sync();
		}
		Some(("pkgs", _args)) => {
			commands::env_pkgs();
		}
		Some(("add", args)) => {
			let name = args.get_one::<String>("name").expect("required argument");
			let version = match args.get_one::<String>("version") {
				Some(version) => match VersionReq::parse(version) {
					Ok(version) => version,
					Err(e) => abort("Invalid version range", Some(&e)),
				},
				None => VersionReq::default(),
			};

			commands::env_add(name, &version);
		}
		Some(("activate", _args)) => {
			commands::pen_activate();
		}

		// Installation
		Some(("uninstall", _args)) => {
			commands::pen_uninstall();
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
