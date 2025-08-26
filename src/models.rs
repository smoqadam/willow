use crate::condition::ConditionConfig;
use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub watchers: Vec<Watcher>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Watcher {
    pub path: String,
    pub recursive: bool,
    pub actions: Vec<Action>,
    pub conditions: Vec<ConditionConfig>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Created,
    Modified,
    Deleted,
    Unsupported,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    Move { destination: String },
    Rename { template: String },
    Log { message: String },
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventInfo {
    pub path: PathBuf,
    pub event: Event,
}
