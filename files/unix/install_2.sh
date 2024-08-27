#!/bin/sh

PEN_DIR="$HOME/.pen"
# Check if the pen directory exists, if yes, exit
if [ -d "$PEN_DIR" ]; then
    echo "Directory $PEN_DIR already exists. Exiting."
    exit 1
fi

TMP_DIR="/tmp"
# Check if the tmp directory exists, if not, exit
if [ ! -d "$TMP_DIR" ]; then
    echo "/tmp directory does not exist. Aborting installation."
    exit 1
fi

TMP_PEN_DIR="$TMP_DIR/pen_tmp"
# Check if TMP_PEN_DIR exists
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



PEN_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/main.sh"
case "$OSTYPE" in
  linux-gnu)
    PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/linux/core"
    ;;
  darwin*)
    PEN_EXECUTABLE_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/unix/macos/core"
    ;;
  *)
    echo "Unsupported operating system. Exiting."
    exit 1
    ;;
esac

handle_failure() {
    rm -rf "$PEN_DIR" || { echo "Catastrophic failure: Unable to delete $PEN_DIR. Please manually remove this directory if necessary by running 'rm -rf $PEN_DIR' in your terminal."; exit 1; }
    rm -rf "$TMP_PEN_DIR"
    exit 1
}

# Curl the main.sh script and core executable from GitHub and put them in the tmp directory
curl -4 -s -o "$TMP_PEN_DIR/main.sh" "$PEN_SCRIPT_URL" || { echo "Failed to download main.sh. Exiting."; cleanup; exit 1; }
curl -4 -s -o "$TMP_PEN_DIR/core" "$PEN_EXECUTABLE_URL" || { echo "Failed to download core. Exiting."; cleanup; exit 1; }

mkdir -p "$PEN_DIR" || { echo "Failed to create PEN_DIR. Exiting."; exit 1; }

# Move downloaded files to the .pen directory
mv "$TMP_PEN_DIR/"* "$PEN_DIR" || {
    echo "Failed to move files to $PEN_DIR."
    handle_failure
}

# Make main.sh and core executable
chmod +x "$PEN_DIR/main.sh" "$PEN_DIR/core" || { echo "Failed to make files executable. Exiting."; handle_failure; }

mkdir -p "$PEN_DIR/python_versions"|| { echo "Failed to create python_versions directory. Exiting."; handle_failure; }


###########################################################
###########################################################
###########################################################

# Function to add text to a single configuration file (appending to the end)
add_text() {

  handle_add_text_failure() {
    sed -i '/^$/d' "$config_file" || { echo "Failed to remove empty lines from $config_file. Exiting."; handle_failure; }
    handle_failure
  }
  local config_file="$1"

  # Create the file if it does not exist
  if [[ ! -f "$config_file" ]]; then
    echo "Configuration file $config_file does not exist. Creating it..."
    if ! touch "$config_file"; then
      echo "Failed to create configuration file $config_file. Exiting."
      handle_failure
    fi
  fi

  if [[ -f "$config_file" ]]; then
    echo "Appending text to $config_file..."
    # Add a newline before adding the new text
    if ! echo >> "$config_file"; then
      echo "Failed to add newline to $config_file. Exiting."
      handle_add_text_failure
    fi

    # Write all lines of text to the file
    if ! echo '# pen' >> "$config_file"; then
      echo "Failed to append text to $config_file. Exiting."
      handle_add_text_failure
    fi

    if ! echo 'alias pen=". $HOME/.pen/main.sh"' >> "$config_file"; then
      echo "Failed to append text to $config_file. Exiting."
      # Remove the first found instance of '# pen' from the end of the file
      sed -i '$s/# pen//g' "$config_file" || { echo "Failed to remove '# pen' from $config_file. Exiting."; handle_add_text_failure; }
      handle_add_text_failure
    fi
  else
    echo "Configuration file $config_file does not exist."
    handle_failure
  fi
}

# Add to the appropriate shell config file based on the default shell
case "$SHELL" in
    */bash)
        add_text "$HOME/.bashrc"
        ;;
    */zsh)
        add_text "$HOME/.zshrc"
        ;;
    *)
        echo "Unsupported shell: $DEFAULT_SHELL"
        ;;
esac
