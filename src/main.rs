mod args;
mod config;

use args::*;

fn main() {
    let args: Args = read_args();
    let config: config::Config = config::read_config(&args.config_file);
    // let config: config::Config = config::example_config();
    // let config_string: String = serde_json::to_string_pretty(&config).expect("Could not serialize config");
    // println!("{}", config_string);
    println!("{:?}", config);
}
