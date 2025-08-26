use crate::conditions::Condition;
use crate::models::EventInfo;
use std::fs;

pub struct ContainsCondition {
    text: String,
}

impl ContainsCondition {
    pub fn new(text: String) -> Self {
        ContainsCondition { text }
    }
}

impl Condition for ContainsCondition {
    fn matches(&self, event_info: &EventInfo) -> bool {
        if let Ok(content) = fs::read_to_string(&event_info.path) {
            return content.contains(&self.text);
        }
        false
    }
}
