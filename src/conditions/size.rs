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
    fn matches(&self, ev: &EventInfo, ctx: &EngineCtx) -> bool {
        if let Ok(metadata) = ctx.fs.metadata(&ev.path) {
            return metadata.len() as i64 > self.size;
        }
        false
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
    fn matches(&self, ev: &EventInfo, ctx: &EngineCtx) -> bool {
        if let Ok(metadata) = ctx.fs.metadata(&ev.path) {
            return (metadata.len() as i64) < self.size;
        }
        false
    }
}
