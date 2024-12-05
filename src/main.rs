mod commands;
mod serial;
mod settings;
use crate::settings::get_settings;
use crate::commands::command_loop;
use anyhow::Result;

fn main() -> Result<()> {
    let settings = get_settings();
    let settings_cpy = settings.clone();
    std::thread::spawn(move || {
        if let Err(_) = command_loop(settings_cpy) {
            std::process::exit(1);
        }
    });
    serial::open(settings)?;
    Ok(())
}
