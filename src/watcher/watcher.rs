use crate::models::{Config, Event, EventInfo};
use anyhow::Result;
use notify::event::ModifyKind;
use notify::{RecommendedWatcher};
use notify::{EventKind, RecursiveMode, Watcher as NotifyWatcher};
use notify_debouncer_full::{DebounceEventResult, new_debouncer, FileIdMap, Debouncer};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

pub fn watch(config: &Config) -> Result<(Debouncer<RecommendedWatcher, FileIdMap>, mpsc::Receiver<EventInfo>)> {
    let (tx, rx) = mpsc::channel();

    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |result: DebounceEventResult| {
            if let Ok(events) = result {
                for debounced_event in &events {
                    println!("debouced: {:?}", debounced_event);
                    let ev = EventInfo {
                        event: match debounced_event.kind {
                            EventKind::Create(_) => Event::Created,
                            EventKind::Modify(ModifyKind::Data(..)) => Event::Modified,
                            EventKind::Remove(_) => Event::Deleted,
                            _ => {

                                return
                            },
                        },
                        // always get the last path. todo: is it safe?
                        path: debounced_event.paths[debounced_event.paths.len() -1 ].clone(),
                    };
                    let _ = tx.send(ev);
                }
            }
        },
    )?;

    for w in &config.watchers {
        let path = Path::new(&w.path);
        debouncer.watch(
            path,
            if w.recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            },
        )?;
    }

    Ok((debouncer, rx))
}
