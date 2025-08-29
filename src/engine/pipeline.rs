use crate::models::{EventInfo, RuntimeRule};
use std::sync::{mpsc::{Receiver, Sender}, Arc};
use super::context::EngineCtx;

#[derive(Clone)]
pub struct PipelineMsg {
    pub event: EventInfo,
    pub rules: Vec<Arc<RuntimeRule>>,
}

/// Stage trait for pipeline stages that filter and transform events
pub trait Stage: Send + Sync {
    fn run(
        &mut self,
        ctx: Arc<EngineCtx>,
        rx: Receiver<PipelineMsg>,
        tx: Sender<PipelineMsg>,
    );
}

/// Sink trait for final stages that consume events without forwarding
pub trait Sink: Send + Sync {
    fn run(&mut self, ctx: Arc<EngineCtx>, rx: Receiver<PipelineMsg>);
}
