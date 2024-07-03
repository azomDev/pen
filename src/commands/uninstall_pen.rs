use std::fs;
use std::path::PathBuf;

pub fn uninstall(pen_dir: &PathBuf, bashrc_file: &PathBuf) {

    // Remove .pen directory if it exists
    if pen_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&pen_dir) {
            eprintln!("Failed to remove directory {}: {}", pen_dir.display(), e);
            return;
        }
        println!("Removed directory: {}", pen_dir.display());
    } else {
        println!("Directory {} does not exist.", pen_dir.display());
    }

    // Modify .bashrc to remove pen alias and comment
    if bashrc_file.exists() {
        let bashrc_content = match fs::read_to_string(&bashrc_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read file {}: {}", bashrc_file.display(), e);
                return;
            }
        };

        let mut new_bashrc_content = String::new();
        let mut pen_alias_found = false;

        for line in bashrc_content.lines() {
            if line.contains("alias pen=") || line.contains("# pen") {
                pen_alias_found = true;
                continue;
            }
            new_bashrc_content.push_str(line);
            new_bashrc_content.push('\n');
        }

        if pen_alias_found {
            if let Err(e) = fs::write(&bashrc_file, new_bashrc_content.trim_end()) {
                eprintln!("Failed to write to file {}: {}", bashrc_file.display(), e);
                return;
            }
            println!("Alias and comment removed from {}", bashrc_file.display());
        } else {
            println!("Alias and comment not found in {}", bashrc_file.display());
        }
    } else {
        println!("File {} does not exist.", bashrc_file.display());
    }
    
    println!("Uninstallation complete. Please restart your terminal session to apply the changes.");
}