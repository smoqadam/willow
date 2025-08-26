use crate::conditions::Condition;
use crate::models::EventInfo;
use glob::Pattern;

pub struct GlobCondition {
    pattern: Pattern,
}

impl GlobCondition {
    pub fn new(pattern: String) -> anyhow::Result<Self> {
        let pattern = Pattern::new(&pattern)?;
        Ok(GlobCondition { pattern })
    }
}

impl Condition for GlobCondition {
    fn matches(&self, event_info: &EventInfo) -> bool {
        if let Some(filename) = event_info.path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                return self.pattern.matches(filename_str);
            }
        }
        false
    }
}
