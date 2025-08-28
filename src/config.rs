use crate::models::Config;
use std::fs;
use anyhow::Result;

pub fn load(path: String) -> Result<Config> {
    // println!("{:?}", path);
    let content = fs::read_to_string(path)?;
    // println!("{:?}", content);
    let config: Config = serde_yaml::from_str(content.as_str())?;
    // println!("{:?}", config);
   Ok(config)
}
