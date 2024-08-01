#!/bin/bash

# Define the lines of text to add or remove
TEXT_FOR_PEN=(
  '# pen'
  'alias pen=". $HOME/.pen/main.sh"'
)

# Define configuration file paths for each shell
BASH_CONFIG_FILES=(
  "~/.bashrc"
)

ZSH_CONFIG_FILES=(
  "~/.zshrc"
)

# Function to add text to configuration files (appending to the end)
add_text() {
  local config_files=("${!1}")

  for file in "${config_files[@]}"; do
    file=$(eval echo "$file")  # Resolve ~ to $HOME

    # Create the file if it does not exist
    if [[ ! -f "$file" ]]; then
      echo "Configuration file $file does not exist. Creating it..."
      touch "$file"
    fi

    if [[ -f "$file" ]]; then
      echo "Appending text to $file..."
      # Add a newline before adding the new text
      echo >> "$file"
      # Write all lines of text to the file
      for line in "${TEXT_FOR_PEN[@]}"; do
        echo "$line" >> "$file"
      done
    else
      echo "Configuration file $file does not exist, and failed to create it."
    fi
  done
}

# Function to remove text from configuration files
remove_text() {
  local config_files=("${!1}")

  for file in "${config_files[@]}"; do
    file=$(eval echo "$file")  # Resolve ~ to $HOME
    if [[ -f "$file" ]]; then
      echo "Removing text from $file..."
      # Remove lines containing the pattern
      sed -i -e '/^# pen$/ {N; /^# pen\nalias pen=". $HOME\/.pen\/main.sh"\n*$/d;}' \
                 -e '/^# pen$/ {N; /^# pen\nalias pen=". $HOME\/.pen\/main.sh"$/d;}' "$file"
    else
      echo "Configuration file $file does not exist."
    fi
  done
}

# Prompt user to select shell
echo "Select the shell you want to modify:"
echo "1) Bash"
echo "2) Zsh"
read -p "Enter the number corresponding to your choice: " shell_choice

case "$shell_choice" in
  1)
    CONFIG_FILES=("${BASH_CONFIG_FILES[@]}")
    ;;
  2)
    CONFIG_FILES=("${ZSH_CONFIG_FILES[@]}")
    ;;
  *)
    echo "Invalid selection. Please choose 1 for Bash or 2 for Zsh."
    exit 1
    ;;
esac

# Check if the correct number of arguments is provided
if [[ $# -ne 1 ]]; then
  echo "Usage: $0 {add|remove}"
  exit 1
fi

# Main script execution
case "$1" in
  add)
    echo "Adding text..."
    add_text CONFIG_FILES[@]
    ;;
  remove)
    echo "Removing text..."
    remove_text CONFIG_FILES[@]
    ;;
  *)
    echo "Invalid option. Use 'add' to add text or 'remove' to remove text."
    exit 1
    ;;
esac
