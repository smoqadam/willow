use notify::{Error, Event, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use crate::models::{Config, Rule};

pub fn watch<T>(config: &Config, callback: T) -> Result<(), Error>
where
    T: Fn(Event),
{
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;

    for rule in &config.rules {
        let path = Path::new(rule.watch.as_str());
        watcher.watch(path, RecursiveMode::Recursive)?;
    }

    for res in rx {
        match res {
            Ok(event) => callback(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
