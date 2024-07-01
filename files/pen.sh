#!/bin/bash

# Function to print help message
print_help() {
  cat << EOF
This tool helps with managing Python environments with different Python versions.

Usage:
  pen {activate|deactivate|create} [options]

Commands:
  activate             Activate the virtual environment.
  deactivate           Deactivate the virtual environment.
  create --pyversion=VERSION
                       Create a new virtual environment with the specified Python version.

Options:
  -h, --help           Show this help message.

Examples:
  pen activate
  pen deactivate
  pen create --pyversion=3.11.9
EOF
}

# Function to activate the virtual environment
activate() {
  if [ -f "./env/bin/activate" ]; then
    # Source the activate script to activate the virtual environment
    source ./env/bin/activate
    if [[ "$VIRTUAL_ENV" != "" ]]; then
      echo "Virtual environment activated."
    else
      echo "Failed to activate virtual environment."
    fi
  else
    echo "Virtual environment not found in ./env/bin. Please ensure the virtual environment exists."
  fi
}

# Function to deactivate the virtual environment
deactivate_env() {
  if [[ -n "$VIRTUAL_ENV" ]]; then
    # Deactivate the virtual environment
    deactivate
    echo "Virtual environment deactivated."
  else
    echo "No virtual environment is currently active."
  fi
}

# Function to handle invalid commands
invalid_command() {
  echo "Invalid command. Usage: $0 {activate|deactivate|create}"
  print_help
}

# If no arguments are passed, show help message
if [ $# -eq 0 ]; then
  print_help
fi

# Check the first argument passed to the script
case "$1" in
  activate)
    activate
    ;;
  deactivate)
    deactivate_env
    ;;  
  create)
    ./../target/release/pen "$@"
    ;;
  -h|--help|"")
    print_help
    ;;
  *)
    invalid_command
    ;;
esac
