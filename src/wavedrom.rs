// {signal: [
//     {name: 'clk', wave: 'p.........'},
//     {name: 'dat', wave: 'x.23456x..', data: ['head', 'body', 'other', 'tail', 'data']},
//     {name: 'req', wave: '0.1..0.1.0'},
//     {},
//     {name: 'ack', wave: '1......01.'}
//   ]}

use serde::{Deserialize, Serialize};

use crate::{config, wavedrom};

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
            vcd_signals.insert(signal_name.clone(), vcd_signal);
        }

        let mut wavedrom_signals: std::collections::HashMap<String, wavedrom::Signal> = std::collections::HashMap::new();
        for (signal_name, vcd_signal) in vcd_signals {
			println!("Signal name: {signal_name}\n");
            let prewave: Vec<String> = vcd_signal.read_to_array(config.time_start, config.time_end).expect("Could not read signal to array");
            println!("{:?}\n######################################", prewave);


            // let wavedrom_signal: wavedrom::Signal = wavedrom::Signal {
            //     name: signal_name.clone(),
            //     wave: "x".to_string(),
            //     data: None
            // };
            // wavedrom_signals.insert(signal_name.clone(), wavedrom_signal);
        }
        todo!();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signal {
    pub name: String,
    pub wave: String,
    pub data: Option<Vec<String>>
}

