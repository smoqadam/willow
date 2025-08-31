use crate::conditions::Condition;
use crate::engine::EngineCtx;
use crate::models::EventInfo;

pub struct ContainsCondition {
    text: String,
}

impl ContainsCondition {
    pub fn new(text: String) -> Self {
        ContainsCondition { text }
    }
}

impl Condition for ContainsCondition {
    fn kind(&self) -> crate::conditions::ConditionKind {
        crate::conditions::ConditionKind::Io
    }
    fn matches(&self, ev: &EventInfo, ctx: &EngineCtx) -> bool {
        if let Ok(content) = ctx.fs.read_to_string(&ev.path) {
            return content.contains(&self.text);
        }
        false
    }
}
