use crate::actions::Action;
use crate::template::Template;
use crate::engine::EngineCtx;
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
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()> {
        debug!("Starting move action for path: {:?}", path);

        let template = Template::new(self.destination.clone());
        let rendered_destination = template.render(path);
        
        let dest_path = Path::new(&rendered_destination);
        
        // append the filename if the rendered destination is a directory
        let final_dest_path = if rendered_destination.ends_with('/') || rendered_destination.ends_with('\\') {
            let filename = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("No filename in path {:?}", path))?;
            dest_path.join(filename)
        } else {
            dest_path.to_path_buf()
        };
        
        debug!("Moving {:?} to {:?}", path, final_dest_path);

        // create parent directory if it doesn't exist
        if let Some(parent) = final_dest_path.parent() {
            ctx.fs.create_dir_all(parent)?;
        }

        // todo: check for overwrite
        ctx.fs.rename(path, &final_dest_path).map_err(|e| {
            error!("Move action error: {:?}", e);
            anyhow::anyhow!("Failed to move {:?} to {:?}: {}", path, final_dest_path, e)
        })?;
        info!("moved {:?} to {:?}", path, final_dest_path);
        Ok(())
    }
}
