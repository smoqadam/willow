
#[derive(Debug, Clone)]
pub struct Template {
    pub value: String,
}
impl Template {
    pub fn new(value: String) -> Self {
        Template { value }
    }
    pub fn render(&self) -> String {
        self.value.replace(
            "{datetime}",
            &chrono::Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string(),
        )
            .replace("{date}", &chrono::Utc::now().format("%Y-%m-%d").to_string())
    }
}
