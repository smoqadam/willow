use crate::conditions::Condition;
use crate::models::{Event, EventInfo};

pub struct EventCondition {
    event_type: Event,
}

impl EventCondition {
    pub fn new(event_type: Event) -> Self {
        EventCondition { event_type }
    }
}

impl Condition for EventCondition {
    fn matches(&self, event_info: &EventInfo) -> bool {
        event_info.event == self.event_type
    }
}
