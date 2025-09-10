use crate::engine::EngineCtx;
use std::path::Path;

mod log;
mod move_action;

pub use log::LogAction;
pub use move_action::{MoveAction, MoveOverwritePolicy};

pub trait Action: Send + Sync {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()>;
}
