mod watcher;
mod config;
mod models;
mod matcher;
mod actions;
mod conditions;
mod rules;

use clap::Parser;
use notify::Error;

#[derive(Parser, Debug)]
#[command(name = "willow", version, about = "Watch a directory for file changes", long_about = None)]
pub struct Cli {
    /// Optional config file
    #[arg(short, long)]
    pub config: Option<String>,
}


fn main() -> Result <(), Error>{
    let cli = Cli::parse();
    let config = config::load(cli.config.unwrap())?;
    
    let (_watcher, rx) = watcher::watch(&config)?;
    for event in rx {
        let matched_rules = rules::from_event(&event, &config);
        for rule in matched_rules {
            println!("Rule matched: {:?}", rule);
        }
    }

    Ok(())
}