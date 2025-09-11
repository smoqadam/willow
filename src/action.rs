use crate::actions::{Action, LogAction, MoveAction, MoveOverwritePolicy};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ActionConfig {
    Move {
        destination: String,
        #[serde(default)]
        overwrite: Option<MoveOverwritePolicy>,
    },
    Log {
        message: String,
    },
}

impl ActionConfig {
    pub fn into_action(self) -> Box<dyn Action> {
        match self {
            ActionConfig::Move {
                destination,
                overwrite,
            } => Box::new(MoveAction::new(destination, overwrite)),
            ActionConfig::Log { message } => Box::new(LogAction::new(message)),
        }
    }
}
