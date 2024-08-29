#!/usr/bin/env bash

# Function to activate the virtual environment
activate_env() {
  if [ ! -d "./env" ]; then
    echo "The ./env directory is missing, which means no virtual environment is present"
    return
  fi

  if [ -f "./env/bin/activate" ]; then
    source ./env/bin/activate
    if [[ "$VIRTUAL_ENV" != "" ]]; then
      echo "Virtual environment activated"
    else
      echo "Failed to activate virtual environment"
    fi
  else
    echo "Virtual environment not found in ./env/bin"
  fi
}

# Function to deactivate the virtual environment
deactivate_env() {
  if [[ -n "$VIRTUAL_ENV" ]]; then
    deactivate
    echo "Virtual environment deactivated"
  else
    echo "No virtual environment is currently active"
  fi
}

case "$1" in
  activate | a)
    activate_env
    ;;
  deactivate | d)
    deactivate_env
    ;;
  *)
    $HOME/.pen/core "$@"
    ;;
esac
