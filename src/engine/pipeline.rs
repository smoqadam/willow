use super::context::EngineCtx;
use crate::engine::ActionSink;
use crate::models::{EventInfo, RuntimeRule};
use std::sync::{
    Arc, mpsc,
    mpsc::{Receiver, Sender},
};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone)]
pub struct PipelineMsg {
    pub event: EventInfo,
    pub rules: Vec<Arc<RuntimeRule>>,
}

/// Stage trait for pipeline stages that filter and transform events
pub trait Stage: Send + Sync {
    fn run(&mut self, ctx: Arc<EngineCtx>, rx: Receiver<PipelineMsg>, tx: Sender<PipelineMsg>);
}

/// Sink trait for final stages that consume events without forwarding
pub trait Sink: Send + Sync {
    fn run(&mut self, ctx: Arc<EngineCtx>, rx: Receiver<PipelineMsg>);
}

pub struct PipelineBuilder {
    ctx: Arc<EngineCtx>,
    stages: Vec<Box<dyn Stage>>,
    sink: Box<dyn Sink>,
}

impl PipelineBuilder {
    pub fn new(ctx: Arc<EngineCtx>, sink: impl Sink + 'static) -> Self {
        PipelineBuilder {
            ctx,
            stages: Vec::new(),
            sink: Box::new(sink),
        }
    }

    pub fn add_stage(mut self, stage: impl Stage + 'static) -> Self {
        self.stages.push(Box::new(stage));
        self
    }

    pub fn sink(mut self, sink: impl Sink + 'static) -> Self {
        self.sink = Box::new(sink);
        self
    }

    pub fn build(self) -> (Sender<PipelineMsg>, Vec<JoinHandle<()>>) {
        let mut handles = Vec::new();
        let (ingress_tx, mut prev_rx) = mpsc::channel::<PipelineMsg>();

        // Spawn each stage: prev_rx -> stage -> next_tx
        for (i, mut stage) in self.stages.into_iter().enumerate() {
            let (next_tx, next_rx) = mpsc::channel::<PipelineMsg>();
            let ctx = self.ctx.clone();
            let name = format!("stage-{}", i);
            let handle = thread::Builder::new()
                .name(name)
                .spawn(move || {
                    stage.run(ctx, prev_rx, next_tx);
                })
                .expect("spawn stage");
            handles.push(handle);
            prev_rx = next_rx;
        }

        let mut sink = self.sink;
        let ctx = self.ctx.clone();
        let handle = thread::Builder::new()
            .name("sink".into())
            .spawn(move || {
                sink.run(ctx, prev_rx);
            })
            .expect("spawn sink");
        handles.push(handle);

        (ingress_tx, handles)
    }
}
