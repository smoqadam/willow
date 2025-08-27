use crate::conditions::Condition;
use std::fs;
use std::path::PathBuf;

pub struct SizeGtCondition {
    size: i64,
}

impl SizeGtCondition {
    pub fn new(size: i64) -> Self {
        SizeGtCondition { size }
    }
}

impl Condition for SizeGtCondition {
    fn matches(&self, path: &PathBuf) -> bool {
        if let Ok(metadata) = fs::metadata(&path) {
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
    fn matches(&self, path: &PathBuf) -> bool {
        if let Ok(metadata) = fs::metadata(&path) {
            return (metadata.len() as i64) < self.size;
        }
        false
    }
}
