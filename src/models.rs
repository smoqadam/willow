use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub(crate) watch: String,
    pub(crate) action: String,
}


#[derive(Deserialize, Debug)]
pub struct Config {
    pub rules: Vec<Rule>,
}