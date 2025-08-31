use crate::actions::Action;
use crate::engine::EngineCtx;
use crate::template::Template;
use log::{debug, info};
use std::path::Path;

pub struct LogAction {
    message: String,
}

impl LogAction {
    pub fn new(message: String) -> Self {
        LogAction { message }
    }
}

impl Action for LogAction {
    fn run(&self, path: &Path, _ctx: &EngineCtx) -> anyhow::Result<()> {
        debug!("Starting log action for path: {:?}", path);

        let template = Template::new(self.message.clone());
        let rendered_message = template.render(path);

        info!("Log: {}", rendered_message);
        Ok(())
    }
}
