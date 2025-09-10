use crate::conditions::Condition;
use crate::engine::EngineCtx;
use crate::models::EventInfo;

pub struct SizeGtCondition {
    size: i64,
}

impl SizeGtCondition {
    pub fn new(size: i64) -> Self {
        SizeGtCondition { size }
    }
}

impl Condition for SizeGtCondition {
    fn kind(&self) -> crate::conditions::ConditionKind {
        crate::conditions::ConditionKind::Io
    }
    fn matches(&self, ev: &EventInfo, ctx: &EngineCtx) -> bool {
        if let Some(sz) = ev.meta.as_ref().and_then(|m| m.size) {
            return sz as i64 > self.size;
        }
        match ctx.fs.metadata(&ev.path) {
            Ok(md) => (md.len() as i64) > self.size,
            Err(_) => false,
        }
    }
}

pub struct SizeLtCondition {
    size: i64,
}

impl SizeLtCondition {
    pub fn new(size: i64) -> Self {
        SizeLtCondition { size }
    }
}

impl Condition for SizeLtCondition {
    fn kind(&self) -> crate::conditions::ConditionKind {
        crate::conditions::ConditionKind::Io
    }
    fn matches(&self, ev: &EventInfo, ctx: &EngineCtx) -> bool {
        if let Some(sz) = ev.meta.as_ref().and_then(|m| m.size) {
            return (sz as i64) < self.size;
        }
        match ctx.fs.metadata(&ev.path) {
            Ok(md) => (md.len() as i64) < self.size,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::EngineCtx;
    use crate::fs::StdFs;
    use crate::models::{Event, EventInfo, FileMeta};
    use std::path::PathBuf;
    use std::sync::{Arc, atomic::AtomicBool};
    use std::fs as stdfs;

    fn ctx_std() -> EngineCtx {
        EngineCtx::new(Arc::new(StdFs::new()), Arc::new(AtomicBool::new(false)))
    }

    #[test]
    fn size_gt_with_meta_short_circuits() {
        let cond = SizeGtCondition::new(10);
        let ev = EventInfo { path: PathBuf::from("/tmp/a"), event: Event::Any, meta: Some(FileMeta { size: Some(11), modified: None, name: None, ext: None }) };
        assert!(cond.matches(&ev, &ctx_std()));
        let ev2 = EventInfo { path: PathBuf::from("/tmp/a"), event: Event::Any, meta: Some(FileMeta { size: Some(9), modified: None, name: None, ext: None }) };
        assert!(!cond.matches(&ev2, &ctx_std()));
    }

    #[test]
    fn size_lt_with_meta_short_circuits() {
        let cond = SizeLtCondition::new(10);
        let ev = EventInfo { path: PathBuf::from("/tmp/a"), event: Event::Any, meta: Some(FileMeta { size: Some(9), modified: None, name: None, ext: None }) };
        assert!(cond.matches(&ev, &ctx_std()));
        let ev2 = EventInfo { path: PathBuf::from("/tmp/a"), event: Event::Any, meta: Some(FileMeta { size: Some(11), modified: None, name: None, ext: None }) };
        assert!(!cond.matches(&ev2, &ctx_std()));
    }

    #[test]
    fn size_checks_fallback_to_fs_metadata() {
        let dir = PathBuf::from("target/test_size");
        let _ = stdfs::create_dir_all(&dir);
        let file_small = stdfs::canonicalize(dir.join("small.bin")).unwrap_or(dir.join("small.bin"));
        stdfs::write(&file_small, vec![0u8; 5]).unwrap();

        let ev = EventInfo { path: file_small.clone(), event: Event::Any, meta: None };
        let ctx = ctx_std();
        assert!(SizeLtCondition::new(10).matches(&ev, &ctx));
        assert!(!SizeGtCondition::new(10).matches(&ev, &ctx));

        let file_big = stdfs::canonicalize(dir.join("big.bin")).unwrap_or(dir.join("big.bin"));
        stdfs::write(&file_big, vec![0u8; 20]).unwrap();
        let ev2 = EventInfo { path: file_big.clone(), event: Event::Any, meta: None };
        assert!(SizeGtCondition::new(10).matches(&ev2, &ctx));
        assert!(!SizeLtCondition::new(10).matches(&ev2, &ctx));
    }
}
