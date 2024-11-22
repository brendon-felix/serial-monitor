use anyhow::{bail, Result};
use std::fs;
use std::io::{self, Write};
use toml;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    port: String,
    // baud_rate: u32,
    // reconnect: bool,
    // buffer_size: u32,
}

//update to use a filename parameter
pub fn get_config(file: String) -> Result<Config, Box<dyn std::error::Error>>{
    let toml_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}