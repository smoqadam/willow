use notify::{Error, Event, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use crate::models::Rule;

pub fn watch<T>(rules: &Vec<Rule>, callback: T) -> Result<(), Error>
where
    T: Fn(Event),
{
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(rules[0].watch_path.as_str()), RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => callback(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
