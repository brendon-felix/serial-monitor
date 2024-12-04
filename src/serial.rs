use crate::config::Config;
use anyhow::{bail, Context, Result};
use serialport5::{self, SerialPort, SerialPortBuilder};
use std::io::{self, BufWriter, BufReader, BufRead, Read, Write};
use std::fs::{self, OpenOptions};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::time::Duration;
use colored::*;
// use chrono::Local;

pub fn open_serial_port(config: &Config) -> Result<(SerialPort, String)> {
    let port_name = config.port.clone();
    let baud_rate = config.baud_rate;
    let port = SerialPortBuilder::new()
        .baud_rate(baud_rate)
        .open(port_name.clone())?;
    Ok((port, port_name))
}

pub fn read_serial_loop<W: Write>(
    port: Arc<Mutex<SerialPort>>,
    stdout: &mut W,
    file: &mut Box<dyn Write>,
) -> Result<()> {
    let mut buffer = Vec::new();
    let ansi_escape = Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
    loop {
        let mut port = port.lock().unwrap();
        let mut data = [0; 256]; // Smaller buffer for each read
        match port.read(&mut data) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                buffer.extend_from_slice(&data[..n]);
                if let Ok(text) = String::from_utf8(buffer.clone()) {
                    let mut reader = BufReader::new(text.as_bytes());
                    let mut line = String::new();
                    while reader.read_line(&mut line)? > 0 {
                        if line.ends_with('\n') {
                            // Output to console
                            stdout.write_all(line.as_bytes())
                                .context("Failed to write to stdout")?;
                            
                            // Output to file
                            let output_line = ansi_escape.replace_all(&line, "");   // Remove unwanted ANSI codes
                            // let timestamp = Local::now().format("%H:%M:%S.%3f");
                            // let output_line = format!("[{}] {}", timestamp, output_line);    // Attach timestamp
                            file.write_all(output_line.as_bytes())
                                .context("Failed to write to file")?;
                            stdout.flush().context("Failed to flush stdout")?;
                            line.clear(); // Clear the line buffer for the next line
                        }
                    }
                    buffer = line.into_bytes(); // Resave remaining content back to the buffer for next read
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

fn new_output_file(file_path: String) -> Result<Box<dyn Write>> {
    /* ------------------------------- REMOVE THIS ------------------------------ */
    if fs::metadata(&file_path).is_ok() {
        fs::remove_file(&file_path).context("Failed to remove existing output file")?;
    }
    /* -------------------------------------------------------------------------- */

    let file: Box<dyn Write> = Box::new(OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .context("Failed to open output file")?);
    
    Ok(file)

}

pub fn open(config: Config) -> Result<()> {
    let mut try_reconnect = false;
    let mut stdout = Box::new(BufWriter::with_capacity(1024, io::stdout()));
    let mut file = new_output_file("temp.txt".to_string()).expect("Can't open output file");

    loop {
        let result = open_serial_port(&config);
        match result {
            Ok((port, name)) => {
                // let _ = std::process::Command::new("cmd").args(["/c", "cls"]).status(); // clear console
                let connect_msg = format!("{} connected", name);
                println!("{}", connect_msg.bold().green());
                let port_arc = Arc::new(Mutex::new(port.try_clone()?));
                match read_serial_loop(port_arc, &mut stdout, &mut file) {
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
