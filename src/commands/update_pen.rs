use std::process::Command;

pub fn update() {
    // Define the path to the updater.sh script
    let updater_script_path: String = format!("{}/.pen/updater.sh", std::env::var("HOME").unwrap());

    // Execute the updater.sh script
    Command::new("bash")
        .arg(updater_script_path)
        .status()
        .expect("Failed to execute updater.sh");

    println!("Updater script executed successfully.");
}
