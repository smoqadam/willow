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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::EngineCtx;
    use crate::fs::Fs;
    use crate::models::{Event, EventInfo};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, atomic::AtomicBool};
    use std::{fs, io};

    #[derive(Default)]
    struct MockFs {
        content: String,
        err: bool,
    }

    impl Fs for MockFs {
        fn metadata(&self, _path: &Path) -> io::Result<fs::Metadata> {
            Err(io::Error::other("unused"))
        }
        fn create_dir_all(&self, _path: &Path) -> io::Result<()> {
            Ok(())
        }
        fn rename(&self, _from: &Path, _to: &Path) -> io::Result<()> {
            Ok(())
        }
        fn exists(&self, _path: &Path) -> bool {
            false
        }
        fn read_to_string(&self, _path: &Path) -> io::Result<String> {
            if self.err {
                Err(io::Error::other("boom"))
            } else {
                Ok(self.content.clone())
            }
        }
    }

    fn ctx_with(content: &str, err: bool) -> EngineCtx {
        EngineCtx::new(
            Arc::new(MockFs {
                content: content.into(),
                err,
            }) as Arc<dyn Fs>,
            Arc::new(AtomicBool::new(false)),
        )
    }

    #[test]
    fn finds_substring_when_present() {
        let ctx = ctx_with("hello world", false);
        let cond = ContainsCondition::new("world".into());
        let ev = EventInfo {
            path: PathBuf::from("/tmp/file.txt"),
            event: Event::Any,
            meta: None,
        };
        assert!(cond.matches(&ev, &ctx));
    }

    #[test]
    fn returns_false_when_absent_or_error() {
        let ctx = ctx_with("hello", false);
        let cond = ContainsCondition::new("world".into());
        let ev = EventInfo {
            path: PathBuf::from("/tmp/file.txt"),
            event: Event::Any,
            meta: None,
        };
        assert!(!cond.matches(&ev, &ctx));

        let ctx_err = ctx_with("ignored", true);
        assert!(!cond.matches(&ev, &ctx_err));
    }
}
