use std::path::Path;
use crate::engine::EngineCtx;

mod move_action;
mod rename;
mod log;

pub use move_action::MoveAction;
pub use rename::RenameAction;
pub use log::LogAction;

pub trait Action: Send + Sync {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()>;
}
