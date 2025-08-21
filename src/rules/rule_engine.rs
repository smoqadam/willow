use crate::models::{Config, EventInfo, Rule};

pub fn from_event<'a>(event: &EventInfo, config: &'a Config) -> Vec<&'a Rule> {
    config.watchers
        .iter()
        .flat_map(|w| &w.rules)
        .filter(|rule| rule.matches(event))
        .collect()
}

impl Rule {
    pub fn matches(&self, event: &EventInfo) -> bool {
        if self.event != event.event {
            return false;
        }
        self.conditions.iter().all(|cond| cond.matches(event))
    }
}