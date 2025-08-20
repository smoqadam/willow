use crate::models::{Config, Rule};
use std::fs;
use std::io::Error;

pub fn load(path: String) -> Result<Config, Error> {
    println!("{:?}", path);
    let content = fs::read_to_string(path)?;
    // println!("{:?}", content);
    let config: Config = serde_json::from_str(content.as_str())?;
    println!("{:?}", config);

    Ok(config)
}
