use crate::actions::Action;
use crate::engine::EngineCtx;
use crate::template::Template;
use log::{debug, info};
use std::path::Path;

pub struct RenameAction {
    template: String,
}

impl RenameAction {
    pub fn new(template: String) -> Self {
        RenameAction { template }
    }
}

impl Action for RenameAction {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()> {
        debug!(
            "Starting rename action for path: {:?} with template: {}",
            path, self.template
        );

        let template = Template::new(self.template.clone());
        let rendered_name = template.render(path);

        let parent_dir = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory for path {:?}", path))?;
        let new_path = parent_dir.join(&rendered_name);

        info!("Renaming {:?} to {:?}", path, new_path);
        ctx.fs.rename(path, &new_path)?;
        Ok(())
    }
}
