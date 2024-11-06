# Project Structure

This document outlines the structure of the repository, providing a clear overview of the files and directories included. Each section is described to help new contributors understand the purpose and organization of the project.

## Directory Overview

### `builders/`
This directory contains scripts used for building and installing the project.

- **`dev_install.sh`**: A script to set up the development environment, including dependencies and configuration settings.
- **`linux_release_builder.sh`**: A script specifically designed to package the application for Linux distributions, preparing the necessary files for release.
- **`linux_release_builder.sh`**: A script specifically designed to package the application for Linux distributions, preparing the necessary files for release.

### `release_files/`
This directory contains compiled binaries and installation scripts for various operating systems.

- **`unix/`**
  - **`linux/`**
    - **`core`**: The main binary for Linux systems.
    - **`core.sha256`**: The SHA256 checksum for the Linux binary, used for verifying integrity.
  - **`macos/`**
    - **`core`**: The main binary for macOS systems.
    - **`core.sha256`**: The SHA256 checksum for the macOS binary.
  - **`install.sh`**: A universal installation script that automates the installation process for both Linux and macOS.

### `src/`
This is the source code directory, where the main application code resides.

### `tests/`
This is the tests directory, where the all the tests can be runned to check if pen is working well.

### Root Files

- **`.gitignore`**: Specifies files and directories that should be ignored by Git, helping to keep the repository clean.
- **`Cargo.lock`**: Automatically generated file that locks the versions of dependencies for reproducible builds.
- **`Cargo.toml`**: The manifest file for Rust projects, detailing dependencies, project metadata, and build settings.
- **`LICENSE`**: The license under which the project is distributed, specifying usage rights and obligations.
- **`README.md`**: The main documentation file that provides an overview of the project, installation instructions, and usage details.
- **`STRUCTURE.md`**: This document, outlining the repository structure for easier navigation and understanding.

## Conclusion

This structure is designed to facilitate a smooth onboarding process for new contributors. Each part of the project is organized logically, allowing for easy access to scripts, binaries, and source code. If you have any questions or need further clarification, please refer to the `README.md` or reach out to me :)
