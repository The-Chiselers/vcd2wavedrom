

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct VCD {
    pub name: String,
    #[serde(rename = "type")]
    pub vcd_type: VCDType,
    pub children: Option<Vec<VCD>>,
    pub data: Option<Vec<(u64, String)>>
}



impl VCD {
    pub fn from_file(vcd_file: &str) -> VCD {
        let vcd_string: String = std::fs::read_to_string(vcd_file).expect("Could not read VCD file");
        let vcd: VCD = serde_json::from_str(&vcd_string).expect("Could not parse VCD file");
        vcd
    }

    pub fn find_signal(&self, signal_name: &str) -> Option<&VCD> {
        if self.name == signal_name {
			// if signal_name == "reset" {
			// 	println!("Found reset\n");
			// }
			if self.data.is_some() && self.data.as_ref().unwrap().len() > 0 {
				return Some(self);
			}
            return Some(self);
        }
        if let Some(children) = &self.children {
            for child in children {
                if let Some(signal) = child.find_signal(signal_name) {
					return Some(signal);
                }
            }
        }
        None
    }

    pub fn read_to_array(&self, start_time: usize, end_time: usize) -> Option<Vec<String>> {
        // [(time, value), (time2, value2), ...]
        // time 2 can be much greater than time 1, the values in between are assumed to be the same as time 1
        let mut data: Vec<String> = Vec::new();
        let mut prev_value: &String = &String::new();
        let mut prev_time: usize = 0;

		let name = &self.name;
		if name == "io_apb_PSEL" {
			println!("Found io_apb_PSEL\n");
		}

        if self.data.is_none() {
            return None;
        }
        let input_data: &Vec<(u64, String)> = self.data.as_ref().unwrap();
        
        let mut input_data_index: usize = 0;
        while (prev_time <= start_time) {
			input_data_index += 1;
			if (input_data_index >= input_data.len()) {
				break;
			}
            prev_time = input_data[input_data_index].0 as usize;
            prev_value = &input_data[input_data_index].1;
        }
        input_data_index -= 1;
        prev_time = input_data[input_data_index].0 as usize;
        prev_value = &input_data[input_data_index].1;

        // At this point, prev_time is either the value at start_time or the value before start_time
        let mut current_value: String = prev_value.clone();
        let mut current_time: usize = start_time;
        let mut next_index: usize = input_data_index + 1;

        data.push(current_value.clone());
        while (current_time < end_time) {
			if (next_index >= input_data.len()) {
				data.push(".".to_string());
				current_time += 1;
				continue;
			}
			let next_time: usize = input_data[next_index].0 as usize;
			let next_value: &String = &input_data[next_index].1;
			if (current_time == next_time) {
				current_value = next_value.clone();
				data.push(current_value.clone());
				current_time += 1;
				next_index += 1;
				continue;
			}
			if (current_time < next_time) {
				data.push(".".to_string());
				current_time += 1;
				continue;
			}

        }

        Some(data)
    }   
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VCDType {
    name: VCDTypeInner,
    width: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VCDTypeInner {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "wire")]
    Wire,
}



