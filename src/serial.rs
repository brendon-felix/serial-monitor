use crate::args::Args;
use anyhow::{bail, Context, Result};
use log::{info, warn};
use serialport5::{self, SerialPort, SerialPortBuilder, SerialPortInfo, SerialPortType};
use std::io::{self, BufWriter, BufReader, BufRead, Read, Write};
use std::fs::{self, OpenOptions, File};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use chrono::Local;

pub fn find_by_usb_info(args: &Args) -> Result<Option<SerialPortInfo>> {
    let ports = serialport5::available_ports().unwrap();
    for port in ports {
        let port_clone = port.clone();
        match port.port_type {
            SerialPortType::UsbPort(info) => {
                let pid = format!("{:04x}", info.pid);
                let vid = format!("{:04x}", info.vid);
                if args.pid.clone().unwrap_or_default() == pid {
                    return Ok(Some(port_clone));
                } else if args.vid.clone().unwrap_or_default() == vid {
                    return Ok(Some(port_clone));
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

pub fn open_serial_port(args: &Args) -> Result<(SerialPort, String)> {
    let port_name = if args.pid.is_some() || args.vid.is_some() {
        find_by_usb_info(&args)?.map(|port_info| port_info.port_name)
    } else {
        args.port.clone()
    };

    let port_name = port_name.clone().context("Port not found!")?;
    let baud_rate = args.baud_rate.context("No baud rate specified!")?;
    info!("open port {:?} with rate of {}", port_name, baud_rate);
    let port = SerialPortBuilder::new()
        .baud_rate(baud_rate)
        .open(port_name.clone())?;
    // let _ = std::process::Command::new("cmd").args(["/c", "cls"]).spawn();
    Ok((port, port_name))
}

pub fn read_stdin_loop(_port: Arc<Mutex<SerialPort>>, _port_name: &str) -> Result<()> {
    enable_raw_mode()?;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            // Read the key event
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                // Respond only to key press actions
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('c') => {
                            // Clear console output
                            let _ = std::process::Command::new("cmd").args(["/c", "cls"]).spawn();
                        }
                        KeyCode::Char('d') => {
                            // Truncate the file to delete its contents
                            File::create("output.txt").expect("Failed to truncate the file");
                            println!("Output file contents deleted");
                        }
                        KeyCode::Char('s') => {
                            println!("Save");
                        }
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            std::process::exit(0);
                        }
                        KeyCode::Char('p') => {
                            println!("Set Port");
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn read_serial_loop<W: Write>(
    port: Arc<Mutex<SerialPort>>,
    stdout: &mut W,
    file: &mut W,
    flush_stdout: bool,
    port_name: &str,
) -> Result<()> {
    let mut buffer = Vec::new();
    let ansi_escape = Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
    
    loop {
        let mut port = port.lock().unwrap();
        let mut data = [0; 128]; // Smaller buffer for each read
        match port.read(&mut data) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                // Append new data to the buffer
                buffer.extend_from_slice(&data[..n]);
                
                // Convert buffer to a string to process lines
                if let Ok(text) = String::from_utf8(buffer.clone()) {
                    // Create a buffered reader to read lines
                    let mut reader = BufReader::new(text.as_bytes());
                    let mut line = String::new();

                    while reader.read_line(&mut line)? > 0 {
                        // Only process if a full line is read (ends with a newline)
                        if line.ends_with('\n') {
                            // Remove unwanted ANSI codes
                            stdout.write_all(line.as_bytes())
                                .context("Failed to write to stdout")?;
                            
                            let filtered_line = ansi_escape.replace_all(&line, "");
                            let timestamp = Local::now().format("%H:%M:%S.%3f");
                            let timed_line = format!("[{}] {}", timestamp, filtered_line);

                        
                            file.write_all(timed_line.as_bytes())
                                .context("Failed to write to file")?;

                            if flush_stdout {
                                stdout.flush().context("Failed to flush stdout")?;
                            }

                            // Clear the line buffer for the next line
                            line.clear();
                        }
                    }

                    // Remaining content in line buffer might be an incomplete line
                    buffer = line.into_bytes(); // Resave remaining content back to the buffer for next read
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                warn!("Failed to read from {}: {}", port_name, e);
                bail!("");
            }
        }
    }
}

pub fn get_stdout_with_buffer_size(args: &Args) -> Box<dyn Write> {
    if let Some(buf_size) = args.buf_size {
        Box::new(BufWriter::with_capacity(buf_size, io::stdout()))
    } else {
        Box::new(BufWriter::with_capacity(1048576, io::stdout()))
        // Box::new(io::stdout())
    }
}

pub fn open_with_reconnect(args: &Args) -> Result<()> {
    let mut retry_count = 0;

    let mut stdout = get_stdout_with_buffer_size(args);

    // Open the file for writing if the path is provided in args
    let file_path = "output.txt";
    /* -------------------------------------------------------------------------- */
    // remove this
    if fs::metadata(file_path).is_ok() {
        fs::remove_file(file_path).context("Failed to remove existing output file")?;
    }
    /* -------------------------------------------------------------------------- */
    let mut file: Box<dyn Write> = Box::new(OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .context("Failed to open output file")?);


    loop {
        let result = open_serial_port(&args);
        match result {
            Ok((port, name)) => {
                let port_arc = Arc::new(Mutex::new(port.try_clone()?));
                let port_arc_clone = port_arc.clone();

                // Spawn a thread to read from stdin and write to the serial port.
                let name_clone = name.clone();
                std::thread::spawn(move || {
                    if let Err(_) = read_stdin_loop(port_arc_clone, &name_clone) {
                        std::process::exit(1);
                    }
                });

                // Read from serial port and write to stdout in the main thread.
                match read_serial_loop(port_arc, &mut stdout, &mut file, args.flush, &name) {
                    Ok(_) => {
                        // Successful read, break out of the loop
                        break;
                    }
                    Err(_) => {
                        // Reconnect
                        // Delay before attempting the next reconnect
                        std::thread::sleep(Duration::from_secs(1));

                        // Decrease the retry count
                        retry_count -= 1;

                        // Log a message or take any other necessary action
                        log::warn!("Reconnecting... Retries left: {}", retry_count);
                    }
                }
            }
            _ => {
                retry_count += 1;
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    }
    Ok(())
}

// pub fn open(args: &Args) -> Result<()> {
//     // Connect normally without reconnection logic
//     let (port, name): (SerialPort, String) = open_serial_port(&args)?;
//     let port_arc = Arc::new(Mutex::new(port));

//     let port_arc_clone = port_arc.clone();

//     // Spawn a thread to read from stdin and write to the serial port.
//     let name_clone = name.clone();
//     std::thread::spawn(move || {
//         if let Err(_) = read_stdin_loop(port_arc_clone, &name_clone) {
//             std::process::exit(1);
//         }
//     });

//     let mut stdout = BufWriter::new(std::io::stdout());

//     // Read from serial port and write to stdout in the main thread.
//     match read_serial_loop(port_arc, &mut stdout, args.flush, &name) {
//         Err(_) => {
//             // Handle any specific error logic if needed
//         }
//         _ => {}
//     }
//     Ok(())
// }
