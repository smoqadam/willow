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
    fn kind(&self) -> crate::conditions::ConditionKind { crate::conditions::ConditionKind::Io }
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
    fn kind(&self) -> crate::conditions::ConditionKind { crate::conditions::ConditionKind::Io }
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
