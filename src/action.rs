use crate::actions::{
    Action, ExecAction, ExecActionConfig, LogAction, MoveAction, MoveOverwritePolicy,
};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ActionConfig {
    Move {
        destination: String,
        #[serde(default)]
        overwrite: Option<MoveOverwritePolicy>,
    },
    Exec {
        command: String,
        #[serde(default)]
        args: Option<Vec<String>>,
        #[serde(default)]
        cwd: Option<String>,
        #[serde(default)]
        env: Option<Vec<(String, String)>>,
        #[serde(default)]
        timeout_secs: Option<u64>,
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
            ActionConfig::Exec {
                command,
                args,
                cwd,
                env,
                timeout_secs,
            } => Box::new(ExecAction::new(ExecActionConfig {
                command,
                args,
                cwd,
                env,
                timeout_secs,
            })),
            ActionConfig::Log { message } => Box::new(LogAction::new(message)),
        }
    }
}
