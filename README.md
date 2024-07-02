# **pen**

**pen** is an easy-to-use tool for effortlessly managing and switching between virtual environments with specific Python versions.

## Features

- Create virtual environments with specific Python versions.
- Activate and deactivate virtual environments seamlessly.
- Simplified command interface for ease of use.

## Installation

For now, only linux with bash and the ~/.bashrc file are supported.
```
curl -sSL https://raw.githubusercontent.com/azomDev/pen/main/files/install.sh | bash
```
### Manual installation
You will need cargo
```
git clone https://github.com/azomDev/pen.git
cd pen
cargo build --release
mkdir ~/.pen
cp target/release/pen ~/.pen/penCreateEnv
cp files/pen.sh ~/.pen/
echo 'alias pen=". ~/.pen/pen.sh"' >> ~/.bashrc
source ~/.bashrc
cd ..
rm -rf pen/
```

## Usage
OUTATED

### Commands
Usage: pen <COMMAND>

Commands:
  create     Create a new virtual environment with the specified Python version in the current directory [aliases: c]
  install    Install a specified Python version [aliases: i]
  delete     Delete the virtual environment in the current directory or delete a specific Python version
  list       Lists the installed Python versions from pen [aliases: l]
  uninstall  Completely uninstalls pen from the computer (does not include virtual environements)
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

## Example

```sh
# Create a virtual environment with Python version 3.11.9 in the current directory
pen create --pyversion=3.11.9

# Activate the virtual environment in the current directory
pen activate

# Deactivate the virtual environment in the current directory
pen deactivate
```


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub, even for the smallest bug or the smallest idea.
If there is any issue you see that you might want to try doing, just let a comment on the issue and I will let it to you.

## License

This project is licensed under the `MIT` License. See the LICENSE file for details.
