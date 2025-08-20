// use crate::models::{Config, Rule};
// use notify::Event;
//
// pub fn apply(config: &Config, event: &Event) {
//     /// iterate over config.rules and match the condition with event
//     for rule in &config.rules {
//         match rule.matches(&event) {
//             Ok(res) => {
//                 let a = Action::execute(res);
//             },
//             Err(e) => eprintln!("error {}", e),
//         }
//     }
// }
