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
## Usage

### Commands

- Activate Virtual Environment

  `pen activate`

- Deactivate Virtual Environment

  `pen deactivate`

- Create Virtual Environment

  `pen create --pyversion=VERSION`

- Show the help message
  
  `pen --help`, `pen -h` or `pen`

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

## License

This project is licensed under the `MIT` License. See the LICENSE file for details.
