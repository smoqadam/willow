use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{Action, Event, Rule};
//
// pub trait Action {
//     fn execute();
// }

pub struct ActionContext<'a> {
    pub paths: &'a [PathBuf],
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

        for p in ctx.paths {
            let filename = p.file_name()
                .ok_or_else(|| anyhow::anyhow!("No filename in path {:?}", p))?;

            let dest_path = dest_dir.join(filename);
            fs::rename(p, &dest_path)
                .map_err(|e| anyhow::anyhow!("Failed to move {:?} to {:?}: {}", p, dest_path, e))?;
        }

        Ok(())
    }
}

pub struct RenameAction {
    pub template: String,
}

impl ActionRunner for RenameAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        println!("Renaming {:?} with template {}", ctx.paths, self.template);
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
