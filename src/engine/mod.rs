mod context;
mod pipeline;
mod stages;

pub use context::EngineCtx;
pub use pipeline::{PipelineMsg, Sink, Stage};

use crate::engine::pipeline::PipelineBuilder;
use crate::engine::stages::{ActionSink, IoFilterStage, StabilityStage, StaticFilterStage};
use crate::fs::{Fs, StdFs};
use crate::models::{Config, RuntimeRule, RuntimeWatcher, Watcher};
use log::debug;
use std::sync::mpsc::Sender;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread;
use std::thread::JoinHandle;

pub struct EngineHandle {
    stage_handles: Vec<JoinHandle<()>>,
    watcher_handles: Vec<JoinHandle<()>>,
    ingress: Sender<PipelineMsg>,
    shutdown: Arc<AtomicBool>,
}

impl EngineHandle {
    pub fn shutdown(self) {
        self.shutdown.store(true, Ordering::SeqCst);
        drop(self.ingress);
        for h in self.watcher_handles {
            let _ = h.join();
        }
        for h in self.stage_handles {
            let _ = h.join();
        }
    }
}

pub fn start(config: &Config) -> anyhow::Result<EngineHandle> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let ctx = Arc::new(EngineCtx::new(
        Arc::new(StdFs::new()) as Arc<dyn Fs>,
        shutdown.clone(),
    ));

    let builder = PipelineBuilder::new(ctx.clone(), ActionSink::new());
    let (pipeline_tx, stage_handles) = builder
        // only for static conditions that can be run on path (e.g regex)
        .add_stage(StaticFilterStage::new())
        // this stage checks  if the file is stable
        .add_stage(StabilityStage::new())
        // some conditions (kind::IO) requires to access filesystem therefore they are more expensive
        // and some times the file need to be stable enough to check its size (e.g. SizeGt conditions)
        .add_stage(IoFilterStage::new())
        .build();

    let watcher_handles = spawn_watcher(&config.watchers, pipeline_tx.clone(), ctx.clone())?;
    Ok(EngineHandle {
        stage_handles,
        watcher_handles,
        ingress: pipeline_tx,
        shutdown,
    })
}

fn spawn_watcher(
    watchers: &Vec<Watcher>,
    ingress_tx: Sender<PipelineMsg>,
    ctx: Arc<EngineCtx>,
) -> anyhow::Result<Vec<JoinHandle<()>>> {
    let mut handles = Vec::new();
    for watcher_config in watchers {
        let runtime_watcher = RuntimeWatcher {
            path: watcher_config.path.clone().into(),
            recursive: watcher_config.recursive,
            ignore: watcher_config.ignore.clone(),
            rules: gather_rules(&watcher_config)?,
        };
        let ingress_tx_clone = ingress_tx.clone();
        let ctx2 = ctx.clone();
        let h = thread::Builder::new()
            .name(format!("watcher:{}", runtime_watcher.path.display()))
            .spawn(move || {
                let (rx, _debouncer) = runtime_watcher.watch().expect("failed to start watcher");
                loop {
                    if ctx2.shutdown.load(Ordering::Relaxed) {
                        break;
                    }
                    match rx.recv_timeout(std::time::Duration::from_millis(200)) {
                        Ok(ev) => {
                            debug!("raw event {:?}", ev);
                            if ingress_tx_clone
                                .send(PipelineMsg {
                                    event: ev,
                                    rules: runtime_watcher.rules.clone(),
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                }
            })?;
        handles.push(h);
    }
    Ok(handles)
}

fn gather_rules(watcher: &Watcher) -> anyhow::Result<Vec<Arc<RuntimeRule>>> {
    let mut runtime_rules: Vec<Arc<RuntimeRule>> = Vec::new();

    for rule in &watcher.rules {
        let mut conditions: Vec<Box<dyn crate::conditions::Condition>> = Vec::new();
        for condition_config in &rule.conditions {
            conditions.push(condition_config.clone().into_condition()?);
        }

        let mut actions: Vec<Box<dyn crate::actions::Action>> = Vec::new();
        for action_config in &rule.actions {
            actions.push(action_config.clone().into_action());
        }

        runtime_rules.push(Arc::new(RuntimeRule {
            event: rule.event.clone(),
            conditions,
            actions,
        }));
    }
    Ok(runtime_rules)
}
