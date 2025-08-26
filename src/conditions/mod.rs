use std::path::PathBuf;

mod event;

pub trait  Condition {
    fn matches(&self, path: &PathBuf) -> bool;
}

