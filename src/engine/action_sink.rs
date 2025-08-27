use crate::engine::pipeline::Sink;
use crate::models::{EventInfo, RuntimeRule};
use std::sync::{mpsc::Receiver, Arc};
use log::error;

pub struct ActionSink;

impl ActionSink {
    pub fn new() -> Self {
        Self
    }
}

impl Sink for ActionSink {
    fn run(&mut self, rx: Receiver<(EventInfo, Vec<Arc<RuntimeRule>>)>) {
        while let Ok((ev, rules)) = rx.recv() {
            for rule in rules {
                for action in &rule.actions {
                    if let Err(e) = action.run(&ev.path) {
                        error!("action failed on {}: {:?}", ev.path.display(), e);
                    }
                }
            }
        }
    }
}
