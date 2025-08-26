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
        let rendered_name = template.render();
        
        info!("Renaming {:?} with template {}", event_info.path, rendered_name);
        fs::copy(&event_info.path, &rendered_name)?;
        Ok(())
    }
}
