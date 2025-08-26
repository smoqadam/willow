use crate::action::ActionContext;
use crate::models::{Config};
use log::{error, info};
use std::thread;
use crate::watcher::RuntimeWatcher;

pub fn start(config: &Config) -> anyhow::Result<()> {
    for watcher_config in &config.watchers {
        // Convert ConditionConfig to trait objects
        let mut conditions: Vec<Box<dyn crate::conditions::Condition>> = Vec::new();
        for condition_config in &watcher_config.conditions {
            conditions.push(condition_config.clone().into_condition()?);
        }

        let runtime_watcher = RuntimeWatcher {
            path: watcher_config.path.clone(),
            recursive: watcher_config.recursive,
            actions: watcher_config.actions.clone(),
            conditions,
        };

        thread::spawn(move || {
            let (rx, _debouncer) = runtime_watcher.watch().expect("failed to start watcher");
            while let Ok(ev) = rx.recv() {
                info!("ev: {:?}", ev);
                
                let all: bool = runtime_watcher.conditions.iter().all(|c| c.matches(&ev));
                if all {
                    info!("all conditions matched for {}", runtime_watcher.path);
                    for action in &runtime_watcher.actions {
                        let exec = action.clone().into_exec();
                        if let Err(e) = exec.run(&ActionContext {
                            path: &ev.path,
                            event: &ev.event,
                        }) {
                            error!("Action failed on {}: {:?}", ev.path.display(), e);
                            break;
                        }
                    }
                }
            }
        });
    }

    Ok(())
}
