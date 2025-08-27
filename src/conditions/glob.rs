use std::path::PathBuf;
use crate::conditions::Condition;
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
    fn matches(&self, path: &PathBuf) -> bool {
        if let Some(filename) = path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                return self.pattern.matches(filename_str);
            }
        }
        false
    }
}
