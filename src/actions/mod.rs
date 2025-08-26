use crate::models::EventInfo;

mod move_action;
mod rename;
mod log;

pub use move_action::MoveAction;
pub use rename::RenameAction;
pub use log::LogAction;

pub trait Action: Send + Sync {
    fn run(&self, event_info: &EventInfo) -> anyhow::Result<()>;
}
