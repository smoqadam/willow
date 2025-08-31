use crate::engine::EngineCtx;
use crate::models::EventInfo;

mod contains;
mod extension;
mod glob;
mod regex;
mod size;

pub use contains::ContainsCondition;
pub use extension::ExtensionCondition;
pub use glob::GlobCondition;
pub use regex::RegexCondition;
pub use size::{SizeGtCondition, SizeLtCondition};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionKind {
    Static,
    Io,
}

pub trait Condition: Send + Sync {
    fn kind(&self) -> ConditionKind;
    fn matches(&self, ev: &EventInfo, ctx: &EngineCtx) -> bool;
}
