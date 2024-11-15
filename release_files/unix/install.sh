#!/usr/bin/env sh

# Just making sure $HOME isn't playing hide and seek
if [ ! -d "$HOME" ]; then
    echo "$HOME directory does not exist. Aborting installation."
    exit 1
fi

BIN_DIR="$HOME/.local/bin"
PEN_BIN_FILE="$BIN_DIR/pen"

CONFIG_DIR="$HOME/.config"
PEN_CONFIG_FILE="$CONFIG_DIR/pen.toml"

ROOT_TMP_DIR="/tmp"
ROOT_TMP_PEN_DIR="$ROOT_TMP_DIR/pen_tmp"
ROOT_TMP_PEN_CORE_FILE="$ROOT_TMP_PEN_DIR/core"
CHECKSUM_FILE_NAME="core.sha256"
ROOT_TMP_PEN_CHECKSUM_FILE="$ROOT_TMP_PEN_DIR/$CHECKSUM_FILE_NAME"

PEN_DIR="$HOME/.cache/pen"
PYTHON_VERSIONS_DIR="$PEN_DIR/python"
PYTHON_PACKAGES_DIR="$PEN_DIR/packages"
PEN_TEMP_DIR="$PEN_DIR/temp"


# PREPARING INSTALLATION


BASE_URL="https://raw.githubusercontent.com/azomDev/pen/main/release_files/unix"
if [ "$1" == "TESTING_ARG_DO_NOT_USE" ]; then
    echo "USING TESTING BRANCH SPECIFIED IN INSTALL SCRIPT, YOU SHOULD KNOW WHAT YOU ARE DOING."
    BASE_URL="$2"
fi

case "$OSTYPE" in
  linux-gnu) FILES_URL="$BASE_URL/linux" ;;
  darwin*)   FILES_URL="$BASE_URL/macos" ;;
  *)         echo "Unsupported operating system. Exiting." && exit 1 ;;
esac


## CHECKING BASIC DIRECTORIES AND FILES EXISTENCE


# Check if the tmp directory exists, if not, exit
if [ ! -d "$ROOT_TMP_DIR" ]; then
    echo "$ROOT_TMP_DIR directory does not exist. Aborting installation."
    exit 1
fi

# Check if a $PEN_DIR file/directory exists, if yes, exit
if [ -e "$PEN_DIR" ]; then
    echo "$PEN_DIR already exists. Exiting."
    exit 1
fi

# Check if a $PEN_BIN_FILE file/directory exists, if yes, exit
if [ -e "$PEN_BIN_FILE" ]; then
    echo "$PEN_BIN_FILE already exists. Exiting."
    exit 1
fi

# Check if a $PEN_CONFIG_FILE file/directory exists, if yes, exit
if [ -e "$PEN_CONFIG_FILE" ]; then
    echo "$PEN_CONFIG_FILE already exists. Exiting."
    exit 1
fi

if [ -d "$ROOT_TMP_PEN_DIR" ]; then
    # Clear all contents in $ROOT_TMP_PEN_DIR while keeping the directory
    rm -rf "$PEN_TEMP_DIR"/* || { echo "Failed to clear $ROOT_TMP_PEN_DIR. Exiting."; exit 1; }
else
    # Create $ROOT_TMP_PEN_DIR if it does not exist
    mkdir "$ROOT_TMP_PEN_DIR" || { echo "Failed to create $ROOT_TMP_PEN_DIR. Exiting."; exit 1; }
fi


## DOWNLOAD FILES


# -4: Force the use of IPv4
# --fail: Make curl fail silently on HTTP errors
# -s: Run curl in "silent" mode, suppressing progress output
# -o: Save the downloaded content to the file specified by $ROOT_TMP_PEN_CORE_FILE

# Download the core (pen binary) from the specified URL
if ! curl -4 --fail -s -o "$ROOT_TMP_PEN_CORE_FILE" "$FILES_URL/core"; then
  echo "Failed to download core. Exiting."
  exit 1
fi

# Download the checksum
if ! curl -4 --fail -s -o "$ROOT_TMP_PEN_CHECKSUM_FILE" "$FILES_URL/core.sha256"; then
  echo "Failed to download checksum. Exiting."
  exit 1
fi

# Verify checksum
cd "$ROOT_TMP_PEN_DIR" || { echo "Failed to change directory to $ROOT_TMP_PEN_DIR"; exit 1; }
if ! sha256sum -c $CHECKSUM_FILE_NAME --status --strict; then
    echo "Checksum verification failed. Exiting."
    exit 1
fi


## DEFINE SOME FUNCTIONS


# NOTE: I am pretty sure "rm -rf" and "rm -f" will succeed even if the specified path does not exist
handle_failure() {
    rm -rf "$PEN_DIR" || { echo -e "\033[31mCatastrophic failure: Unable to delete $PEN_DIR. Please manually remove this directory if necessary by running 'rm -rf $PEN_DIR' in your terminal.\033[0m"; }
    rm -f "$PEN_BIN_FILE" || { echo -e "\033[31mCatastrophic failure: Unable to delete $PEN_BIN_FILE. Please manually remove this file by running 'rm -f $PEN_BIN_FILE' in your terminal.\033[0m"; }
    rm -f "$PEN_CONFIG_FILE" || { echo -e "\033[31mCatastrophic failure: Unable to delete $PEN_CONFIG_FILE. Please manually remove this file by running 'rm -f $PEN_CONFIG_FILE' in your terminal.\033[0m"; }
    rm -rf "$ROOT_TMP_PEN_DIR" # Safe to fail since it's in /tmp
    exit 1
}

trap 'handle_failure; exit 1' INT HUP TERM QUIT ABRT USR1 USR2


## CREATE AND USE MAIN PEN DIRECTORY


mkdir -p "$BIN_DIR" || { echo "Failed to create $BIN_DIR. Exiting."; handle_failure; }
mkdir -p "$CONFIG_DIR" || { echo "Failed to create $CONFIG_DIR. Exiting."; handle_failure; }
mkdir -p "$PEN_DIR" || { echo "Failed to create $PEN_DIR. Exiting."; handle_failure; }
mkdir "$PYTHON_VERSIONS_DIR" || { echo "Failed to create $PYTHON_VERSIONS_DIR. Exiting."; handle_failure; }
mkdir "$PYTHON_PACKAGES_DIR" || { echo "Failed to create $PYTHON_PACKAGES_DIR. Exiting."; handle_failure; }
mkdir "$PEN_TEMP_DIR" || { echo "Failed to create $PEN_TEMP_DIR. Exiting."; handle_failure; }

touch "$PEN_TEMP_DIR" || { echo "Failed to create $PEN_TEMP_DIR. Exiting."; handle_failure; }

mv "$ROOT_TMP_PEN_CORE_FILE" "$PEN_BIN_FILE" || { echo "Failed to move files to $PEN_DIR."; handle_failure; }

chmod +x "$PEN_BIN_FILE" || { echo "Failed to make files executable. Exiting."; handle_failure; }


## DONE


echo -e "\033[1;32mINSTALLATION COMPLETE.\033[0m"
echo -e "\033[1;33mTo complete the setup, please add the following to your PATH environment variable:\033[0m"
echo -e "\033[1;33m  export PATH=\$HOME/.local/bin:\$PATH\033[0m"
echo -e "\033[1;33mThis can be added to your ~/.bashrc, ~/.zshrc, or equivalent shell configuration file.\033[0m"
