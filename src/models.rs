use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub watchers: Vec<Watcher>,
}

#[derive(Deserialize, Debug)]
pub struct Watcher {
    pub path: String,
    pub recursive: bool,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub event: Event,
    pub actions: Vec<Action>,
    pub conditions: Vec<Condition>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Created,
    Modified,
    Deleted,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    Move { destination: String },
    Rename { template: String },
    Log { message: String },
}

pub struct EventInfo {
    pub paths: Vec<PathBuf>,
    pub event: Event,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Condition {
    Regex { value: String },
    Glob { value: String },
    Extension { value: String },
    SizeGt { value: i64 },
    SizeLt { value: i64 },
    Contains { text: String },
}
