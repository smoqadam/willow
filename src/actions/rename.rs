use crate::actions::Action;
use crate::template::Template;
use std::fs;
use std::path::PathBuf;
use log::{debug, info};

pub struct RenameAction {
    template: String,
}

impl RenameAction {
    pub fn new(template: String) -> Self {
        RenameAction { template }
    }
}

impl Action for RenameAction {
    fn run(&self, path: &PathBuf) -> anyhow::Result<()> {
        debug!("Starting rename action for path: {:?} with template: {}", path, self.template);
        
        let template = Template::new(self.template.clone());
        let rendered_name = template.render(path);
        
        let parent_dir = path.parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory for path {:?}", path))?;
        let new_path = parent_dir.join(&rendered_name);
        
        info!("Renaming {:?} to {:?}", path, new_path);
        fs::rename(&path, &new_path)?;
        Ok(())
    }
}
