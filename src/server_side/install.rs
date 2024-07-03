use std::fs::{self, File};
use std::io::{self, Write};
use std::io::copy;
use std::path::PathBuf;
use dirs::home_dir;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::env::temp_dir;

fn main() -> io::Result<()> {
    let pen_dir = home_dir().expect("Failed to get home directory").join(".pen");
    let tmp_dir = temp_dir();
    let python_versions_dir = pen_dir.join("pythonVersions");

    let (pen_script_name, version_txt_name, pen_executable_name, server_url) = get_os_specific_details();
    let pen_script_url = format!("{}{}", server_url, pen_script_name);
    let version_txt_url = format!("{}{}", server_url, version_txt_name);
    let pen_executable_url = format!("{}{}", server_url, pen_executable_name);

    if pen_dir.exists() {
        println!("Directory {} already exists. Exiting", pen_dir.display());
        return Ok(());
    }

    add_alias(&pen_dir)?;

    // Create the necessary directories
    fs::create_dir_all(&python_versions_dir)?;

    // Download and move files to the appropriate directory
    if download_file(&pen_script_url, &tmp_dir.join(pen_script_name))? &&
       download_file(&version_txt_url, &tmp_dir.join(version_txt_name))? &&
       download_file(&pen_executable_url, &tmp_dir.join(pen_executable_name))? {

        fs::rename(tmp_dir.join(pen_script_name), pen_dir.join(pen_script_name))?;
        fs::rename(tmp_dir.join(version_txt_name), pen_dir.join(version_txt_name))?;
        fs::rename(tmp_dir.join(pen_executable_name), pen_dir.join(pen_executable_name))?;
    } else {
        eprintln!("Failed to download one or more files");
        return Err(io::Error::new(io::ErrorKind::Other, "Download failed"));
    }

    println!("Setup completed successfully.");

    Ok(())
}

fn add_alias(pen_dir: &PathBuf) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let profile_path = home_dir().expect("Failed to get home directory").join("Documents").join("WindowsPowerShell").join("Microsoft.PowerShell_profile.ps1");
        let alias_string = format!(r#"Set-Alias -Name pen -Value "{}\main.bat""#, pen_dir.display());
        fs::OpenOptions::new().create(true).append(true).open(profile_path)?.write_all(alias_string.as_bytes())?;
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let shell_profile = if std::env::var("SHELL").unwrap_or_default().contains("zsh") {
            home_dir().expect("Failed to get home directory").join(".zshrc")
        } else {
            home_dir().expect("Failed to get home directory").join(".bashrc")
        };
        let alias_string = format!(r#"\n# pen\nalias pen=". $HOME/.pen/main.sh""#);
        fs::OpenOptions::new().create(true).append(true).open(shell_profile)?.write_all(alias_string.as_bytes())?;
    }

    Ok(())
}

fn download_file(url: &str, file_path: &PathBuf) -> io::Result<bool> {
    let client = Client::new();
    let response = client.get(url).header(USER_AGENT, "Rust reqwest").send();

    let response = match response {
        Ok(resp) => resp,
        Err(_) => return Ok(false),
    };

    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        let content = response.bytes().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        copy(&mut content.as_ref(), &mut file)?;
        Ok(true)
    } else {
        Ok(false)
    }
}


fn get_os_specific_details() -> (&'static str, &'static str, &'static str, &'static str) {
    #[cfg(target_os = "windows")]
    {
        (
            "main.bat",
            "version.txt",
            "core.exe",
            "https://raw.githubusercontent.com/azomDev/pen/main/files/windows/"
        )
    }

    #[cfg(target_os = "macos")]
    {
        (
            "main.sh",
            "version.txt",
            "core",
            "https://raw.githubusercontent.com/azomDev/pen/main/files/macos/"
        )
    }

    #[cfg(target_os = "linux")]
    {
        (
            "main.sh",
            "version.txt",
            "core",
            "https://raw.githubusercontent.com/azomDev/pen/main/files/linux/"
        )
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        panic!("Unsupported operating system")
    }
}
