use crate::engine::pipeline::{Stage, PipelineMsg};
use crate::engine::EngineCtx;
use crate::models::{EventInfo, RuntimeRule, Event};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{mpsc::{Receiver, Sender}, Arc};
use std::time::{Duration, Instant, SystemTime};
use log::{debug, info, warn};

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
    check_count: u16, // Track how many times we've checked this file
}

pub struct StabilityStage {
    min_quiet: Duration,
    stable_required: u8,
    max_checks: u16, // Maximum checks before giving up
    max_pending_files: usize, // Limit pending files to prevent memory exhaustion
    temp_extensions: HashSet<String>, // Changed to owned strings
    state: HashMap<PathBuf, PendingFile>,
    sibling_map: HashMap<String, HashSet<PathBuf>>, // basename -> set of temp siblings
    last_cleanup: Instant,
    cleanup_interval: Duration,
}

impl StabilityStage {
    pub fn new() -> Self {
        Self {
            min_quiet: Duration::from_secs(3),
            stable_required: 2,
            max_checks: 100, // Prevent infinite checking
            max_pending_files: 10000, // Prevent memory exhaustion
            temp_extensions: ["part", "crdownload", "download", "tmp", "temp"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            state: HashMap::new(),
            sibling_map: HashMap::new(),
            last_cleanup: Instant::now(),
            cleanup_interval: Duration::from_secs(300), // Clean up every 5 minutes
        }
    }

    /// Safely extract basename from path, handling edge cases
    fn get_basename(path: &Path) -> Option<String> {
        path.file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty() && s.len() < 256) // Reasonable length limit
            .map(|s| s.to_string())
    }

    /// Check if path is safe to process (basic path traversal protection)
    fn is_safe_path(path: &Path) -> bool {
        // Basic checks for path safety
        if let Some(path_str) = path.to_str() {
            // Reject paths with suspicious patterns
            if path_str.contains("..") || path_str.contains('\0') {
                return false;
            }
        }

        // Must be absolute path for security
        path.is_absolute()
    }

    /// Cleanup old entries to prevent memory leaks
    fn cleanup_old_entries(&mut self) {
        if self.last_cleanup.elapsed() < self.cleanup_interval {
            return;
        }

        let now = Instant::now();
        let max_age = Duration::from_secs(3600); // 1 hour max age

        let mut to_remove = Vec::new();

        for (path, file) in &self.state {
            if now.duration_since(file.last_event) > max_age || file.check_count >= self.max_checks {
                warn!("Removing stale file from tracking: {:?} (age: {:?}, checks: {})",
                      path, now.duration_since(file.last_event), file.check_count);
                to_remove.push(path.clone());
            }
        }

        for path in to_remove {
            if let Some(file) = self.state.remove(&path) {
                // Clean up sibling map too
                if let Some(siblings) = self.sibling_map.get_mut(&file.basename) {
                    siblings.remove(&path);
                    if siblings.is_empty() {
                        self.sibling_map.remove(&file.basename);
                    }
                }
            }
        }

        self.last_cleanup = now;
    }

