use crate::conditions::Condition;
use crate::engine::EngineCtx;
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
    fn matches(&self, ev: &EventInfo, _ctx: &EngineCtx) -> bool {
        if let Some(ext) = ev.path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return ext_str == self.extension;
            }
        }
        false
    }
}
