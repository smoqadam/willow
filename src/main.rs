mod watcher;
mod rule;
mod models;
mod matcher;

use clap::Parser;
use notify::Error;

#[derive(Parser, Debug)]
#[command(name = "willow", version, about = "Watch a directory for file changes", long_about = None)]
pub struct Cli {
    /// Target directory to watch
    pub target: String,

    /// Optional config file
    #[arg(short, long)]
    pub config: Option<String>,
}


fn main() -> Result <(), Error>{
    let cli = Cli::parse();
    // parse config

    let rules = rule::load(cli.target);

    // watch for events and
    watcher::watch(&rules, |event| {
        matcher::apply(&rules, &event);
    })?;

    Ok(())
}