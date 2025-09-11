use crate::conditions::Condition;
use crate::engine::EngineCtx;
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
    fn kind(&self) -> crate::conditions::ConditionKind {
        crate::conditions::ConditionKind::Static
    }
    fn matches(&self, ev: &EventInfo, _ctx: &EngineCtx) -> bool {
        if let Some(filename) = ev.path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                return self.regex.is_match(filename_str);
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
    fn matches_filename_against_regex() {
        let cond = RegexCondition::new("^file_\\d+\\.txt$".to_string()).unwrap();
        let ev = EventInfo {
            path: PathBuf::from("/tmp/dir/file_123.txt"),
            event: Event::Modified,
            meta: None,
        };
        assert!(cond.matches(&ev, &ctx()));
    }

    #[test]
    fn does_not_match_non_matching_filename() {
        let cond = RegexCondition::new("^file_\\d+\\.txt$".to_string()).unwrap();
        let ev = EventInfo {
            path: PathBuf::from("/tmp/dir/other.log"),
            event: Event::Modified,
            meta: None,
        };
        assert!(!cond.matches(&ev, &ctx()));
    }
}
