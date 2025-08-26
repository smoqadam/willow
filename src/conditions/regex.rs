use crate::conditions::Condition;
use crate::models::EventInfo;
use regex::Regex;

pub struct RegexCondition {
    regex: Regex,
}

impl RegexCondition {
    pub fn new(pattern: String) -> anyhow::Result<Self> {
        let regex = Regex::new(&pattern)?;
        Ok(RegexCondition { regex })
    }
}

impl Condition for RegexCondition {
    fn matches(&self, event_info: &EventInfo) -> bool {
        if let Some(filename) = event_info.path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                return self.regex.is_match(filename_str);
            }
        }
        false
    }
}
