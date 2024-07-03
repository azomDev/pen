# **pen**

**pen** is an easy-to-use tool for effortlessly managing and switching between virtual environments with specific Python versions.

Pen is still in alpha, so be careful when using it.

## Features

- Create virtual environments with specific Python versions.
- Activate and deactivate virtual environments seamlessly.
- Simplified command interface for ease of use.

## Installation

For now, only linux with bash and the ~/.bashrc file are supported.
```
curl -sSL https://raw.githubusercontent.com/azomDev/pen/main/files/linux/install.sh | bash
```
### Manual installation
You will need cargo
OUDATED
```
git clone https://github.com/azomDev/pen.git
cd pen
cargo build --release
mkdir ~/.pen
cp target/release/pen ~/.pen/penOtherCommands
cp files/pen.sh ~/.pen/
echo 'alias pen=". ~/.pen/pen.sh"' >> ~/.bashrc
source ~/.bashrc
cd ..
rm -rf pen/
```

## Usage
pen \<COMMAND\>

### Commands
- **create**: Create a new virtual environment with the specified Python version in the current directory
  - **Aliases**: `c`
- **install**: Install a specified Python version
  - **Aliases**: `i`
- **delete**: Delete the virtual environment in the current directory or delete a specific Python version
- **list**: Lists the installed Python versions from pen
  - **Aliases**: `l`
- **uninstall**: Completely uninstalls pen from the computer (does not include virtual environments)
- **help**: Print this message or the help of the given subcommand(s)

#### Options:
- **-h, --help**: Print help
- **-V, --version**: Print version


### Example Usage

#### Creating a new virtual environment
```
pen create 3.9.1
```

#### Installing a specific Python version
```
pen install 3.8.5
```

#### Deleting the virtual environment in the current directory
```
pen delete
```

#### Deleting a specific Python version
```
pen delete 3.8.5
```

#### Listing installed Python versions
```
pen list
```

#### Uninstalling pen completely
```
pen uninstall
```

#### Getting help for a specific command
```
pen help create
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub, even for the smallest bug or the smallest idea.
If there is any issue you see that you might want to try doing, just let a comment on the issue and I will let it to you.

## License

This project is licensed under the `MIT` License. See the LICENSE file for details.
