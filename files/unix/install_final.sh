#!/bin/sh

PEN_DIR="$HOME/.pen"
TMP_DIR="/tmp"
TMP_PEN_DIR="$TMP_DIR/pen_tmp"
PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/main.sh"

## CHECKING BASIC FOLDER EXISTENCE

# Check if the pen directory exists, if yes, exit
if [ -d "$PEN_DIR" ]; then
    echo "Directory $PEN_DIR already exists. Exiting."
    exit 1
fi

# Check if the tmp directory exists, if not, exit
if [ ! -d "$TMP_DIR" ]; then
    echo "/tmp directory does not exist. Aborting installation."
    exit 1
fi

if [ -d "$TMP_PEN_DIR" ]; then
    # Clear all contents in TMP_PEN_DIR while keeping the directory
    if ! rm -rf "$TMP_PEN_DIR"/*; then
        echo "Failed to clear TMP_PEN_DIR. Exiting."
        exit 1
    fi
else
    # Create TMP_PEN_DIR if it does not exist
    mkdir -p "$TMP_PEN_DIR" || { echo "Failed to create TMP_PEN_DIR. Exiting."; exit 1; }
fi


case "$OSTYPE" in
  linux-gnu)
    PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/linux/core"
    DEFAULT_SHELL="bash"
    ;;
  darwin*)
    PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/macos/core"
    DEFAULT_SHELL="zsh"
    ;;
  *)
    echo "Unsupported operating system. Exiting."
    exit 1
    ;;
esac

## DEFINE SOME FUNCTIONS

handle_failure() {
    rm -rf "$PEN_DIR" || { echo "Catastrophic failure: Unable to delete $PEN_DIR. Please manually remove this directory if necessary by running 'rm -rf $PEN_DIR' in your terminal."; exit 1; }
    rm -rf "$TMP_PEN_DIR"
    exit 1
}

add_text() {
    local shell=$1
    local file

    pen_alias='alias pen=". $HOME/.pen/main.sh"'

    # Determine the file based on the shell
    if [[ "$shell" == "bash" ]]; then
        file="$HOME/.bashrc"
    elif [[ "$shell" == "zsh" ]]; then
        file="$HOME/.zshrc"
    else
        echo "Unsupported shell: $shell. If this message is printed, please open an issue on GitHub about it."
        handle_failure
    fi

    if [[ -f "$file" ]]; then
        # Append a newline and the string to the file
        echo -e "\n$pen_alias" >> "$file" || handle_failure
    else
        # Prompt the user to create the file if it doesn't exist
        read -p "File $file does not exist. Would you like to create it? (Y/n) " choice || handle_failure
        if [[ "$choice" == "n" || "$choice" == "N" ]]; then
            echo "File was not created. Exiting."
        else
            touch "$file" || handle_failure
            echo -e "\n$pen_alias" >> "$file" || handle_failure
        fi
    fi
}


## ASK ABOUT DEFAULT SHELL

echo "This script will configure your shell environment."
echo "The default shell for your system is $DEFAULT_SHELL."
read -p "Would you like to change the default shell? Enter 'bash' or 'zsh' to choose a new shell, or press Enter to keep the default: " chosen_shell

## DOWNLOAD FILES

curl -4 -s -o "$TMP_PEN_DIR/main.sh" "$PEN_SCRIPT_URL" || { echo "Failed to download main.sh. Exiting."; exit 1; }
curl -4 -s -o "$TMP_PEN_DIR/core" "$PEN_EXECUTABLE_URL" || { echo "Failed to download core. Exiting."; exit 1; }

## CREATE AND USE MAIN PEN DIRECTORY

mkdir -p "$PEN_DIR" || { echo "Failed to create PEN_DIR. Exiting."; exit 1; }

mv "$TMP_PEN_DIR/"* "$PEN_DIR" || {
    echo "Failed to move files to $PEN_DIR."
    handle_failure
}

chmod +x "$PEN_DIR/main.sh" "$PEN_DIR/core" || { echo "Failed to make files executable. Exiting."; handle_failure; }

mkdir -p "$PEN_DIR/python_versions"|| { echo "Failed to create python_versions directory. Exiting."; handle_failure; }

## ADD LINE TO SHELL CONFIG

if [[ -z "$chosen_shell" ]]; then
    add_text "$default_shell"
elif [[ "$chosen_shell" == "bash" || "$chosen_shell" == "zsh" ]]; then
    add_text "$chosen_shell"
else
    echo "Invalid input. Please enter 'bash', 'zsh', or leave empty to keep the default."
    handle_failure
fi