    fn add_event(&mut self, ev: EventInfo, rules: Vec<Arc<RuntimeRule>>) {
        // Security check
        if !Self::is_safe_path(&ev.path) {
            warn!("Rejecting unsafe path: {:?}", ev.path);
            return;
        }

        // Prevent memory exhaustion
        if self.state.len() >= self.max_pending_files {
            warn!("Too many pending files ({}), dropping event for: {:?}",
                  self.state.len(), ev.path);
            return;
        }

        let basename = match Self::get_basename(&ev.path) {
            Some(name) => name,
            None => {
                debug!("Could not extract basename from: {:?}", ev.path);
                return;
            }
        };

        // Check for temp extensions more safely
        let is_temp = ev.path.extension()
            .and_then(|e| e.to_str())
            .map(|ext| {
                let ext_lower = ext.to_ascii_lowercase();
                self.temp_extensions.contains(&ext_lower)
            })
            .unwrap_or(false);

        if is_temp {
            debug!("Detected temp file: {:?}", ev.path);
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
                    check_count: 0,
                },
            );
        }
    }

    fn has_sibling_artifacts(&self, basename: &str, ctx: &EngineCtx) -> bool {
        // Check sibling map first
        if let Some(siblings) = self.sibling_map.get(basename) {
            if !siblings.is_empty() {
                return true;
            }
        }

        // More efficient filesystem probing - find any file with this basename first
        let sample_parent = self.state.values()
            .find(|f| f.basename == basename)
            .and_then(|f| f.path.parent());

        if let Some(parent) = sample_parent {
            // Only check a reasonable number of extensions to prevent DoS
            for ext in self.temp_extensions.iter().take(10) {
                let temp_path = parent.join(format!("{}.{}", basename, ext));
                if ctx.fs.exists(&temp_path) {
                    debug!("Found temp artifact on filesystem: {:?}", temp_path);
                    return true;
                }
            }
        }
        false
    }

    fn check_stability(&mut self, ctx: &EngineCtx, tx: &Sender<PipelineMsg>) {
        let now = Instant::now();
        let mut to_emit = Vec::new();
        let mut to_remove = Vec::new();
        let mut cleared_basenames = HashSet::new();

        // Pre-compute which files have sibling artifacts to avoid repeated checks
        let mut files_with_siblings = HashSet::new();
        let basenames: HashSet<String> = self.state.values()
            .map(|f| f.basename.clone())
            .collect();

        for basename in basenames {
            if self.has_sibling_artifacts(&basename, ctx) {
                files_with_siblings.insert(basename);
            }
        }

        for (path, file) in self.state.iter_mut() {
            file.check_count += 1;

            // Skip if not past quiet period
            if now.duration_since(file.last_event) < self.min_quiet {
                debug!("Skipping {:?}, not past quiet period", path);
                continue;
            }

            // Skip if temp siblings still present
            if files_with_siblings.contains(&file.basename) {
                debug!("Skipping {:?}, temp siblings still present for {:?}",
                       path, file.basename);
                continue;
            }

            // Give up if we've checked too many times
            if file.check_count >= self.max_checks {
                warn!("Giving up on file after {} checks: {:?}", file.check_count, path);
                to_remove.push(file.path.clone());
                continue;
            }

            // Safe metadata access with error handling
            match ctx.fs.metadata(&file.path) {
                Ok(meta) => {
                    let size = meta.len();
                    let mtime = meta.modified().ok();

                    debug!("Probing {:?}: size={}, stable_count={}, checks={}, orig_kind={:?}, saw_modified={}",
                           path, size, file.stable_count, file.check_count, file.orig_kind, file.saw_modified);

                    // Check stability
                    if Some(size) == file.last_size && mtime == file.last_mtime {
                        file.stable_count = file.stable_count.saturating_add(1);
                    } else {
                        file.stable_count = 0;
                    }

                    file.last_size = Some(size);
                    file.last_mtime = mtime;

                    // Emit conditions with better logic
                    let stable_enough = file.stable_count >= self.stable_required;
                    let not_zero_created = !(size == 0 && matches!(file.orig_kind, Event::Created));
                    let event_condition = match file.orig_kind {
                        Event::Created => file.saw_modified,
                        _ => true,
                    };

                    if stable_enough && not_zero_created && event_condition {
                        info!("File is stable: {:?}", file.path);
                        to_emit.push(PipelineMsg {
                            event: EventInfo {
                                path: file.path.clone(),
                                event: file.orig_kind.clone(),
                            },
                            rules: file.rules.clone(),
                        });
                        to_remove.push(file.path.clone());
                        cleared_basenames.insert(file.basename.clone());
                    }
                }
                Err(err) => {
                    debug!("Failed to stat {:?}: {:?} (removing from tracking)", file.path, err);
                    to_remove.push(file.path.clone());
                    cleared_basenames.insert(file.basename.clone());
                }
            }
        }

        // Emit events
        for msg in to_emit {
            info!("Emitting stable event: {:?}", msg.event);
            if tx.send(msg).is_err() {
                debug!("Channel closed during emit, stopping");
                break;
            }
        }

        // Clean up state
        for path in to_remove {
            debug!("Removing file from state: {:?}", path);
            self.state.remove(&path);
        }

        // Clean up sibling map more efficiently
        for basename in cleared_basenames {
            debug!("Clearing sibling map for: {:?}", basename);
            self.sibling_map.remove(&basename);
        }
    }
}

impl Stage for StabilityStage {
    fn run(
        &mut self,
        ctx: Arc<EngineCtx>,
        rx: Receiver<PipelineMsg>,
        tx: Sender<PipelineMsg>,
    ) {
        let mut last_check = Instant::now();
        let check_interval = Duration::from_secs(1);

        info!("Stability stage starting");

        loop {
            // Process incoming events with timeout
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(msg) => {
                    self.add_event(msg.event, msg.rules);
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
                self.check_stability(&ctx, &tx);
                self.cleanup_old_entries();
                last_check = Instant::now();
            }
        }

        info!("Stability stage shut down cleanly");
    }
}
