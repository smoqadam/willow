use crate::models::EventInfo;

pub struct EventCondition {}

impl Condition for EventCondition
{
    fn matches(&self, event: &EventInfo) -> bool {
        true
    }
}