# SpewCap

Basic serial monitor program to capture serial spew (BIOS spew, Super I/O spew, etc) written in the *best* programming language. I kinda hate using TeraTerm, so this is intended to replace that.

## Features

- [x] Use the TOML file or CLI arguments to configure the application (CLI arguments take precedent)
- [x] Easy-to-use key commands (`C` to clear, `Q` to quit, etc.)
- [x] Save logs using the built-in file explorer (use `S` to save)
- [ ] Serial input (for host TX)

## Installation

1. Clone this repository
    ```sh
    git clone https://github.azc.ext.hp.com/wks-fw/SpewCap.git
    cd spewcap
    ```

2. Build the project (optional, requires Rust and Cargo)

    - [Install Rust](https://www.rust-lang.org/tools/install)

    ```sh
    cargo build --release
    ```

3. Place the executable and TOML file in another location
    - If building from source:
        ```sh
        cp .\target\release\spewcap.exe 'C:\Program Files\SpewCap\'
        cp .\config.toml 'C:\Program Files\SpewCap\'
        ```
    - If using an existing release:
        ```sh
        cp .\Releases\v0.10.0\spewcap.exe 'C:\Program Files\SpewCap\'
        cp .\config.toml 'C:\Program Files\SpewCap\'
        ```

4. Right click on spewcap.exe to create a shortcut, then add the shortcut to the Start menu folder
    > Tip: Place the shortcut file here: `C:\ProgramData\Microsoft\Windows\Start Menu\Programs\SpewCap\`

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

### With command line option flags

```sh
./spewcap.exe -p COM5 -b 115200
```

### With `config.toml` configured

```toml
# config.toml

port = 'COM5'
baud_rate = 115200
log_folder = 'C:\Users\username\Logs'
clear_on_start = true
```
```sh
./spewcap.exe
```