use std::env;

#[derive(Debug)]
pub struct Args {
    pub vcd_file: String,
    pub config_file: String,
}

pub fn read_args() -> Args {
    let vcd_file: String = env::args().nth(1).expect("No VCD file provided");
    let config_file: String = env::args().nth(2).expect("No config file provided");

    Args {
        vcd_file,
        config_file,
    }
}