use crate::models::{Config, Event, EventInfo};
use notify::Event as NotifyEvent;
use notify::event::ModifyKind;
use notify::{Error, EventKind, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;
use std::sync::mpsc;

pub fn watch(
    config: &Config,
) -> Result<(notify::RecommendedWatcher, mpsc::Receiver<EventInfo>), Error> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res: Result<NotifyEvent, _>| {
        if let Ok(event) = res {
            let ev = EventInfo {
                event: match event.kind {
                    EventKind::Create(_) => Event::Created,
                    EventKind::Modify(ModifyKind::Data(..)) => Event::Modified,
                    EventKind::Remove(_) => Event::Deleted,
                    _ => return,
                },
                paths: event.paths,
            };
            let _ = tx.send(ev);
        }
    })?;

    for w in &config.watchers {
        let path = Path::new(&w.path);
        watcher.watch(
            path,
            if w.recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            },
        )?;
    }

    Ok((watcher, rx))
}