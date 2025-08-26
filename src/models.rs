use crate::condition::ConditionConfig;
use crate::action::ActionConfig;
use crate::conditions::Condition;
use crate::actions::Action;
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
    pub actions: Vec<ActionConfig>,
    pub conditions: Vec<ConditionConfig>,
}

pub struct RuntimeWatcher {
    pub path: String,
    pub recursive: bool,
    pub actions: Vec<Box<dyn Action>>,
    pub conditions: Vec<Box<dyn Condition>>,
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
pub struct EventInfo {
    pub path: PathBuf,
    pub event: Event,
}
