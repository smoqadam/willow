use crate::engine::pipeline::Stage;
use crate::models::{EventInfo, RuntimeRule, Event};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{mpsc::{Receiver, Sender}, Arc};
use std::time::{Duration, Instant, SystemTime};
use log::{debug, info};

struct PendingFile {
    path: PathBuf,
    last_size: Option<u64>,
    last_mtime: Option<SystemTime>,
    last_event: Instant,
    stable_count: u8,
    rules: Vec<Arc<RuntimeRule>>,
    basename: String,
    orig_kind: Event,
    saw_modified: bool,
}

pub struct StabilityStage {
    min_quiet: Duration,
    stable_required: u8,
    temp_extensions: HashSet<&'static str>,
    state: HashMap<PathBuf, PendingFile>,
    sibling_map: HashMap<String, HashSet<PathBuf>>, // basename -> set of temp siblings
}

impl StabilityStage {
    pub fn new() -> Self {
        Self {
            min_quiet: Duration::from_secs(3),
            stable_required: 2,
            temp_extensions: ["part", "crdownload", "download"].iter().cloned().collect(),
            state: HashMap::new(),
            sibling_map: HashMap::new(),
        }
    }

    fn get_basename(path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string()
    }

    fn add_event(&mut self, ev: EventInfo, rules: Vec<Arc<RuntimeRule>>) {
        let ext = ev.path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let basename = Self::get_basename(&ev.path);

        if self.temp_extensions.contains(ext.as_str()) {
            debug!("Detected temp file: {:?} (ext: .{})", ev.path, ext);
            self.sibling_map
                .entry(basename)
                .or_default()
                .insert(ev.path.clone());
            return;
        }

        let now = Instant::now();
        let is_modify = matches!(ev.event, Event::Modified);

        if let Some(existing) = self.state.get_mut(&ev.path) {
            debug!("Updating existing file: {:?}, event: {:?}", ev.path, ev.event);
            existing.last_event = now;
            existing.saw_modified |= is_modify;
            existing.stable_count = 0; // Reset stability count on new event
        } else {
            debug!("Tracking new file: {:?}, event: {:?}", ev.path, ev.event);
            self.state.insert(
                ev.path.clone(),
                PendingFile {
                    path: ev.path,
                    last_size: None,
                    last_mtime: None,
                    last_event: now,
                    stable_count: 0,
                    rules,
                    basename,
                    orig_kind: ev.event,
                    saw_modified: is_modify,
                },
            );
        }
    }

    fn has_sibling_artifacts(&self, basename: &str) -> bool {
        // Check sibling map first
        if let Some(siblings) = self.sibling_map.get(basename) {
            if !siblings.is_empty() {
                return true;
            }
        }

        // Probe filesystem for temp artifacts
        if let Some(parent) = self.state.values().find(|f| f.basename == basename).map(|f| f.path.parent()).flatten() {
            for ext in &self.temp_extensions {
                let temp_path = parent.join(format!("{}.{}", basename, ext));
                if temp_path.exists() {
                    debug!("Found temp artifact on filesystem: {:?}", temp_path);
                    return true;
                }
            }
        }
        false
    }

    fn check_stability(&mut self, tx: &Sender<(EventInfo, Vec<Arc<RuntimeRule>>)>) {
        let now = Instant::now();
        let mut to_emit = vec![];
        let mut to_remove = vec![];
        let mut cleared_basenames = vec![];

        // Collect paths that need sibling artifact checking
        let paths_to_check: Vec<_> = self.state.keys().cloned().collect();
        let mut skip_paths = HashSet::new();

        for path in paths_to_check {
            if let Some(file) = self.state.get(&path) {
                if self.has_sibling_artifacts(&file.basename) {
                    debug!("Skipping {:?}, temp siblings still present for {:?}", path, file.basename);
                    skip_paths.insert(path);
                }
            }
        }

        for (path, file) in self.state.iter_mut() {
            // Check emit conditions
            if now.duration_since(file.last_event) < self.min_quiet {
                debug!("Skipping {:?}, not past quiet period", path);
                continue;
            }

            if skip_paths.contains(path) {
                continue;
            }

            match fs::metadata(&file.path) {
                Ok(meta) => {
                    let size = meta.len();
                    let mtime = meta.modified().ok();

                    debug!("Probing {:?}: size={}, stable_count={}, orig_kind={:?}, saw_modified={}", 
                           path, size, file.stable_count, file.orig_kind, file.saw_modified);

                    if Some(size) == file.last_size && mtime == file.last_mtime {
                        file.stable_count += 1;
                    } else {
                        file.stable_count = 0;
                    }

                    file.last_size = Some(size);
                    file.last_mtime = mtime;

                    // Check all emit conditions
                    let stable_enough = file.stable_count >= self.stable_required;
                    let not_zero_created = !(size == 0 && matches!(file.orig_kind, Event::Created));
                    let event_condition = (matches!(file.orig_kind, Event::Created) && file.saw_modified) 
                                        || !matches!(file.orig_kind, Event::Created);

                    if stable_enough && not_zero_created && event_condition {
                        info!("File is stable: {:?}", file.path);
                        to_emit.push((
                            EventInfo {
                                path: file.path.clone(),
                                event: file.orig_kind.clone(),
                            },
                            file.rules.clone(),
                        ));
                        to_remove.push(file.path.clone());
                        cleared_basenames.push(file.basename.clone());
                    }
                }
                Err(err) => {
                    debug!("Failed to stat {:?}: {:?}", file.path, err);
                    to_remove.push(file.path.clone());
                }
            }
        }

        for (info, rules) in to_emit {
            info!("Emitting stable event: {:?}", info);
            if tx.send((info, rules)).is_err() {
                break; // Channel closed, exit gracefully
            }
        }
        for path in to_remove {
            debug!("Removing file from state: {:?}", path);
            self.state.remove(&path);
        }
        for basename in cleared_basenames {
            debug!("Clearing sibling map for: {:?}", basename);
            self.sibling_map.remove(&basename);
        }
    }
}

impl Stage for StabilityStage {
    fn run(
        &mut self,
        rx: Receiver<(EventInfo, Vec<Arc<RuntimeRule>>)>,
        tx: Sender<(EventInfo, Vec<Arc<RuntimeRule>>)>,
    ) {
        let mut last_check = Instant::now();
        let check_interval = Duration::from_secs(1);

        loop {
            // Process incoming events with timeout
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok((ev, rules)) => {
                    self.add_event(ev, rules);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout is expected, continue to stability check
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    info!("Stability stage shutting down - channel disconnected");
                    break;
                }
            }

            // Periodically check for stable files
            if last_check.elapsed() >= check_interval {
                self.check_stability(&tx);
                last_check = Instant::now();
            }
        }
    }
}
