use crate::actions::Action;
use crate::models::EventInfo;
use crate::template::Template;
use log::{debug, info};

pub struct LogAction {
    message: String,
}

impl LogAction {
    pub fn new(message: String) -> Self {
        LogAction { message }
    }
}

impl Action for LogAction {
    fn run(&self, event_info: &EventInfo) -> anyhow::Result<()> {
        debug!("Starting log action for path: {:?}", event_info.path);
        
        let template = Template::new(self.message.clone());
        let rendered_message = template.render(event_info);
        
        info!("Log: {}", rendered_message);
        Ok(())
    }
}
