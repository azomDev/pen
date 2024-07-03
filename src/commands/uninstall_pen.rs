use std::fs;
use std::path::Path;

pub fn uninstall() {
    let home_dir = std::env::var("HOME").expect("Failed to get HOME directory");
    let pen_dir = Path::new(&home_dir).join(".pen");
    let bashrc = Path::new(&home_dir).join(".bashrc");

    // Remove .pen directory if it exists
    if Path::new(&pen_dir).exists() {
        if let Err(e) = fs::remove_dir_all(&pen_dir) {
            eprintln!("Failed to remove directory {}: {}", pen_dir.display(), e);
            return;
        }
        println!("Removed directory: {}", pen_dir.display());
    } else {
        println!("Directory {} does not exist.", pen_dir.display());
    }

    // Modify .bashrc to remove pen alias and comment
    if Path::new(&bashrc).exists() {
        let bashrc_content = match fs::read_to_string(&bashrc) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read file {}: {}", bashrc.display(), e);
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
            if let Err(e) = fs::write(&bashrc, new_bashrc_content.trim_end()) {
                eprintln!("Failed to write to file {}: {}", bashrc.display(), e);
                return;
            }
            println!("Alias and comment removed from {}", bashrc.display());
        } else {
            println!("Alias and comment not found in {}", bashrc.display());
        }
    } else {
        println!("File {} does not exist.", bashrc.display());
    }
    
    println!("Uninstallation complete. Please restart your terminal session to apply the changes.");
}