// {signal: [
//     {name: 'clk', wave: 'p.........'},
//     {name: 'dat', wave: 'x.23456x..', data: ['head', 'body', 'other', 'tail', 'data']},
//     {name: 'req', wave: '0.1..0.1.0'},
//     {},
//     {name: 'ack', wave: '1......01.'}
//   ]}

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::{config, wavedrom};
use crate::vcd::{BinaryUnit, StringUnit, WaveUnit};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wavedrom {
    pub signal: Vec<Signal>
}

impl Wavedrom {
    pub fn from_vcd(vcd: &crate::vcd::VCD, config: &crate::config::Config) -> Wavedrom {
        let mut vcd_signals: std::collections::HashMap<String, &crate::vcd::VCD> = std::collections::HashMap::new();
        let config_signals: &std::collections::HashMap<String, config::Signal> = &config.signals;

        for (signal_name, signal) in config_signals {
            let vcd_signal: &crate::vcd::VCD = vcd.find_signal(&signal_name).expect("Could not find signal in VCD");
			// if signal_name in config.clocks, then get Polarity from the signal, replace all chars with . and the first with either N or P

			vcd_signals.insert(signal_name.clone(), vcd_signal);
        }

        let mut wavedrom_signals: std::collections::HashMap<String, wavedrom::Signal> = std::collections::HashMap::new();
        for (signal_name, vcd_signal) in vcd_signals {
			// println!("Signal name: {signal_name}\n");
            let wave_vec: Vec<crate::vcd::WaveUnit> = vcd_signal.read_to_array(config.time_start.unwrap(), config.time_end.unwrap()).expect("Could not read signal to array");
            let mut signal: wavedrom::Signal = wavedrom::Signal::new(signal_name.clone());
			for wave_unit in wave_vec {
				signal.add_wave_unit(wave_unit);
			}
			wavedrom_signals.insert(signal_name.clone(), signal);

        }
        let mut wavedrom_signal_vec: Vec<wavedrom::Signal> = Vec::new();
		for (signal_name, signal) in wavedrom_signals {
			let clocks = &config.clocks;
			if clocks.contains_key(&signal_name) {
				let mut new_clock_wave: String = String::new();
				// peak first char, if 1 then P, else N
				if signal.wave.chars().next().unwrap() == '1' {
					new_clock_wave.push('P');
				} else {
					new_clock_wave.push('N');
				}
				for _ in 1..signal.wave.len() {
					new_clock_wave.push('.');
				}
				let new_signal = wavedrom::Signal {
					name: signal_name.clone(),
					wave: new_clock_wave,
					data: signal.data.clone()
				};
				wavedrom_signal_vec.push(new_signal);
			} else {
				wavedrom_signal_vec.push(signal);
			}
		}
		Wavedrom {
			signal: wavedrom_signal_vec
		}
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signal {
	pub name: String,
	pub wave: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WaveType {
	Binary,
	Hex,
}

impl Signal {
	pub fn new(name: String) -> Signal {
		Signal {
			name,
			wave: String::new(),
			data: None
		}
	}

	pub fn add_wave_unit(&mut self, wave_unit: WaveUnit) {
		match wave_unit {
			WaveUnit::Binary(binary_wave_unit) => {
				match binary_wave_unit {
					BinaryUnit::Bool(bool_value) => {
						if bool_value {
							self.wave.push('1');
						} else {
							self.wave.push('0');
						}
					}
					BinaryUnit::Same => {
						self.wave.push('.');
					}
				}
			}
			WaveUnit::String(string_wave_unit) => {
				match string_wave_unit {
					StringUnit::String(string_value) => {
						let string_value_as_str: String = string_value.to_string();
						if self.data.is_none() {
							self.data = Some(Vec::new());
						}
						self.data.as_mut().unwrap().push(string_value_as_str);
						self.wave.push('=');
					}
					StringUnit::Same => {
						self.wave.push('.');
					}
				}
			}
		}
	}
}


