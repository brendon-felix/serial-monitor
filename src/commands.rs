// use crate::config::Config;
use crate::settings::Settings;
use anyhow::{Context, Result};
use serialport5;
use std::time::Duration;
use std::fs::{self, File};
// use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use colored::*;
use rfd::FileDialog;

pub fn command_loop(config: Settings) -> Result<()> {
    enable_raw_mode().context("couldn't enabled raw mode")?;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('c') => clear_console(),
                        KeyCode::Char('d') => wipe_output_file(),
                        KeyCode::Char('l') => list_ports(),
                        KeyCode::Char('q') => quit(),
                        KeyCode::Char('s') => save(&config.log_folder),
                        KeyCode::Char('p') => set_port(),
                        KeyCode::Char('h') => help_message(),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn print_separator(text: &str) {
    if let Some((width, _)) = term_size::dimensions() {
        let text_length = text.len();
        match text_length {
            n if n >= width - 2 => println!("{}", text),
            n if n == 0 => {
                let separator = "-".repeat(width);
                println!("{}", separator);
            }
            _ => {
                let side_length = (width - text_length - 2) / 2;
                let separator = "-".repeat(side_length);
                println!("{} {} {}", separator, text, separator);
            }
        }
    } else {
        println!(
            "----------------------- {} -----------------------",
            text
        );
    }
}

fn clear_console() {
    let _ = std::process::Command::new("cmd").args(["/c", "cls"]).status();
}

fn wipe_output_file() {
    File::create("log.txt").expect("Failed to truncate the file");
    print_separator("Output file wiped");
}

fn help_message() {
    print_separator("Help");
    print!
    (
"Press the following keys to execute commands:

- 'c': Clear the console
- 'd': Wipe the output file
- 'l': List available ports
- 'q': Quit the application
- 's': Save current state
- 'p': Set the current port
- 'h': Display this help message
"
    );
    print_separator("");
}

fn list_ports() {
    print_separator("List of connected ports");
    let ports = serialport5::available_ports().expect("Could not list ports");
    if ports.len() == 0 {
        println!("{}", "No ports found!".bold().red());
        return;
    }
    for (index, port) in ports.into_iter().enumerate() {
        println!("{}. {}", index + 1, port.port_name);
    }
    print_separator("");
}

fn quit() {
    disable_raw_mode().expect("Could not diable raw mode");
    std::process::exit(0);
}

fn save(destination_path: &String) {
    print_separator("Save output file");
    // let mut destination_path = String::new();
    // print!("Enter file path: ");
    // io::stdout().flush().unwrap();
    // io::stdin()
    //     .read_line(&mut destination_path)
    //     .expect("Failed to read line");
    // let destination_path = destination_path.trim();
    if let Some(destination_path) = FileDialog::new()
        .add_filter("log", &["txt", "log"])
        .set_title("Save Log File")
        .set_directory(destination_path)
        .set_file_name("log.txt")
        .save_file()
    {
        print_separator("");
        match fs::copy("log.txt", &destination_path) {
            Ok(_) => println!("Saved {}", destination_path.display()),
            Err(e) => println!("Error copying file: {}", e),
        }
    } else {
        println!("Save operation was canceled");
    }

    // let destination_path = "output.txt";
    
    print_separator("");
}

fn set_port() {
    print_separator("Set serial port");
}