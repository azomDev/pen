#!/usr/bin/env bash

# Define the lines of text to add or remove
TEXT_FOR_PEN=(
  '# pen'
  'alias pen=". $HOME/.pen/main.sh"'
)

# Define configuration file paths for each shell
BASH_CONFIG_FILES=(
  "~/.bashrc"
)

ZSH_CONFIG_FILES=(
  "~/.zshrc"
)

# Define variables
PEN_DIR="$HOME/.pen"
TMP_DIR="/tmp"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/main.sh"
# VERSION_TXT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/version.txt"
case "$OSTYPE" in
  linux-gnu)
    PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/linux/core"
    ;;
  darwin*)
    PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/macos/core"
    ;;
  *)
    echo "Unsupported operating system. Exiting."
    exit 1
    ;;
esac
TMP_PEN_DIR="$TMP_DIR/pen_tmp"

# Function to handle cleanup
cleanup() {
    echo "Cleaning up..."
    rm -rf "$TMP_PEN_DIR"
}

# Check if the tmp directory exists, if no, exit
if [ ! -d "$TMP_DIR" ]; then
    echo "/tmp directory does not exist. Aborting installation."
    exit 1
fi

# Check if the pen directory exists, if yes, exit
if [ -d "$PEN_DIR" ]; then
    echo "Directory $PEN_DIR already exists. Exiting."
    exit 1
fi

# Create temporary directory for downloading files
mkdir -p "$TMP_PEN_DIR"

# Curl the main.sh script and core executable from GitHub and put them in the tmp directory
curl -4 -o "$TMP_PEN_DIR/main.sh" "$PEN_SCRIPT_URL" || { echo "Failed to download main.sh. Exiting."; cleanup; exit 1; }
# curl -o "$TMP_PEN_DIR/version.txt" "$VERSION_TXT_URL" || { echo "Failed to download version.txt. Exiting."; cleanup; exit 1; }
curl -4 -o "$TMP_PEN_DIR/core" "$PEN_EXECUTABLE_URL" || { echo "Failed to download core. Exiting."; cleanup; exit 1; }

# Create .pen directory in the home of the user
mkdir -p "$PEN_DIR"

# Move downloaded files to the .pen directory
mv "$TMP_PEN_DIR/"* "$PEN_DIR" || {
    echo "Failed to move files to $PEN_DIR."
    rm -rf "$PEN_DIR" || { echo "Catastrophic failure: Unable to delete $PEN_DIR. Manual cleanup required."; cleanup; exit 1; }
    cleanup
    exit 1
}

# Cleanup temporary directory
cleanup

# Make the core executable
chmod +x "$PEN_DIR/main.sh"
chmod +x "$PEN_DIR/core"

# Create pythonVersions directory inside .pen
mkdir -p "$PEN_DIR/pythonVersions"


################################################################



# Create the alias

# Function to add text to configuration files (appending to the end)
add_text() {
  local config_files=("${!1}")

  for file in "${config_files[@]}"; do
    file=$(eval echo "$file")  # Resolve ~ to $HOME

    # Create the file if it does not exist
    if [[ ! -f "$file" ]]; then
      echo "Configuration file $file does not exist. Creating it..."
      touch "$file"
    fi

    if [[ -f "$file" ]]; then
      echo "Appending text to $file..."
      # Add a newline before adding the new text
      echo >> "$file"
      # Write all lines of text to the file
      for line in "${TEXT_FOR_PEN[@]}"; do
        echo "$line" >> "$file"
      done
    else
      echo "Configuration file $file does not exist, and failed to create it."
    fi
  done
}
# Prompt user to select shell
echo "Select the shell you want to modify:"
echo "1) Bash"
echo "2) Zsh"
read -p "Enter the number corresponding to your choice: " shell_choice

case "$shell_choice" in
  1)
    CONFIG_FILES=("${BASH_CONFIG_FILES[@]}")
    ;;
  2)
    CONFIG_FILES=("${ZSH_CONFIG_FILES[@]}")
    ;;
  *)
    echo "Invalid selection. Please choose 1 for Bash or 2 for Zsh."
    exit 1
    ;;
esac
echo "Adding text..."
add_text CONFIG_FILES[@]

echo "Installation complete. Please restart your terminal session to apply the changes."
