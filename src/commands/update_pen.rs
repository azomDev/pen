// use reqwest::blocking::Client;
// use reqwest::header::USER_AGENT;
// use std::fs::File;
// use std::io::copy;
// use std::path::PathBuf;
// use std::process::Command;

// #[cfg(unix)]
// use std::os::unix::fs::PermissionsExt;

// pub fn update(temp_dir: &PathBuf, update_script_url: &str) {
//     let temp_file_path = temp_dir.join("update_script");

//     if download_file(update_script_url, &temp_file_path) {
//         if !run_update_script(&temp_file_path) {
//             eprintln!("Error: Failed to execute the update script");
//             return;
//         }
//     } else {
//         eprintln!("Error: Failed to download update script");
//         return;
//     }
//     println!("Update successful");
// }

// fn download_file(url: &str, file_path: &PathBuf) -> bool {
//     let client = match Client::new()
//         .get(url)
//         .header(USER_AGENT, "Rust reqwest")
//         .send()
//     {
//         Ok(response) => match response.error_for_status() {
//             Ok(valid_response) => valid_response,
//             Err(_) => return false,
//         },
//         Err(_) => return false,
//     };

//     let mut file = match File::create(&file_path) {
//         Ok(f) => f,
//         Err(_) => return false,
//     };

//     let content = match client.bytes() {
//         Ok(bytes) => bytes,
//         Err(_) => return false,
//     };

//     if copy(&mut content.as_ref(), &mut file).is_err() {
//         return false;
//     }

//     return true;
// }

// fn run_update_script(file_path: &PathBuf) -> bool {
//     if cfg!(target_os = "windows") {
//         if Command::new("cmd")
//             .args(&["/C", file_path.to_str().unwrap()])
//             .status()
//             .is_err()
//         {
//             return false;
//         }
//     } else {
//         // Make the script executable
//         let perms = match std::fs::metadata(file_path) {
//             Ok(metadata) => metadata.permissions(),
//             Err(_) => return false,
//         };

//         let mut perms = perms;
//         #[cfg(unix)]
//         perms.set_mode(0o755);

//         if std::fs::set_permissions(file_path, perms).is_err() {
//             return false;
//         }

//         if Command::new("/bin/sh")
//             .arg(file_path.to_str().unwrap())
//             .status()
//             .is_err()
//         {
//             return false;
//         }
//     }

//     true
// }
