use crate::models::EventInfo;

mod event;
mod regex;
mod glob;
mod extension;
mod size;
mod contains;

pub use event::EventCondition;
pub use regex::RegexCondition;
pub use glob::GlobCondition;
pub use extension::ExtensionCondition;
pub use size::{SizeGtCondition, SizeLtCondition};
pub use contains::ContainsCondition;

pub trait Condition: Send + Sync {
    fn matches(&self, event_info: &EventInfo) -> bool;
}

