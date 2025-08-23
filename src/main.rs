mod actions;
mod conditions;
mod config;
mod debouncer;
mod matcher;
mod models;
mod rules;
mod watcher;

use actions::*;
use anyhow::Result;
use clap::Parser;
use debouncer::Debouncer;
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

    let (_watcher, rx) = watcher::watch(&config)?;
    let mut db = Debouncer::new();

    for event in rx {
        db.push(event);
        if db.flush_if_ready() {
            let normalized_events = db.drain();
            for norm_event in normalized_events {
                let event_info = models::EventInfo {
                    paths: vec![norm_event.path],
                    event: norm_event.event,
                };
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
        }
    }

    Ok(())
}
