use crate::models::{EventInfo, Condition};
use regex::Regex;
use glob::Pattern;
use std::fs;


impl Condition {
    pub fn matches(&self, event: &EventInfo) -> bool {
        match self {
            Condition::Regex { value } => {
                if let Ok(regex) = Regex::new(value) {
                    event.paths.iter().any(|p| {
                        p.to_str().map_or(false, |s| regex.is_match(s))
                    })
                } else {
                    false
                }
            }
            Condition::Glob { value } => {
                if let Ok(pattern) = Pattern::new(value) {
                    event.paths.iter().any(|p| {
                        p.to_str().map_or(false, |s| pattern.matches(s))
                    })
                } else {
                    false
                }
            }
            Condition::Extension { value } => {
                event.paths.iter().any(|p| {
                    p.extension().map_or(false, |e| e.to_str().unwrap_or("") == value)
                })
            }
            Condition::SizeGt { value } => {
                event.paths.iter().any(|p| {
                    fs::metadata(p).map_or(false, |m| m.len() as i64 > *value)
                })
            }
            Condition::SizeLt { value } => {
                event.paths.iter().any(|p| {
                    fs::metadata(p).map_or(false, |m| (m.len() as i64) < *value)
                })
            }
            Condition::Contains { text } => {
                event.paths.iter().any(|p| p.to_str().unwrap_or("").contains(text))
            }
        }
    }
}