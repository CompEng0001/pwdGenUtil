# Password Generator in Rust

A secure password generator written in Rust, with options for clipboard copying and command-line arguments. This tool generates passwords that avoid similar characters, duplicate characters, and sequential characters to maximize security and usability.

## Features

- **Random Password Generation**: Generates passwords with a specified length, default 16, using a secure random generator.
- **No Similar Characters**: Excludes characters like `0`, `O`, `I`, and `l` to avoid confusion.
- **No Duplicate or Sequential Characters**: Ensures passwords are unique and non-sequential for increased security.
- **Clipboard Support**: Automatically copies generated password to the clipboard.
- **Verbose Mode**: Optionally display the generated password in the terminal.

## Installation

1. Clone the repository:
    
    ```bash
    git clone https://github.com/yourusername/pwdgenUtil.git
    cd pwdgenUtil
    ```

2. Build the project using Cargo:

    ```bash
    cargo build --release
    ```

## Usage

- Run the program with default settings (password copied to clipboard):

    ```bash
    ./target/release/pwdgenUtil
    ```

- With options

    ```
    ./target/release/pwdgenUtil -l 20 -v
    ```

## Requirements

- **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).
- **Dependencies**:
    - rand (version 0.8): For random number generation. 
    - clipboard (version 0.5): To copy passwords to the clipboard.
    - clap (version 4.0): For command-line argument parsing.

## Contributing
Feel free to fork this project, submit issues, or make pull requests to contribute. All contributions are welcome!

## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.
