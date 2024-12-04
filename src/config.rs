use anyhow::{Result, Context};
use std::fs;
// use std::io::{self, Write};
use toml;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub port: String,
    pub baud_rate: u32,
    // reconnect: bool,
    // pub buffer_size: usize,
}

pub fn get_config(filename: String) -> Result<Config>{
    let toml_str = fs::read_to_string(filename).context("could not read config file")?;
    let config: Config = toml::from_str(&toml_str).context("could not serialize config file")?;
    Ok(config)
}

// pub fn set_config(filename: String) -> Result<()> {

// }