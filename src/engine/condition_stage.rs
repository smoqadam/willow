use crate::engine::pipeline::Stage;
use crate::models::{EventInfo, RuntimeRule};
use std::sync::{mpsc::{Receiver, Sender}, Arc};
use log::info;

pub struct ConditionStage;

impl ConditionStage {
    pub fn new() -> Self {
        Self
    }
}

impl Stage for ConditionStage {
    fn run(
        &mut self,
        rx: Receiver<(EventInfo, Vec<Arc<RuntimeRule>>)>,
        tx: Sender<(EventInfo, Vec<Arc<RuntimeRule>>)>,
    ) {
        while let Ok((ev, rules)) = rx.recv() {
            // filter rules: only keep those that all conditions match
            let matching: Vec<_> = rules.into_iter()
                .filter(|r| r.conditions.iter().all(|c| c.matches(&ev.path)))
                .collect();

            if !matching.is_empty() {
                info!("conditions matched for {:?}: {} rules", ev.path, matching.len());
                if tx.send((ev, matching)).is_err() {
                    break; // Channel closed, exit gracefully
                }
            }
        }
    }
}
