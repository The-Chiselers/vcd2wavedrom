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
use crate::vcd::{WaveUnit};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wavedrom {
    pub signal: Vec<Signal>
}

impl Wavedrom {
    pub fn from_vcd(vcd: &crate::vcd::VCD, config: &crate::config::Config) -> Wavedrom {
        let mut vcd_signals: Vec<(String, &crate::vcd::VCD)> = Vec::new();
        let config_signals: &Vec<config::Signal> = &config.signals;

        for (signal) in config_signals {
			let mut signal_name = signal.name.clone();
            let vcd_signal: &crate::vcd::VCD = vcd.find_signal(&signal_name).expect("Could not find signal in VCD");
			for config_signal in config_signals {
				if config_signal.name == signal_name {
					signal_name = config_signal.rename.clone();
				}
			}
			vcd_signals.push((signal_name.clone(), vcd_signal));
        }

        let mut wavedrom_signals: std::collections::HashMap<String, wavedrom::Signal> = std::collections::HashMap::new();
        for (index, (signal_name, vcd_signal)) in vcd_signals.iter().enumerate() {
			// println!("Signal name: {signal_name}\n");
            let wave_vec: Vec<crate::vcd::WaveUnit> = vcd_signal.read_to_array(config.time_start.unwrap(), config.time_end.unwrap()).expect("Could not read signal to array");
            let mut signal: wavedrom::Signal = wavedrom::Signal::new(signal_name.clone(), index);
			for wave_unit in wave_vec {
				signal.add_wave_unit(wave_unit);
			}
			wavedrom_signals.insert(signal_name.clone(), signal);

        }
        let mut wavedrom_signal_vec: Vec<wavedrom::Signal> = Vec::new();
		for (signal_name, signal) in wavedrom_signals {
			// if signal_name in config.signals, then rename it
			let mut signal_name = signal_name.clone();
			let clocks = &config.clocks;
			if clocks.contains_key(&signal_name) {
				let mut new_clock_wave: String = String::new();
				// peak first char, if 1 then P, else N
				// if signal.wave.chars().next().unwrap() == '1' {
				// 	new_clock_wave.push('P');
				// } else {
				// 	new_clock_wave.push('N');
				// }
				new_clock_wave.push('P');
				for _ in 1..signal.wave.len() {
					new_clock_wave.push('.');
				}
				let new_signal = wavedrom::Signal {
					name: signal_name.clone(),
					wave: new_clock_wave,
					data: signal.data.clone(),
					order: signal.order,
				};
				wavedrom_signal_vec.push(new_signal);
			} else {
				wavedrom_signal_vec.push(signal);
			}
		}
		wavedrom_signal_vec.sort_by(|a, b| a.order.cmp(&b.order));
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
	pub order: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WaveType {
	Binary,
	Hex,
}

impl Signal {
	pub fn new(name: String, order: usize) -> Signal {
		Signal {
			name,
			wave: String::new(),
			data: None,
			order,
		}
	}

	pub fn add_wave_unit(&mut self, wave_unit: WaveUnit) {
		match wave_unit {
			WaveUnit::Binary(bool_value) => {
				if bool_value {
					self.wave.push('1');
				} else {
					self.wave.push('0');
				}
			}
			WaveUnit::Hex(hex_string) => {
				if self.data.is_none() {
					self.data = Some(Vec::new());
				}
				self.wave.push('=');
				self.data.as_mut().unwrap().push(hex_string);
			}
			WaveUnit::Same => {
				self.wave.push('.');
			}
		}
	}
}


