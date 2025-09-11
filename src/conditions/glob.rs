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
    fn kind(&self) -> crate::conditions::ConditionKind {
        crate::conditions::ConditionKind::Static
    }
    fn matches(&self, ev: &EventInfo, _ctx: &EngineCtx) -> bool {
        if let Some(filename) = ev.path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                return self.pattern.matches(filename_str);
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
    fn matches_glob_on_filename() {
        let cond = GlobCondition::new("*.jpeg".to_string()).unwrap();
        let ev = EventInfo {
            path: PathBuf::from("/tmp/pic.jpeg"),
            event: Event::Created,
            meta: None,
        };
        assert!(cond.matches(&ev, &ctx()));
    }

    #[test]
    fn non_match_for_other_extensions() {
        let cond = GlobCondition::new("*.jpeg".to_string()).unwrap();
        let ev = EventInfo {
            path: PathBuf::from("/tmp/doc.pdf"),
            event: Event::Created,
            meta: None,
        };
        assert!(!cond.matches(&ev, &ctx()));
    }
}
