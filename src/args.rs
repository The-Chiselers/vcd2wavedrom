use std::env;

use clap::{arg, Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub vcd_file: String,
    #[arg(short, long)]
    pub config_file: String,
}

pub fn read_args() -> Args {
    Args::parse()
}

// pub fn read_args() -> Args {
//     let vcd_file: String = env::args().nth(1).expect("No VCD file provided");
//     let config_file: String = env::args().nth(2).expect("No config file provided");

//     Args {
//         vcd_file,
//         config_file,
//     }
// }

