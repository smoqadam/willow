mod watcher;
mod config;
mod models;
mod matcher;
mod actions;

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
    // parse config
    let config = config::load(cli.config.unwrap())?;

    // println!("{:?}", config);
    // watcher::start(config);

    //
    // // watch for events and
    // watcher::watch(&config, |event| {
    //     matcher::apply(&config, &event);
    // })?;

    Ok(())
}