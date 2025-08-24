use crate::models::{Condition, EventInfo};
use glob::Pattern;
use regex::Regex;
use std::fs;

impl Condition {
    pub fn matches(&self, event: &EventInfo) -> bool {
        match self {
            Condition::Regex { value } => {
                if let Ok(regex) = Regex::new(value) {
                    event.path.to_str().map_or(false, |s| regex.is_match(s))
                } else {
                    false
                }
            }
            Condition::Glob { value } => {
                if let Ok(pattern) = Pattern::new(value) {
                    event.path.to_str().map_or(false, |s| pattern.matches(s))
                } else {
                    false
                }
            }
            Condition::Extension { value } => event
                .path
                .extension()
                .map_or(false, |e| e.to_str().unwrap_or("") == value),

            Condition::SizeGt { value } => {
                fs::metadata(&event.path).map_or(false, |m| m.len() as i64 > *value)
            }

            Condition::SizeLt { value } => {
                fs::metadata(&event.path).map_or(false, |m| (m.len() as i64) < *value)
            }

            Condition::Contains { text } => event.path.to_str().unwrap_or("").contains(text),
        }
    }
}
