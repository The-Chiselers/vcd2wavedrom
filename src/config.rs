use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub signals: Vec<Signal>,
	pub clocks: HashMap<String, String>,
    pub time_start: Option<usize>,
    pub time_end: Option<usize>,
}

impl Config {
    pub fn from_file(config_file: &str) -> Config {
        let config_string: String = std::fs::read_to_string(config_file).expect("Could not read config file");
        let config: Config = serde_json::from_str(&config_string).expect("Could not parse config file");
        config
    }
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Signals {
//     pub signals: HashMap<String, Signal>,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Signal {
	pub name: String,
    pub rename: String,
}


// pub fn example_config() -> Config {
//     let mut signals = HashMap::new();
//     signals.insert("signal1".to_string(), Signal { rename: "Signal 1".to_string() });
//     signals.insert("signal2".to_string(), Signal { rename: "Signal 2".to_string() });
//     Config {
//         signals: signals,
//         time_start: 0,
//         time_end: 100,
//     }
// }
