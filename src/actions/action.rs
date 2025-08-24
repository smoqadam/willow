use crate::models::{Action, Event};
use std::fs;
use std::path::{Path, PathBuf};
use log::{debug, error, info};

pub struct ActionContext<'a> {
    pub path: &'a PathBuf,
    pub event: &'a Event,
}

pub trait ActionRunner {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()>;
}
impl Action {
    pub fn into_exec(self) -> Box<dyn ActionRunner> {
        debug!("Converting action to executable: {:?}", self);
        match self {
            Action::Move { destination } => {
                debug!("Creating MoveAction with destination: {}", destination);
                Box::new(MoveAction { destination })
            },
            Action::Rename { template } => {
                debug!("Creating RenameAction with template: {}", template);
                Box::new(RenameAction { template })
            },
            Action::Log { message } => {
                debug!("Creating LogAction with message: {}", message);
                Box::new(LogAction { message })
            },
        }
    }
}

pub struct MoveAction {
    pub destination: String,
}

impl ActionRunner for MoveAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        debug!("Starting move action for path: {:?}", ctx.path);
        debug!("Starting move action for event: {:?}", ctx.event);
        let dest_dir = Path::new(&self.destination);
        let filename = ctx
            .path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("No filename in path {:?}", ctx.path))?;

        let dest_path = dest_dir.join(filename);
        debug!("Moving {:?} to {:?}", ctx.path, dest_path);
        
        fs::rename(&ctx.path, &dest_path).map_err(|e| {
            error!("Move action error: {:?}", e);
            anyhow::anyhow!("Failed to move {:?} to {:?}: {}", ctx.path, dest_path, e)
        })?;
        info!("moved {:?} to {:?}", ctx.path, dest_path);
        Ok(())
    }
}

pub struct RenameAction {
    pub template: String,
}

impl ActionRunner for RenameAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        debug!("Starting rename action for path: {:?} with template: {}", ctx.path, self.template);
        info!("Renaming {:?} with template {}", ctx.path, self.template);
        fs::copy(ctx.path, &self.template)?;
        Ok(())
    }
}

pub struct LogAction {
    pub message: String,
}

impl ActionRunner for LogAction {
    fn run(&self, ctx: &ActionContext) -> anyhow::Result<()> {
        debug!("Starting log action for path: {:?}", ctx.path);
        info!("Log: {}", self.message);
        Ok(())
    }
}
