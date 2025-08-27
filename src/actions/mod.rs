use std::path::PathBuf;

mod move_action;
mod rename;
mod log;

pub use move_action::MoveAction;
pub use rename::RenameAction;
pub use log::LogAction;

pub trait Action: Send + Sync {
    fn run(&self, path: &PathBuf) -> anyhow::Result<()>;
}
