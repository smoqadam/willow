use std::path::PathBuf;
use crate::conditions::Condition;

pub struct ExtensionCondition {
    extension: String,
}

impl ExtensionCondition {
    pub fn new(extension: String) -> Self {
        ExtensionCondition { extension }
    }
}

impl Condition for ExtensionCondition {
    fn matches(&self, path: &PathBuf) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return ext_str == self.extension;
            }
        }
        false
    }
}
