use crate::models::Rule;

pub fn load(path: String) -> Vec<Rule> {
    let mut rules = Vec::new();
    rules.push(Rule {watch_path: path});

    rules
}