use anyhow::{Result, Context};
use std::fs;
// use std::io::{self, Write};
use toml;
// use serde::{Deserialize, Serialize};
use serde::Deserialize;

// #[derive(Deserialize, Serialize, Clone)]
#[derive(Deserialize, Clone)]
pub struct Config {
    pub port: String,
    pub baud_rate: u32,
    pub log_folder: String,
}

pub fn get_config(filename: String) -> Result<Config>{
    let toml_str = fs::read_to_string(filename).context("Could not read config file")?;
    let config: Config = toml::from_str(&toml_str).context("Could not serialize config file")?;
    Ok(config)
}

// pub fn set_config(filename: String) -> Result<()> {

// }