# Pen

**Pen** is a tool for managing Python environments with different Python versions. (v0.4.0)

## Features

- Create and manage virtual environments with specified Python versions.
- Easily install, activate, and deactivate environments.
- Simple command interface for streamlined usage.

## Installation
Currently, this software supports only Linux and macOS environments. Windows is not supported at this time.

Requirements:
- sh (does not need to be the default shell)
- curl
- tar
- make
- bash (only for fast install)
- cargo (only for manual install)

### Fast Install
Run this in your terminal:
```
bash <(curl -sL "https://raw.githubusercontent.com/azomDev/pen/main/release_files/unix/install.sh")
```

### Manual Install (cargo needed)

1. Clone the repository and navigate to the directory:
    ```bash
    git clone https://github.com/azomDev/pen.git
    cd pen
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Set up the environment:
    ```bash
    mkdir ~/.pen
    mv target/release/pen ~/.pen/core
    TODO
    ```

4. Clean up and apply changes:
    ```bash
    cd ..
    rm -rf pen/
    sudo ln -s ~/.pen/core /usr/local/bin/pen
    ```

## Usage

**Command Format:** `pen <COMMAND>`

### Commands

- **`create`** (`c`): Create a virtual environment with a specific Python version.
    ```bash
    pen create 3.9.1
    ```

- **`install`** (`i`): Install a specific Python version.
    ```bash
    pen install 3.8.5
    ```

- **`list`** (`l`): List all installed Python versions.
    ```bash
    pen list
    ```

- **`delete`**: Delete the virtual environment in the current directory or a specific Python version.
    ```bash
    pen delete
    pen delete 3.8.5
    ```

- **`activate`** (`a`): Activate the virtual environment. You can exit the environment with `deactivate`
    ```bash
    pen activate
    ```

- **`update`**: Update Pen to the latest version. (This is not yet supported)
    ```bash
    pen update
    ```

- **`uninstall`**: Uninstall Pen (does not remove virtual environments).
    ```bash
    pen uninstall
    ```

- **`help`**: Show help for Pen or a specific command.
    ```bash
    pen
    pen help
    pen help create
    ```

### Options

- **`-h`, `--help`**: Print help information.
- **`-V`, `--version`**: Print the version of pen.


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub for any bugs or suggestions. Feel free to comment on any issue if you're interested in contributing.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
