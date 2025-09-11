use crate::models::{Config, Watcher};
use crate::fs::Fs;
use anyhow::{Result, bail, Context};
use std::sync::Arc;
use std::collections::HashSet;
 
use std::fs;

pub fn load(path: String) -> Result<Config> {
    // println!("{:?}", path);
    let content = fs::read_to_string(path)?;
    // println!("{:?}", content);
    let config: Config = serde_yaml::from_str(content.as_str())?;
    // println!("{:?}", config);
    Ok(config)
}

pub fn validate(config: &Config, fs: Arc<dyn Fs>) -> Result<()> {
    let mut seen: HashSet<std::path::PathBuf> = HashSet::new();
    for watcher in &config.watchers {
        validate_watcher(watcher, fs.clone()).with_context(|| format!("invalid watcher path: {}", watcher.path))?;
        let canon = std::fs::canonicalize(&watcher.path).with_context(|| format!("cannot canonicalize path: {}", watcher.path))?;
        if !seen.insert(canon.clone()) {
            bail!("duplicate watcher path: {}", canon.display());
        }
    }
    Ok(())
}

fn validate_watcher(w: &Watcher, fs: Arc<dyn Fs>) -> Result<()> {
    let path = std::path::Path::new(&w.path);
    let md = fs.metadata(path).with_context(|| format!("watch path not accessible: {}", w.path))?;
    if !md.is_dir() { bail!("watch path is not a directory: {}", w.path); }
    for rule in &w.rules {
        for action in &rule.actions {
            if let crate::action::ActionConfig::Move { destination, .. } = action {
                if destination.trim().is_empty() { bail!("move destination is empty"); }
                if !destination.contains('{') && !destination.contains('}') {
                    let dest_path = std::path::Path::new(destination);
                    if destination.ends_with('/') || destination.ends_with('\\') {
                        let dir = dest_path;
                        if !fs.exists(dir) { bail!("destination directory does not exist: {}", destination); }
                    } else {
                        let parent = dest_path.parent().unwrap_or_else(|| std::path::Path::new(""));
                        if parent.to_string_lossy().is_empty() { bail!("destination has no parent: {}", destination); }
                        if !fs.exists(parent) { bail!("destination parent does not exist: {}", parent.display()); }
                    }
                }
            }
        }
        for cond in &rule.conditions {
            let _ = cond.clone().into_condition().context("invalid condition")?;
        }
    }
    Ok(())
}
