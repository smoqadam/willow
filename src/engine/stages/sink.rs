use crate::engine::EngineCtx;
use crate::engine::pipeline::{PipelineMsg, Sink};
use log::error;
use std::sync::Arc;
use std::sync::mpsc::Receiver;

pub struct ActionSink;

impl ActionSink {
    pub fn new() -> Self {
        Self
    }
}

impl Sink for ActionSink {
    fn run(&mut self, ctx: Arc<EngineCtx>, rx: Receiver<PipelineMsg>) {
        while let Ok(msg) = rx.recv() {
            for rule in msg.rules {
                for action in &rule.actions {
                    if let Err(e) = action.run(&msg.event.path, &ctx) {
                        error!("action failed on {}: {:?}", msg.event.path.display(), e);
                    }
                }
            }
        }
    }
}
