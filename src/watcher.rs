use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use log::debug;
use notify::{EventKind, RecommendedWatcher, RecursiveMode};
use crate::models::{Event, EventInfo, RuntimeWatcher};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, new_debouncer};

impl RuntimeWatcher {
    pub fn watch(&self) -> anyhow::Result<(mpsc::Receiver<EventInfo>, Debouncer<RecommendedWatcher, FileIdMap>)> {
        let (tx, rx) = mpsc::channel();
        let ignore_set: std::sync::Arc<HashSet<String>> = std::sync::Arc::new(
            self.ignore
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .map(|s| s.to_ascii_lowercase())
                .collect::<HashSet<_>>()
        );

        let mut debouncer = new_debouncer(
            Duration::from_millis(100),
            None,
            move |event_result: DebounceEventResult| {
                if let Ok(res) = event_result {
                    let last_event = &res[res.len() - 1];
                    let first_path = &last_event.paths[0];

                    let ext = first_path
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_ascii_lowercase())
                        .unwrap_or_default();

                    if ignore_set.contains(&ext) {
                        debug!("event ignored for {:?}. reason: ignored extension: .{ext}", first_path);
                        return;
                    }

                    tx.send(EventInfo {
                        path: PathBuf::from(first_path),
                        event: match last_event.kind {
                            EventKind::Create(_) => Event::Created,
                            EventKind::Modify(_) => Event::Modified,
                            EventKind::Remove(_) => Event::Deleted,
                            _ => return,
                        },
                    }).unwrap();
                };
            }
        )?;
        
        let recursive = if self.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        debouncer.watch(&self.path, recursive)?;
        Ok((rx, debouncer))
    }
}
