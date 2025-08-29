mod actions;
mod condition;
mod config;
mod models;
mod watcher;
mod engine;
mod action;
mod conditions;
mod template;
mod fs;

use std::thread;
use anyhow::Result;
use clap::Parser;
use log::debug;

#[derive(Parser, Debug)]
#[command(name = "willow", version, about = "Watch a directory for file changes", long_about = None)]
pub struct Cli {
    /// Optional config file
    #[arg(short, long)]
    pub config: String,
}



fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);
    
    let config = config::load(cli.config)?;
    debug!("Parsed CLI arguments: {:?}", config);

    engine::start(&config)?;
    loop {
        thread::park();
    }
}
