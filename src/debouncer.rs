use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use crate::models::{Event, EventInfo};

#[derive(Debug, Clone)]
pub struct NormalizedEvent {
    pub path: PathBuf,
    pub event: Event,
}

pub struct Debouncer {
    pub buffer: HashMap<PathBuf, Vec<Event>>,
    pub last_seen: Instant,
    pub delay: Duration,
}

impl Debouncer {
    pub fn new() -> Self {
        Debouncer {
            buffer: HashMap::new(),
            last_seen: Instant::now(),
            delay: Duration::from_millis(100),
        }
    }
    pub fn push(&mut self, event: EventInfo) {
        for path in event.paths {
            self.buffer.entry(path)
                .or_default()
                .push(event.event.clone());
        }
        self.last_seen = Instant::now();
    }

    pub fn flush_if_ready(&mut self) -> bool {
        self.last_seen.elapsed() >= self.delay
    }

    pub fn drain(&mut self) -> Vec<NormalizedEvent> {
        let events = std::mem::take(&mut self.buffer);
        events.into_iter().map(normalize).collect()
    }
}

fn normalize((path, events): (PathBuf, Vec<Event>)) -> NormalizedEvent {
    // prioritize events: Created > Modified > Deleted
    // if we have multiple events for the same file, use the most significant one
    let event = if events.contains(&Event::Created) {
        Event::Created
    } else if events.contains(&Event::Modified) {
        Event::Modified
    } else {
        Event::Deleted
    };

    NormalizedEvent { path, event }
}
