#!/usr/bin/env bash

# Define variables
PEN_DIR="$HOME/.pen"
TEMP_DIR="$PEN_DIR/temp"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/main.sh"
PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/pen"
VERSION_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/version.txt"
LOCAL_VERSION_FILE="$PEN_DIR/version.txt"
ONLINE_VERSION=$(curl -s "$VERSION_URL")

# Function to log error and exit
log_error_and_exit() {
    echo "$1"
    exit 1
}

# Check if the local version file exists
if [ -f "$LOCAL_VERSION_FILE" ]; then
    LOCAL_VERSION=$(cat "$LOCAL_VERSION_FILE")
else
    LOCAL_VERSION="0.0.0"
fi

# Compare versions
if [ "$ONLINE_VERSION" == "$LOCAL_VERSION" ]; then
    echo "You already have the latest version ($LOCAL_VERSION). No update needed."
    exit 0
fi

# Create a temporary directory for downloading files
mkdir -p "$TEMP_DIR" || log_error_and_exit "Failed to create temporary directory."

# Download the new files to the temporary directory
curl -o "$TEMP_DIR/main.sh" "$PEN_SCRIPT_URL" || log_error_and_exit "Failed to download main.sh."
curl -L -o "$TEMP_DIR/core" "$PEN_EXECUTABLE_URL" || log_error_and_exit "Failed to download core."

# Make the core executable
chmod +x "$TEMP_DIR/core" || log_error_and_exit "Failed to make core executable."

# Move the new files to the .pen directory
mv "$TEMP_DIR/main.sh" "$PEN_DIR/main.sh" || log_error_and_exit "Failed to move main.sh."
mv "$TEMP_DIR/core" "$PEN_DIR/core" || log_error_and_exit "Failed to move core."

# Update the local version file
echo "$ONLINE_VERSION" > "$LOCAL_VERSION_FILE" || log_error_and_exit "Failed to update version file."

# Cleanup temporary directory
rm -rf "$TEMP_DIR"

# Provide feedback to the user
echo "Update to version $ONLINE_VERSION complete. Please restart your terminal session or run 'source ~/.bashrc' to apply the changes."
