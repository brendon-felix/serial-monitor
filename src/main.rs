mod actions;
mod args;
mod serial;
mod config;
use config::Config;
use anyhow::Result;
use args::Args;
use clap::Parser;
use env_logger;

fn main() -> Result<()> {
    env_logger::init();
    // let args = Args::parse();
    // args.validate()?;
    // if args.list {
    //     actions::list_ports()?;
    // } else if args.reconnect.unwrap_or_default() {
    //     serial::open_with_reconnect(&args)?;
    // } else {
    //     // serial::open(&args)?;
    //     serial::open_with_reconnect(&args)?;
    // }
    let args = Args{
        port: Some("COM5".to_string()),
        pid: None,
        vid: None,
        baud_rate: Some(115200),
        reconnect: Some(true),
        list: false,
        buf_size: Some(1048576),
        flush: true,
    };
    args.validate()?;
    serial::open_with_reconnect(&args)?;
    Ok(())
}
