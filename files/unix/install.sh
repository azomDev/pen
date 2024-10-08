#!/usr/bin/env bash

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

    # Determine macOS version
    macos_version=$(sw_vers -productVersion)
    major_version=$(echo "$macos_version" | awk -F '.' '{print $1}')
    minor_version=$(echo "$macos_version" | awk -F '.' '{print $2}')


    if [[ "$major_version" -gt 10 ]] || { [[ "$major_version" -eq 10 ]] && [[ "$minor_version" -ge 15 ]]; }; then
        # macOS Catalina (10.15) and later
        DEFAULT_SHELL="zsh"
    else
        # Older macOS versions
        DEFAULT_SHELL="bash"
    fi

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

trap 'handle_failure; exit 1' INT HUP TERM QUIT ABRT USR1 USR2

add_text() {
    local file=$1
    local pen_alias='alias pen=". $HOME/.pen/main.sh"'

    if [[ -f "$file" ]]; then
        # Append a newline and the string to the file
        echo -e "\n$pen_alias" >> "$file" || handle_failure
    else
        # Prompt the user to create the file if it doesn't exist
        read -p "File $file does not exist. Would you like to create it? (Y/n) " choice || handle_failure
        if [[ "$choice" == "n" || "$choice" == "N" ]]; then
            echo "File was not created. Exiting."
            handle_failure
        else
            touch "$file" || handle_failure
            echo -e "\n$pen_alias" >> "$file" || handle_failure
        fi
    fi
}

## ASK ABOUT DEFAULT SHELL
echo -e "Current default shell: \033[36m\033[1m$DEFAULT_SHELL\033[0m"
read -p "Change the default shell? (Enter 'bash', 'zsh', or press Enter to keep current): " chosen_shell

## DOWNLOAD FILES

if ! curl -4 --fail -s -o "$TMP_PEN_DIR/main.sh" "$PEN_SCRIPT_URL"; then
  echo "Failed to download main.sh. Exiting."
  exit 1
fi

# Attempt to download core; handle errors if the download fails
if ! curl -4 --fail -s -o "$TMP_PEN_DIR/core" "$PEN_EXECUTABLE_URL"; then
  echo "Failed to download core. Exiting."
  exit 1
fi

## CREATE AND USE MAIN PEN DIRECTORY

mkdir -p "$PEN_DIR" || { echo "Failed to create PEN_DIR. Exiting."; exit 1; }

mv "$TMP_PEN_DIR/"* "$PEN_DIR" || {
    echo "Failed to move files to $PEN_DIR."
    handle_failure
}

chmod +x "$PEN_DIR/main.sh" "$PEN_DIR/core" || { echo "Failed to make files executable. Exiting."; handle_failure; }

mkdir -p "$PEN_DIR/python_versions"|| { echo "Failed to create python_versions directory. Exiting."; handle_failure; }

## ADD LINE TO SHELL CONFIG

# Determine the file based on the shell

if [[ -z "$chosen_shell" ]]; then
    if [[ "$DEFAULT_SHELL" == "bash" ]]; then
        config_file="$HOME/.bashrc"
    elif [[ "$DEFAULT_SHELL" == "zsh" ]]; then
        config_file="$HOME/.zshrc"
    else
        echo "Unsupported shell: $shell. If this message is printed, please open an issue on GitHub about it."
        handle_failure
    fi
elif [[ "$chosen_shell" == "bash" ]]; then
    config_file="$HOME/.bashrc"
elif [[ "$chosen_shell" == "zsh" ]]; then
    config_file="$HOME/.zshrc"
else
    echo "Invalid input. Please enter 'bash', 'zsh', or leave empty to keep the default."
    handle_failure
fi

add_text "$config_file"


echo -e "\033[1;32mINSTALLATION COMPLETE.\033[0m"
echo "To complete the setup, please follow one of these steps:"
if [[ "$config_file" == "$HOME/.bashrc" ]]; then
    echo "1. Reload the configuration file with: source ~/.bashrc"
elif [[ "$config_file" == "$HOME/.zshrc" ]]; then
    echo "1. Reload the configuration file with: source ~/.zshrc"
fi
echo "2. Close this terminal and open a new one."
