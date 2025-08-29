use crate::conditions::Condition;
use crate::engine::EngineCtx;
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
    fn matches(&self, ev: &EventInfo, _ctx: &EngineCtx) -> bool {
        if let Some(filename) = ev.path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                return self.pattern.matches(filename_str);
            }
        }
        false
    }
}
