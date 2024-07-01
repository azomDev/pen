#!/bin/bash

# Define variables
PEN_DIR="$HOME/.pen"
PYTHON_VERSIONS_DIR="$PEN_DIR/pythonVersions"
BASHRC="$HOME/.bashrc"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/pen.sh"
PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/pen"

# Create .pen directory in the home of the user
mkdir -p "$PEN_DIR"

# Curl the pen.sh script and penCreateEnv executable from GitHub and put them in the .pen directory
curl -o "$PEN_DIR/pen.sh" "$PEN_SCRIPT_URL"
curl -L -o "$PEN_DIR/penCreateEnv" "$PEN_EXECUTABLE_URL"

# Make the penCreateEnv executable
chmod +x "$PEN_DIR/penCreateEnv"

# Create pythonVersions directory inside .pen
mkdir -p "$PYTHON_VERSIONS_DIR"

# Add alias to the bashrc file
if ! grep -q "alias pen=" "$BASHRC"; then
    echo 'alias pen=". $HOME/.pen/pen.sh"' >> "$BASHRC"
    echo "Alias for pen added to $BASHRC"
else
    echo "Alias for pen already exists in $BASHRC"
fi
source "$BASHRC"
echo "Installation complete."
