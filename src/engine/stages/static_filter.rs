use crate::engine::EngineCtx;
use crate::engine::pipeline::{PipelineMsg, Stage};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

pub struct StaticFilterStage;

impl StaticFilterStage {
    pub fn new() -> Self {
        Self
    }
}

impl Stage for StaticFilterStage {
    fn run(&mut self, ctx: Arc<EngineCtx>, rx: Receiver<PipelineMsg>, tx: Sender<PipelineMsg>) {
        while let Ok(msg) = rx.recv() {
            let ev = &msg.event;
            let matching: Vec<_> = msg
                .rules
                .into_iter()
                .filter(|r| r.event == ev.event || matches!(r.event, crate::models::Event::Any))
                .filter(|r| {
                    r.conditions
                        .iter()
                        .filter(|c| matches!(c.kind(), crate::conditions::ConditionKind::Static))
                        .all(|c| c.matches(ev, &ctx))
                })
                .collect();
            if matching.is_empty() {
                continue;
            }
            if tx
                .send(PipelineMsg {
                    event: ev.clone(),
                    rules: matching,
                })
                .is_err()
            {
                break;
            }
        }
    }
}
