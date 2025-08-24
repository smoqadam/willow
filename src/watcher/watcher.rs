use crate::models::{Config, Event, EventInfo};
use anyhow::Result;
use log::{Level, debug, error, info, log_enabled};
use notify::RecommendedWatcher;
use notify::event::ModifyKind;
use notify::{EventKind, RecursiveMode, Watcher as NotifyWatcher};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, new_debouncer};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

pub fn watch(
    config: &Config,
) -> Result<(
    Debouncer<RecommendedWatcher, FileIdMap>,
    mpsc::Receiver<EventInfo>,
)> {
    debug!("Initializing file watcher with {} watchers", config.watchers.len());
    let (tx, rx) = mpsc::channel();

    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        move |result: DebounceEventResult| {
            if let Ok(events) = result {
                debug!("Debouncer received {} events", events.len());
                // only te last event is important i guess :D
                if let Some(debounced_event) = events.last() {
                    debug!("Processing debounced event: {:?} for paths: {:?}", 
                           debounced_event.kind, debounced_event.paths);
                    let ev = EventInfo {
                        event: match debounced_event.kind {
                            EventKind::Create(_) => Event::Created,
                            EventKind::Modify(ModifyKind::Data(..)) => Event::Modified,
                            EventKind::Remove(_) => Event::Deleted,
                            _ => {
                                debug!("Ignoring event kind: {:?}", debounced_event.kind);
                                return;
                            },
                        },
                        // always get the last path. todo: is it safe?
                        path: debounced_event.paths[debounced_event.paths.len() - 1].clone(),
                    };
                    debug!("Sending event to main loop: {:?}", ev);
                    let _ = tx.send(ev);
                }
            } else {
                error!("Debouncer error: {:?}", result);
            }
        },
    )?;

    for w in &config.watchers {
        let path = Path::new(&w.path);
        let mode = if w.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        info!("Watching path: {:?} (recursive: {})", path, w.recursive);
        debouncer.watch(path, mode)?;
    }

    Ok((debouncer, rx))
}
