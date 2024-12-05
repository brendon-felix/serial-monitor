use clap::Parser;
use std::fs;
use toml;
use serde::Deserialize;

// #[command(author, version, about, long_about = None)]
#[derive(Parser, Debug)]
struct Args {
    /// port name (eg. COM23 OR /dev/ttyUSB0)
    #[arg(short, long)]
    port: Option<String>,

    /// baud rate
    #[arg(short, long)]
    baud_rate: Option<u32>,

    /// log folder path
    #[arg(short, long)]
    log_folder: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    port: Option<String>,
    baud_rate: Option<u32>,
    log_folder: Option<String>,
}

#[derive(Clone)]
pub struct Settings {
    pub port: String,
    pub baud_rate: u32,
    pub log_folder: String,
}

fn get_config(filename: String) -> Option<Config> {
    let toml_str = fs::read_to_string(filename).ok()?;
    toml::from_str(&toml_str).ok()
}

pub fn get_settings() -> Settings {
    let args = Args::parse();
    let config = get_config("config.toml".to_string()).unwrap();
    Settings {
        port: args.port.or(config.port).unwrap_or("COM5".to_string()),
        baud_rate: args.baud_rate.or(config.baud_rate).unwrap_or(115200),
        log_folder: args.log_folder.or(config.log_folder)
            .unwrap_or(r"C:\Users\felixb\OneDrive - HP Inc\Debugs\Springs".to_string()),
    }
}