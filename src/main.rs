mod action;
mod actions;
mod condition;
mod conditions;
mod config;
mod engine;
mod fs;
mod models;
mod template;
mod watcher;

use anyhow::Result;
use clap::Parser;
use log::debug;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "willow", version, about = "Watch a directory for file changes", long_about = None)]
pub struct Cli {
    /// Optional config file
    #[arg(short, long)]
    pub config: String,
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {cli:?}");

    let config = config::load(cli.config)?;
    debug!("Parsed CLI arguments: {config:?}");

    let handle = if cli.dry_run {
        use crate::fs::{DryRunFs, Fs, StdFs};
        engine::start_with_fs(
            &config,
            Arc::new(DryRunFs::new(Arc::new(StdFs::new()) as Arc<dyn Fs>)),
        )?
    } else {
        engine::start(&config)?
    };
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("ctrlc");
    let _ = rx.recv();
    handle.shutdown();
    Ok(())
}
