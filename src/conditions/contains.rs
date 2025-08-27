use crate::conditions::Condition;
use std::fs;
use std::path::PathBuf;

pub struct ContainsCondition {
    text: String,
}

impl ContainsCondition {
    pub fn new(text: String) -> Self {
        ContainsCondition { text }
    }
}

impl Condition for ContainsCondition {
    fn matches(&self, path: &PathBuf) -> bool {
        if let Ok(content) = fs::read_to_string(&path) {
            return content.contains(&self.text);
        }
        false
    }
}
