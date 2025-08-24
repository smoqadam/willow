use crate::actions::ActionRunner;
use crate::models::{Config, Event, EventInfo, Rule};

pub struct RuleEngine {
    pub event: Event,
    pub actions: Vec<Box<dyn ActionRunner>>,
}

impl From<Rule> for RuleEngine {
    fn from(raw: Rule) -> Self {
        RuleEngine {
            event: raw.event,
            actions: raw.actions.into_iter()
                .map(|a| a.into_exec()) // turn into trait objects
                .collect(),
        }
    }
}


pub fn from_event(event: &EventInfo, config: &Config) -> Vec<RuleEngine> {
    config
        .watchers
        .iter()
        .flat_map(|w| {
            w.rules
                .iter()
                .filter(|rule| rule.matches(event))
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
