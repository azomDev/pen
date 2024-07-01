#!/usr/bin/env bash

# Define variables
PEN_DIR="$HOME/.pen"
BASHRC="$HOME/.bashrc"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/pen.sh"
PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/pen"

# Check if the .pen directory exists, if yes, exit
if [ -d "$PEN_DIR" ]; then
    echo "Directory $PEN_DIR already exists. Exiting."
    exit 1 # does this kill the terminal? if yes change
fi

# Check if .bashrc file exists, if no, exit
if [ ! -f "$BASHRC" ]; then
    echo "File $BASHRC does not exist. Exiting."
    exit 1 # does this kill the terminal? if yes change
fi

# Create .pen directory in the home of the user
mkdir -p "$PEN_DIR"

# Curl the pen.sh script and penCreateEnv executable from GitHub and put them in the .pen directory
curl -o "$PEN_DIR/pen.sh" "$PEN_SCRIPT_URL"
curl -L -o "$PEN_DIR/penCreateEnv" "$PEN_EXECUTABLE_URL"

# Make the penCreateEnv executable
chmod +x "$PEN_DIR/penCreateEnv"

# Create pythonVersions directory inside .pen
mkdir -p "$PEN_DIR/pythonVersions"

# Add alias to the bashrc file
if ! grep -q "alias pen=" "$BASHRC"; then
    {
        echo -e '\n# pen'
        echo 'alias pen=". $HOME/.pen/pen.sh"'
    } >>"$BASHRC"
    echo "Alias for pen added to $BASHRC"
else
    echo "Alias for pen already exists in $BASHRC"
fi
echo "Installation complete. Please restart your terminal session or run 'source ~/.bashrc' to apply the changes."

