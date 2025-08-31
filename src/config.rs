use crate::models::Config;
use anyhow::Result;
use std::fs;

pub fn load(path: String) -> Result<Config> {
    // println!("{:?}", path);
    let content = fs::read_to_string(path)?;
    // println!("{:?}", content);
    let config: Config = serde_yaml::from_str(content.as_str())?;
    // println!("{:?}", config);
    Ok(config)
}
