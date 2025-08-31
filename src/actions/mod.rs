use crate::engine::EngineCtx;
use std::path::Path;

mod log;
mod move_action;
mod rename;

pub use log::LogAction;
pub use move_action::MoveAction;
pub use rename::RenameAction;

pub trait Action: Send + Sync {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()>;
}
