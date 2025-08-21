use crate::models::{Config, EventInfo, Rule, RuleEngine};

pub fn from_event(event: &EventInfo, config: &Config) -> Vec<RuleEngine> {
    config
        .watchers
        .iter()
        .flat_map(|w| {
            w.rules
                .iter()
                .filter(|rule| rule.event == event.event)
                .cloned() // Rule
                .map(RuleEngine::from) // Rule -> RuleEngine
                .collect::<Vec<_>>()
        })
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
