#!/usr/bin/env bash

PEN_DIR="$HOME/.pen"
TMP_DIR="/tmp"
TMP_PEN_DIR="$TMP_DIR/pen_tmp"
LINK_PATH="/usr/local/bin/pen"

# TODO /TMP MIGHT BE ON ANOTHER FILESYSTEM, SO DESIGN AROUND THIS

## CHECKING BASIC FOLDER AND STUFF EXISTENCE

[ -e "$LINK_PATH" ] && { echo "Symbolic link already exists: $LINK_PATH"; exit 1; }

# Check if the pen directory exists, if yes, exit
if [ -e "$PEN_DIR" ]; then
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
  linux-gnu) PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/linux/core" ;;
  darwin*)   PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/macos/core" ;;
  *)         echo "Unsupported operating system. Exiting." && exit 1 ;;
esac

## DEFINE SOME FUNCTIONS

handle_failure() {
    rm -rf "$PEN_DIR" || { echo "Catastrophic failure: Unable to delete $PEN_DIR. Please manually remove this directory if necessary by running 'rm -rf $PEN_DIR' in your terminal."; exit 1; }
    rm -rf "$TMP_PEN_DIR"
    exit 1
}

trap 'handle_failure; exit 1' INT HUP TERM QUIT ABRT USR1 USR2

## DOWNLOAD FILES

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

chmod +x "$PEN_DIR/core" || { echo "Failed to make files executable. Exiting."; handle_failure; }

mkdir -p "$PEN_DIR/python_versions"|| { echo "Failed to create python_versions directory. Exiting."; handle_failure; }

## CREATE SYMLINK

echo "Creating a symbolic link at $LINK_PATH requires elevated permissions. Please enter your password."
sudo -k ln -s "$PEN_DIR/core" "$LINK_PATH" || { echo "Failed to create symbolic link at $LINK_PATH"; handle_failure; }

## DONE

echo -e "\033[1;32mINSTALLATION COMPLETE.\033[0m"
