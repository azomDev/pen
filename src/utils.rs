use regex::Regex;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command as ProcessCommand;

use crate::PYTHON_VERSIONS_DIR;

// todo rename to is_well_formated?
// pub fn check_version_format(version: &str) -> bool {
//     let re = Regex::new(r"^\d+\.\d+(\.\d+)?$").expect("Invalid regex");
//     return re.is_match(version);
// }
//

pub fn is_major_minor(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+$").expect("Invalid regex");
    re.is_match(version)
}

pub fn is_major_minor_patch(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+$").expect("Invalid regex");
    re.is_match(version)
}

pub fn is_major_minor_or_patch(version: &str) -> bool {
    is_major_minor(version) || is_major_minor_patch(version)
}

pub fn get_version_path(pyversion: &str) -> PathBuf {
    let version_dir_name = format!("python_{}", pyversion);
    return PYTHON_VERSIONS_DIR.join(version_dir_name);
}

pub fn ask_for_confirmation(prompt: &str) -> bool {
    // Display the prompt
    println!("{}", prompt);

    // Flush stdout to ensure the prompt appears before reading input
    io::stdout().flush().unwrap();

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Trim the input to remove any leading/trailing whitespace
    let input = input.trim();

    // Check if the user confirmed the action
    return input.eq_ignore_ascii_case("y");
}

pub fn get_latest_patch_version(minor_version: &str) -> String {
    let mut patch_version = 0;

    loop {
        // Construct the full version string and the URL
        let full_version = format!("{}.{}", minor_version, patch_version);
        let url = format!(
            "https://www.python.org/ftp/python/{}/Python-{}.tgz",
            full_version, full_version
        );

        // Execute the curl command to get the HTTP status code
        let status_code_output = ProcessCommand::new("curl")
            .arg("-4") // Force IPv4
            .arg("-s")
            .arg("-o")
            .arg("/dev/null") // Discard the output
            .arg("-w")
            .arg("%{http_code}") // Get the HTTP status code
            .arg("-X")
            .arg("HEAD") // Use HEAD request
            .arg(&url)
            .output();

        // Check the result of the curl command
        match status_code_output {
            Ok(output) => {
                let status_code_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if let Ok(status_code) = status_code_str.parse::<u16>() {
                    if status_code == 200 {
                        patch_version += 1; // Valid version, increment and continue
                    } else if patch_version == 0 {
                        return "none".to_string(); // No valid versions found, return "none"
                    } else {
                        return format!("{}.{}", minor_version, patch_version - 1);
                        // Return the last valid version
                    }
                } else {
                    eprintln!("Error: Failed to parse HTTP status code.");
                    return "none".to_string(); // Return "none" on error
                }
            }
            Err(e) => {
                eprintln!("Error executing curl command: {}", e);
                return "none".to_string(); // Return "none" on error
            }
        }
    }
}
