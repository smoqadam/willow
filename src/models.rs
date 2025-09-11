use crate::action::ActionConfig;
use crate::actions::Action;
use crate::condition::ConditionConfig;
use crate::conditions::Condition;
use serde_derive::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub watchers: Vec<Watcher>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Watcher {
    pub path: String,
    pub recursive: bool,
    pub ignore: Option<Vec<String>>,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
    pub event: Event,
    pub conditions: Vec<ConditionConfig>,
    pub actions: Vec<ActionConfig>,
}

pub struct RuntimeWatcher {
    pub path: PathBuf,
    pub recursive: bool,
    pub ignore: Option<Vec<String>>,
    pub rules: Vec<Arc<RuntimeRule>>,
}

pub struct RuntimeRule {
    pub event: Event,
    pub conditions: Vec<Box<dyn Condition>>,
    pub actions: Vec<Box<dyn Action>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Created,
    Modified,
    Deleted,
    Any,
    Unsupported,
}

#[derive(Debug, Clone)]
pub struct EventInfo {
    pub path: PathBuf,
    pub event: Event,
    pub meta: Option<FileMeta>,
}

#[derive(Debug, Clone)]
pub struct FileMeta {
    pub size: Option<u64>,
    #[allow(dead_code)]
    pub modified: Option<SystemTime>,
    #[allow(dead_code)]
    pub name: Option<String>,
    #[allow(dead_code)]
    pub ext: Option<String>,
}
