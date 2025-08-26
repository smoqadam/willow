
use crate::models::EventInfo;

#[derive(Debug, Clone)]
pub struct Template {
    pub value: String,
}

impl Template {
    pub fn new(value: String) -> Self {
        Template { value }
    }
    
    pub fn render(&self, event_info: &EventInfo) -> String {
        let path = &event_info.path;
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let name = path.file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let parent = path.parent()
            .and_then(|p| p.to_str())
            .unwrap_or("");
        let full_path = path.to_str().unwrap_or("");
        
        self.value
            .replace("{datetime}", &chrono::Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string())
            .replace("{date}", &chrono::Utc::now().format("%Y-%m-%d").to_string())
            .replace("{time}", &chrono::Utc::now().format("%H:%M:%S").to_string())
            .replace("{filename}", filename)
            .replace("{name}", name)
            .replace("{ext}", ext)
            .replace("{parent}", parent)
            .replace("{path}", full_path)
    }
}
