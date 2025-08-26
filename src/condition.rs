use serde_derive::Deserialize;
use crate::conditions::{
    Condition, RegexCondition, GlobCondition, 
    ExtensionCondition, SizeGtCondition, SizeLtCondition, ContainsCondition
};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConditionConfig {
    Regex { value: String },
    Glob { value: String },
    Extension { value: String },
    SizeGt { value: i64 },
    SizeLt { value: i64 },
    Contains { value: String },
}

impl ConditionConfig {
    pub fn into_condition(self) -> anyhow::Result<Box<dyn Condition>> {
        match self {
            ConditionConfig::Regex { value } => Ok(Box::new(RegexCondition::new(value)?)),
            ConditionConfig::Glob { value } => Ok(Box::new(GlobCondition::new(value)?)),
            ConditionConfig::Extension { value } => Ok(Box::new(ExtensionCondition::new(value))),
            ConditionConfig::SizeGt { value } => Ok(Box::new(SizeGtCondition::new(value))),
            ConditionConfig::SizeLt { value } => Ok(Box::new(SizeLtCondition::new(value))),
            ConditionConfig::Contains { value } => Ok(Box::new(ContainsCondition::new(value))),
        }
    }
}

