use crate::settings::Settings;
use anyhow::{bail, Context, Result};
use serialport5::{self, SerialPort, SerialPortBuilder};
use std::io::{self, BufWriter, BufReader, BufRead, Read, Write};
use std::fs::{self, File};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::time::Duration;
use colored::*;
use chrono::Local;

pub fn open_serial_port(config: &Settings) -> Result<(SerialPort, String)> {
    let port_name = config.port.clone();
    let baud_rate = config.baud_rate;
    let port = SerialPortBuilder::new()
        .baud_rate(baud_rate)
        .open(port_name.clone())?;
    Ok((port, port_name))
}

pub fn read_serial_loop<W: Write>(
    port: Arc<Mutex<SerialPort>>,
    timestamps: bool,
    stdout: &mut W,
    file: &mut File,
) -> Result<()> {
    let mut buffer = Vec::new();
    let ansi_escape = Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
    loop {
        let mut port = port.lock().unwrap();
        let mut data = [0; 256];
        match port.read(&mut data) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                buffer.extend_from_slice(&data[..n]);
                if let Ok(text) = String::from_utf8(buffer.clone()) {
                    let mut reader = BufReader::new(text.as_bytes());
                    let mut line = String::new();
                    while reader.read_line(&mut line)? > 0 {
                        if line.ends_with('\n') {
                            stdout.write_all(line.as_bytes())
                                .context("Failed to write to stdout")?;
                                
                            let mut output_line = ansi_escape.replace_all(&line, "").to_string();
                            if timestamps {
                                let timestamp = Local::now().format("%H:%M:%S.%3f");
                                output_line = format!("[{}] {}", timestamp, output_line);
                            }

                            file.write_all(output_line.as_bytes())
                                .context("Failed to write to file")?;
                            stdout.flush().context("Failed to flush stdout")?;
                            line.clear();
                        }
                    }
                    buffer = line.into_bytes();
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                println!("Failed to read port: {}", e);
                bail!("");
            }
        }
    }
}

fn new_log(file_path: String) -> Result<File> {
    // remove temporary log if it exists
    if fs::metadata(&file_path).is_ok() {
        fs::remove_file(&file_path).context("Failed to remove existing output file")?;
    }

    let file = File::create(file_path).context("Failed to open output file")?;
    Ok(file)
}

pub fn open(config: Settings) -> Result<()> {
    let mut try_reconnect = false;
    let mut stdout = Box::new(BufWriter::with_capacity(1024, io::stdout()));
    let mut file = new_log("log.txt".to_string()).context("Can't open output file")?;

    loop {
        let result = open_serial_port(&config);
        match result {
            Ok((port, name)) => {
                if config.clear_on_start {
                    let _ = std::process::Command::new("cmd").args(["/c", "cls"]).status(); // clear console
                }
                let connect_msg = format!("{} connected", name);
                println!("{}", connect_msg.bold().green());
                let port_arc = Arc::new(Mutex::new(port.try_clone()?));
                match read_serial_loop(port_arc, config.timestamps, &mut stdout, &mut file) {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
            _ => {
                if !try_reconnect {
                    println!("{}", "Not connected".bold().red());
                    try_reconnect = true;
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
    Ok(())
}
