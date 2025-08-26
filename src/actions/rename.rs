use crate::actions::Action;
use crate::models::EventInfo;
use crate::template::Template;
use std::fs;
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
    fn run(&self, event_info: &EventInfo) -> anyhow::Result<()> {
        debug!("Starting rename action for path: {:?} with template: {}", event_info.path, self.template);
        
        let template = Template::new(self.template.clone());
        let rendered_name = template.render(event_info);
        
        let parent_dir = event_info.path.parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory for path {:?}", event_info.path))?;
        let new_path = parent_dir.join(&rendered_name);
        
        info!("Renaming {:?} to {:?}", event_info.path, new_path);
        fs::rename(&event_info.path, &new_path)?;
        Ok(())
    }
}
