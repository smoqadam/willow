use crate::engine::pipeline::{Stage, PipelineMsg};
use crate::engine::EngineCtx;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub struct IoFilterStage;

impl IoFilterStage {
    pub fn new() -> Self { Self }
}

impl Stage for IoFilterStage {
    fn run(
        &mut self,
        ctx: Arc<EngineCtx>,
        rx: Receiver<PipelineMsg>,
        tx: Sender<PipelineMsg>,
    ) {
        while let Ok(msg) = rx.recv() {
            let ev = &msg.event;
            let filtered: Vec<_> = msg.rules.into_iter()
                .filter(|r| r.event == ev.event || matches!(r.event, crate::models::Event::Any))
                .filter(|r| r.conditions.iter().filter(|c| matches!(c.kind(), crate::conditions::ConditionKind::Io)).all(|c| c.matches(ev, &ctx)))
                .collect();
            if filtered.is_empty() { continue; }
            if tx.send(PipelineMsg { event: ev.clone(), rules: filtered }).is_err() { break; }
        }
    }
}
