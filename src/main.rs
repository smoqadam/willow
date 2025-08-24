mod actions;
mod conditions;
mod config;
mod matcher;
mod models;
mod rules;
mod watcher;

use actions::*;
use anyhow::Result;
use clap::Parser;
use log::{Level, debug, error, info, log_enabled};

#[derive(Parser, Debug)]
#[command(name = "willow", version, about = "Watch a directory for file changes", long_about = None)]
pub struct Cli {
    /// Optional config file
    #[arg(short, long)]
    pub config: Option<String>,
}



fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);
    
    let config = config::load(cli.config.unwrap())?;
    info!("Configuration loaded successfully");

    let (_w, rx) = watcher::watch(&config)?;
    info!("Started watching directories, waiting for file events...");
    
    for event_info in rx {
        debug!("Received event: {:?} for path: {:?}", event_info.event, event_info.path);
        let matched_rules = rules::from_event(&event_info, &config);
        debug!("Found {} matching rules for event", matched_rules.len());
        
        for rule in matched_rules {
            info!("Rule matched: {:?}", rule.event);
            for action in &rule.actions {
                // debug!("Executing action: {:?}", action);
                if let Err(e) = action.run(&ActionContext {
                    path: &event_info.path,
                    event: &event_info.event,
                }) {
                    eprintln!("Action failed: {}", e);
                }
            }
        }
    }

    Ok(())
}
