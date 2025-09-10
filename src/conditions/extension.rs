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
    fn kind(&self) -> crate::conditions::ConditionKind {
        crate::conditions::ConditionKind::Static
    }
    fn matches(&self, ev: &EventInfo, _ctx: &EngineCtx) -> bool {
        if let Some(ext) = ev.path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return ext_str == self.extension;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::EngineCtx;
    use crate::fs::StdFs;
    use crate::models::{Event, EventInfo};
    use std::path::PathBuf;
    use std::sync::{Arc, atomic::AtomicBool};

    fn ctx() -> EngineCtx {
        EngineCtx::new(Arc::new(StdFs::new()), Arc::new(AtomicBool::new(false)))
    }

    #[test]
    fn matches_exact_extension() {
        let cond = ExtensionCondition::new("txt".into());
        let ev = EventInfo { path: PathBuf::from("/x/file.txt"), event: Event::Any, meta: None };
        assert!(cond.matches(&ev, &ctx()));
    }

    #[test]
    fn does_not_match_when_extension_differs() {
        let cond = ExtensionCondition::new("txt".into());
        let ev = EventInfo { path: PathBuf::from("/x/file.md"), event: Event::Any, meta: None };
        assert!(!cond.matches(&ev, &ctx()));
    }
}
