use crate::models::{Event, EventInfo, RuntimeWatcher};
use log::debug;
use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

impl RuntimeWatcher {
    pub fn watch(
        &self,
    ) -> anyhow::Result<(mpsc::Receiver<EventInfo>, Box<dyn std::any::Any + Send>)> {
        let (tx, rx) = mpsc::channel();
        let ignore_set: std::sync::Arc<HashSet<String>> = std::sync::Arc::new(
            self.ignore
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .map(|s| s.to_ascii_lowercase())
                .collect::<HashSet<_>>(),
        );

        // Pre-compute which event kinds this watcher cares about based on its rules
        let allowed_events: HashSet<Event> = self.rules.iter().map(|r| r.event.clone()).collect();

        let mut debouncer = new_debouncer(
            Duration::from_millis(100),
            None,
            move |event_result: DebounceEventResult| {
                if let Ok(res) = event_result {
                    let Some(last_event) = res.last() else {
                        return;
                    };
                    let Some(first_path) = last_event.paths.first() else {
                        return;
                    };

                    let ext = first_path
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_ascii_lowercase())
                        .unwrap_or_default();

                    if ignore_set.contains(&ext) {
                        debug!(
                            "event ignored for {first_path:?}. reason: ignored extension: .{ext}"
                        );
                        return;
                    }

                    let mapped_event = match last_event.kind {
                        EventKind::Create(_) => Event::Created,
                        EventKind::Modify(_) => Event::Modified,
                        EventKind::Remove(_) => Event::Deleted,
                        _ => return,
                    };

                    // Early event filtering: drop if no rule matches this event
                    if !allowed_events.is_empty()
                        && !allowed_events.contains(&Event::Any)
                        && !allowed_events.contains(&mapped_event)
                    {
                        debug!(
                            "event ignored for {first_path:?}. reason: unmatched event: {mapped_event:?}"
                        );
                        return;
                    }

                    if let Err(e) = tx.send(EventInfo {
                        path: PathBuf::from(first_path),
                        event: mapped_event,
                        meta: None,
                    }) {
                        debug!("watcher channel closed while sending event: {e:?}");
                    }
                }
            },
        )?;

        let recursive = if self.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        debouncer.watch(&self.path, recursive)?;
        Ok((rx, Box::new(debouncer)))
    }
}
