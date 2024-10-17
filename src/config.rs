use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub signals: Signals,
    pub time_start: u32,
    pub time_end: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signals {
    pub signals: HashMap<String, Signal>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signal {
    pub rename: String,
}

pub fn read_config(config_file: &str) -> Config {
    let config: Config = serde_json::from_str(&std::fs::read_to_string(config_file).expect("Could not read config file")).expect("Could not parse config file");
    config
}
