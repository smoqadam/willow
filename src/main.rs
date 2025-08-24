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
#[derive(Parser, Debug)]
#[command(name = "willow", version, about = "Watch a directory for file changes", long_about = None)]
pub struct Cli {
    /// Optional config file
    #[arg(short, long)]
    pub config: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load(cli.config.unwrap())?;

    let (_w, rx) = watcher::watch(&config)?;
    for event_info in rx {
        println!("{:?}", event_info);
        let matched_rules = rules::from_event(&event_info, &config);
        for rule in matched_rules {
            println!("Rule matched: {:?}", rule.event);
            for action in &rule.actions {
                if let Err(e) = action.run(&ActionContext {
                    paths: &event_info.paths,
                    event: &event_info.event,
                }) {
                    eprintln!("Action failed: {}", e);
                }
            }
        }
    }

    Ok(())
}
