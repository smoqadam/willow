use std::path::Path;

#[derive(Debug, Clone)]
pub struct Template {
    pub value: String,
}

impl Template {
    pub fn new(value: String) -> Self {
        Template { value }
    }

    pub fn render(&self, path: &Path) -> String {
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let parent = path.parent().and_then(|p| p.to_str()).unwrap_or("");
        let full_path = path.to_str().unwrap_or("");

        self.value
            .replace(
                "{datetime}",
                &chrono::Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string(),
            )
            .replace("{date}", &chrono::Utc::now().format("%Y-%m-%d").to_string())
            .replace("{time}", &chrono::Utc::now().format("%H:%M:%S").to_string())
            .replace("{filename}", filename)
            .replace("{name}", name)
            .replace("{ext}", ext)
            .replace("{parent}", parent)
            .replace("{path}", full_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn renders_basic_placeholders() {
        let tpl = Template::new("{filename}|{name}|{ext}|{parent}|{path}".to_string());
        let path = PathBuf::from("/tmp/dir/file.txt");
        let out = tpl.render(&path);
        assert!(out.contains("file.txt|file|txt|/tmp/dir|/tmp/dir/file.txt"));
    }

    #[test]
    fn renders_time_placeholders_to_non_empty() {
        let tpl = Template::new("{date} {time} {datetime}".to_string());
        let path = PathBuf::from("/tmp/a");
        let out = tpl.render(&path);
        // Ensure placeholders are replaced (no braces remain)
        assert!(!out.contains("{date}"));
        assert!(!out.contains("{time}"));
        assert!(!out.contains("{datetime}"));
        assert!(!out.trim().is_empty());
    }
}
