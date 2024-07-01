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

- Show Help Message
  
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


## Quick temporary todos (ignore if not developping pen):

If you want a release build with all stable optimizations active (PGO, etc),
please run ./configure --enable-optimizations

WARNING: The scripts pip3 and pip3.11 are installed in '/home/azom/.pen/pythonVersions/python_3.11.9/bin' which is not on PATH.
  Consider adding this directory to PATH or, if you prefer to suppress this warning, use --no-warn-script-location.

Do shortcuts like pen a for pen activate
