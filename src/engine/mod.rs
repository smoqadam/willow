mod pipeline;
mod condition_stage;
mod stability_stage;
mod action_sink;

pub use pipeline::{Stage, Sink};
pub use condition_stage::ConditionStage;
pub use stability_stage::StabilityStage;
pub use action_sink::ActionSink;

use crate::models::{Config, RuntimeWatcher, RuntimeRule};
use std::sync::{mpsc, Arc};
use std::thread;
use log::info;

pub fn start(config: &Config) -> anyhow::Result<()> {
    // Channels between pipeline stages
    let (cond_tx, cond_rx) = mpsc::channel();
    let (stab_tx, stab_rx) = mpsc::channel();
    let (act_tx, act_rx) = mpsc::channel();

    // Create and spawn pipeline stages
    let mut condition_stage = ConditionStage::new();
    let mut stability_stage = StabilityStage::new();
    let mut action_sink = ActionSink::new();

    // Spawn stage threads
    thread::spawn(move || {
        condition_stage.run(cond_rx, stab_tx);
    });

    thread::spawn(move || {
        stability_stage.run(stab_rx, act_tx);
    });

    thread::spawn(move || {
        action_sink.run(act_rx);
    });

    // For each watcher, just forward events + rules into the pipeline
    for watcher_config in &config.watchers {
        // Convert each rule's conditions and actions to trait objects
        let mut runtime_rules: Vec<Arc<RuntimeRule>> = Vec::new();
        
        for rule in &watcher_config.rules {
            // Convert ConditionConfig to trait objects
            let mut conditions: Vec<Box<dyn crate::conditions::Condition>> = Vec::new();
            for condition_config in &rule.conditions {
                conditions.push(condition_config.clone().into_condition()?);
            }

            // Convert ActionConfig to trait objects
            let mut actions: Vec<Box<dyn crate::actions::Action>> = Vec::new();
            for action_config in &rule.actions {
                actions.push(action_config.clone().into_action());
            }

            runtime_rules.push(Arc::new(RuntimeRule {
                conditions,
                actions,
            }));
        }

        let runtime_watcher = RuntimeWatcher {
            path: watcher_config.path.clone().into(),
            recursive: watcher_config.recursive,
            ignore: watcher_config.ignore.clone(),
            rules: runtime_rules,
        };

        let cond_tx_clone = cond_tx.clone();

        thread::spawn(move || {
            let (rx, _debouncer) = runtime_watcher.watch().expect("failed to start watcher");
            while let Ok(ev) = rx.recv() {
                info!("raw event {:?}", ev);
                // pass event + relevant rules into the pipeline
                if cond_tx_clone.send((ev, runtime_watcher.rules.clone())).is_err() {
                    break; // Channel closed, exit gracefully
                }
            }
        });
    }

    Ok(())
}
