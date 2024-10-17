mod args;
mod config;
mod vcd;
mod wavedrom;

fn main() {
    let args: args::Args = args::read_args();
    let config: config::Config = config::Config::from_file(&args.config_file);
    let vcd: vcd::VCD = vcd::VCD::from_file(&args.vcd_file);
    let wavedrom: wavedrom::Wavedrom = wavedrom::Wavedrom::from_vcd(&vcd, &config);
	println!("Wavedrom: {:?}", wavedrom);
    println!("{}", serde_json::to_string_pretty(&wavedrom).unwrap());
}
