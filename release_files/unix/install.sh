#!/usr/bin/env bash

PEN_DIR="$HOME/.pen"
TMP_DIR="/tmp"
TMP_PEN_DIR="$TMP_DIR/pen_tmp"
LINK_PATH="/usr/local/bin/pen"

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

BASE_URL="https://raw.githubusercontent.com/azomDev/pen/main/release_files/unix"
if [ "$1" == "TESTING_ARG_DO_NOT_USE" ]; then
    echo "USING TESTING BRANCH SPECIFIED IN INSTALL SCRIPT, YOU SHOULD KNOW WHAT YOU ARE DOING."
    # This url can be changed to test different places for testing.
    BASE_URL="https://raw.githubusercontent.com/azomDev/pen/refs/heads/trying-symoblic-linking-for-pen-core/release_files/unix"
fi

case "$OSTYPE" in
  linux-gnu) FILES_URL="$BASE_URL/linux" ;;
  darwin*)   FILES_URL="$BASE_URL/macos" ;;
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

# Attempt to download core
if ! curl -4 --fail -s -o "$TMP_PEN_DIR/core" "$FILES_URL/core"; then
  echo "Failed to download core. Exiting."
  exit 1
fi

# Download the checksum
if ! curl -4 --fail -s -o "$TMP_PEN_DIR/core.sha256" "$FILES_URL/core.sha256"; then
  echo "Failed to download checksum. Exiting."
  exit 1
fi

# Verify checksum
cd "$TMP_PEN_DIR" || exit 1
if ! sha256sum -c core.sha256 --status --strict; then
    echo "Checksum verification failed. Exiting."
    exit 1
fi

## CREATE AND USE MAIN PEN DIRECTORY

mkdir "$PEN_DIR" || { echo "Failed to create PEN_DIR. Exiting."; exit 1; }

mv "$TMP_PEN_DIR/"* "$PEN_DIR" || {
    echo "Failed to move files to $PEN_DIR."
    handle_failure
}

chmod +x "$PEN_DIR/core" || { echo "Failed to make files executable. Exiting."; handle_failure; }

mkdir "$PEN_DIR/python_versions" || { echo "Failed to create python_versions directory. Exiting."; handle_failure; }
mkdir "$PEN_DIR/temp" || { echo "Failed to create temp directory. Exiting."; handle_failure; }

## CREATE SYMLINK

echo "Creating a symbolic link at $LINK_PATH requires elevated permissions. Please enter your password."
sudo -k ln -s "$PEN_DIR/core" "$LINK_PATH" || { echo "Failed to create symbolic link at $LINK_PATH"; handle_failure; }

## DONE

echo -e "\033[1;32mINSTALLATION COMPLETE.\033[0m"
