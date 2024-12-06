# spewcap

Basic serial monitor program to capture serial spew (BIOS spew, Super I/O spew, etc). I kinda hate using TeraTerm, so this is intended to replace that.

## Features

- Use the TOML file or CLI arguments to configure the application (CLI arguments take precedent)
- Easy-to-use key commands (`C` to clear, `S` to save, `Q` to quit, etc.)

## Todo

- [ ] Add serial input

## Prerequisites

- Rust and Cargo installed: [Install Rust](https://www.rust-lang.org/tools/install)
- A USB serial port device (usually UART)

## Installation

1. Clone this repository:
    ```sh
    git clone https://github.com/yourusername/spewcap.git # UPDATE LINK!
    cd spewcap
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Place the executable and TOML file in another location:
    ```sh
    cp .\config.toml C:\Program Files\serust
    ```

## Usage

1. Change settings in `cargo.toml` (optional)

2. Start the program from a terminal/shell

    With `config.toml` configured

    ```sh
    ./spewcap.exe
    ```

    With command line option flags (override `config.toml` if it exists)

    ```sh
    ./spewcap.exe -p <port-name> -b <baud-rate>
    ```

    To view all options use `-h` or `--help`

    ```sh
    ./spewcap.exe -h
    ```
    > Tip: In Windows Terminal settings, under 'Defaults > Advanced', increase 'History size' to 100,000 and save

    > Use `CTRL+SHIFT+F` to search spew without saving a log

3. Press `L` to list avaiable ports and `H` to view all key commands

4. Quit the program using `Q`

## Example

```sh
./spewcap.exe -p COM5 -b 115200
```
