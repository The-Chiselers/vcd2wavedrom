mod args;
mod config;
mod vcd;
mod wavedrom;

fn main() {
    let args: args::Args = args::read_args();
    let mut config: config::Config = config::Config::from_file(&args.config_file);
    let vcd: vcd::VCD = vcd::VCD::from_file(&args.vcd_file);

	if config.time_start.is_none() {
		config.time_start = Some(0);
	}
	if config.time_end.is_none() {
		config.time_end = Some(vcd.max_time() + 1);
	}

    let wavedrom: wavedrom::Wavedrom = wavedrom::Wavedrom::from_vcd(&vcd, &config);
	// println!("Wavedrom: {:?}", wavedrom);
    println!("{}", serde_json::to_string_pretty(&wavedrom).unwrap());
}
