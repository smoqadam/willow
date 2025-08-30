use crate::engine::pipeline::{Sink, PipelineMsg};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use log::error;
use crate::engine::EngineCtx;

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
