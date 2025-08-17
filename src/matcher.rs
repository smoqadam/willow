use crate::models::{Config, Rule};
use notify::Event;

pub fn apply(config: &Config, event: &Event) {
    println!("{:?} - {:?}", config, event)
}
