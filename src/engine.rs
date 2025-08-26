use crate::actions::ActionContext;
use crate::models::{Config, Watcher};
use log::{error, info};
use std::collections::HashMap;
use std::thread;

pub fn start(config: &Config) -> anyhow::Result<()> {
    for watcher in &config.watchers {
        let w = watcher.clone();
        thread::spawn(move || {
            let (rx, _debouncer) = w.watch().expect("failed to start watcher");
            while let Ok(ev) = rx.recv() {
                info!("ev: {:?}", ev);
                w.conditions
                //
                // let all: bool = w.conditions.iter().all(|c| c.matches(&ev));
                // if all {
                //     info!("all conditions matched for {}", w.path);
                //     for action in &w.actions {
                //         let exec = action.clone().into_exec(); // assuming into_exec() returns some trait object
                //         if let Err(e) = exec.run(&ActionContext {
                //             path: &ev.path,
                //             event: &ev.event,
                //         }) {
                //             error!("Action failed on {}: {:?}", ev.path.display(), e);
                //             break;
                //         }
                //     }
                // }
            }
        });
    }

    Ok(())
}
