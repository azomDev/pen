#!/usr/bin/env bash

# Define variables
PEN_DIR="$HOME/.pen"
TMP_DIR="/tmp"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/main.sh"
VERSION_TXT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/version.txt"
PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/core"
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
curl -o "$TMP_PEN_DIR/main.sh" "$PEN_SCRIPT_URL" || { echo "Failed to download main.sh. Exiting."; cleanup; exit 1; }
curl -o "$TMP_PEN_DIR/version.txt" "$VERSION_TXT_URL" || { echo "Failed to download version.txt. Exiting."; cleanup; exit 1; }
curl -o "$TMP_PEN_DIR/core" "$PEN_EXECUTABLE_URL" || { echo "Failed to download core. Exiting."; cleanup; exit 1; }

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
chmod +x "$PEN_DIR/core"

# Create pythonVersions directory inside .pen
mkdir -p "$PEN_DIR/pythonVersions"

# Create symlink
sudo ln -s ~/.pen/main.sh /usr/local/bin/pen

echo "Installation complete. Please restart your terminal session or run 'source ~/.bashrc' to apply the changes."
