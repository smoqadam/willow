use crate::models::{Action, Event, Rule};
use std::fs;
use std::path::{Path, PathBuf};
//
// pub trait Action {
//     fn execute();
// }

pub struct ActionContext<'a> {
    pub path: &'a PathBuf,
    pub event: &'a Event,
}

pub trait ActionRunner {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()>;
}
impl Action {
    pub fn into_exec(self) -> Box<dyn ActionRunner> {
        match self {
            Action::Move { destination } => Box::new(MoveAction { destination }),
            Action::Rename { template } => Box::new(RenameAction { template }),
            Action::Log { message } => Box::new(LogAction { message }),
        }
    }
}

pub struct MoveAction {
    pub destination: String,
}

impl ActionRunner for MoveAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        let dest_dir = Path::new(&self.destination);


        let filename = ctx
            .path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("No filename in path {:?}", ctx.path))?;

        let dest_path = dest_dir.join(filename);
        println!("Move {:?} to {:?}", filename, dest_path.to_str());
        fs::rename(&ctx.path, &dest_path).map_err(|e| {
            anyhow::anyhow!("Failed to move {:?} to {:?}: {}", ctx.path, dest_path, e)
        })?;

        Ok(())
    }
}

pub struct RenameAction {
    pub template: String,
}

impl ActionRunner for RenameAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        println!("Renaming {:?} with template {}", ctx.path, self.template);
        Ok(())
    }
}

pub struct LogAction {
    pub message: String,
}

impl ActionRunner for LogAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        println!("Log: {}", self.message);
        Ok(())
    }
}
