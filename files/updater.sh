#!/usr/bin/env bash

# Define variables
PEN_DIR="$HOME/.pen"
UPDATE_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/update.sh"
UPDATE_SCRIPT_PATH="$PEN_DIR/update.sh"

# Download the update.sh script from GitHub and save it in the .pen directory
curl -o "$UPDATE_SCRIPT_PATH" "$UPDATE_SCRIPT_URL"

# Make the update.sh script executable
chmod +x "$UPDATE_SCRIPT_PATH"

# Execute the update.sh script
"$UPDATE_SCRIPT_PATH"
