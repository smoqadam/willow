use crate::actions::{Action, LogAction, MoveAction, RenameAction};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ActionConfig {
    Move { destination: String },
    Rename { template: String },
    Log { message: String },
}

impl ActionConfig {
    pub fn into_action(self) -> Box<dyn Action> {
        match self {
            ActionConfig::Move { destination } => Box::new(MoveAction::new(destination)),
            ActionConfig::Rename { template } => Box::new(RenameAction::new(template)),
            ActionConfig::Log { message } => Box::new(LogAction::new(message)),
        }
    }
}
