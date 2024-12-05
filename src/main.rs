mod commands;
mod serial;
mod config;
use crate::config::get_config;
use crate::commands::command_loop;
use anyhow::{Result, Context};
use env_logger;

fn main() -> Result<()> {
    env_logger::init();
    let config = get_config("config.toml".to_string())
        .context("Could not find configuration file `config.toml`")?;
    let config_cpy = config.clone();
    std::thread::spawn(move || {
        if let Err(_) = command_loop(config_cpy) {
            std::process::exit(1);
        }
    });
    serial::open(config)?;
    Ok(())
}
