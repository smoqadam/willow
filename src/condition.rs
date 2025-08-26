use std::collections::HashMap;
use std::path::PathBuf;
use serde_derive::Deserialize;
use crate::models::{Event, EventInfo};



#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Condition {
    Event { value: Event },
    Regex { value: String },
    Glob { value: String },
    Extension { value: String },
    SizeGt { value: i64 },
    SizeLt { value: i64 },
    Contains { value: String },
}


