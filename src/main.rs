mod args;
mod config;

use args::*;

fn main() {
    let args: Args = read_args();
    let config: config::Config = config::read_config(&args.config_file);
    println!("{:?}", config);
}
