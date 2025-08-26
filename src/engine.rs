use crate::models::{Config, RuntimeWatcher};
use log::{error, info};
use std::thread;

pub fn start(config: &Config) -> anyhow::Result<()> {
    for watcher_config in &config.watchers {
        // Convert each rule's conditions and actions to trait objects
        let mut runtime_rules: Vec<crate::models::RuntimeRule> = Vec::new();
        
        for rule in &watcher_config.rules {
            // Convert ConditionConfig to trait objects
            let mut conditions: Vec<Box<dyn crate::conditions::Condition>> = Vec::new();
            // Add the event condition first
            conditions.push(Box::new(crate::conditions::EventCondition::new(rule.event.clone())));
            // Then add the other conditions
            for condition_config in &rule.conditions {
                conditions.push(condition_config.clone().into_condition()?);
            }

            // Convert ActionConfig to trait objects
            let mut actions: Vec<Box<dyn crate::actions::Action>> = Vec::new();
            for action_config in &rule.actions {
                actions.push(action_config.clone().into_action());
            }

            runtime_rules.push(crate::models::RuntimeRule {
                conditions,
                actions,
            });
        }

        let runtime_watcher = RuntimeWatcher {
            path: watcher_config.path.clone(),
            recursive: watcher_config.recursive,
            rules: runtime_rules,
        };

        thread::spawn(move || {
            let (rx, _debouncer) = runtime_watcher.watch().expect("failed to start watcher");
            while let Ok(ev) = rx.recv() {
                info!("ev: {:?}", ev);
                
                // check each rule to see if its conditions match
                for rule in &runtime_watcher.rules {
                    // all conditions must match
                    let all_conditions_match: bool = rule.conditions.iter().all(|c| c.matches(&ev));
                    if all_conditions_match {
                        info!("all conditions matched for rule in watcher {}", runtime_watcher.path);
                        // execute all actions for this rule
                        for action in &rule.actions {
                            if let Err(e) = action.run(&ev) {
                                error!("Action failed on {}: {:?}", ev.path.display(), e);
                                break;//todo: should it break or continue?
                            }
                        }
                    }
                }
            }
        });
    }

    Ok(())
}
