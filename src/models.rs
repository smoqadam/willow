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
    pub path: String,
    pub recursive: bool,
    pub ignore: Option<Vec<String>>,
    pub rules: Vec<RuntimeRule>,
}

pub struct RuntimeRule {
    pub conditions: Vec<Box<dyn Condition>>,
    pub actions: Vec<Box<dyn Action>>,
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
