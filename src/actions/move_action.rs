use crate::actions::Action;
use crate::models::EventInfo;
use std::fs;
use std::path::Path;
use log::{debug, error, info};

pub struct MoveAction {
    destination: String,
}

impl MoveAction {
    pub fn new(destination: String) -> Self {
        MoveAction { destination }
    }
}

impl Action for MoveAction {
    fn run(&self, event_info: &EventInfo) -> anyhow::Result<()> {
        debug!("Starting move action for path: {:?}", event_info.path);
        debug!("Starting move action for event: {:?}", event_info.event);
        
        let dest_dir = Path::new(&self.destination);
        let filename = event_info
            .path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("No filename in path {:?}", event_info.path))?;

        let dest_path = dest_dir.join(filename);
        debug!("Moving {:?} to {:?}", event_info.path, dest_path);

        fs::rename(&event_info.path, &dest_path).map_err(|e| {
            error!("Move action error: {:?}", e);
            anyhow::anyhow!("Failed to move {:?} to {:?}: {}", event_info.path, dest_path, e)
        })?;
        info!("moved {:?} to {:?}", event_info.path, dest_path);
        Ok(())
    }
}
