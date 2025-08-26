use crate::conditions::Condition;
use crate::models::EventInfo;
use std::fs;

pub struct SizeGtCondition {
    size: i64,
}

impl SizeGtCondition {
    pub fn new(size: i64) -> Self {
        SizeGtCondition { size }
    }
}

impl Condition for SizeGtCondition {
    fn matches(&self, event_info: &EventInfo) -> bool {
        if let Ok(metadata) = fs::metadata(&event_info.path) {
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
    fn matches(&self, event_info: &EventInfo) -> bool {
        if let Ok(metadata) = fs::metadata(&event_info.path) {
            return (metadata.len() as i64) < self.size;
        }
        false
    }
}
