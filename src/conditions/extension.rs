use crate::conditions::Condition;
use crate::models::EventInfo;

pub struct ExtensionCondition {
    extension: String,
}

impl ExtensionCondition {
    pub fn new(extension: String) -> Self {
        ExtensionCondition { extension }
    }
}

impl Condition for ExtensionCondition {
    fn matches(&self, event_info: &EventInfo) -> bool {
        if let Some(ext) = event_info.path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return ext_str == self.extension;
            }
        }
        false
    }
}
