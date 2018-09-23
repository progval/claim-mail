extern crate toml;

extern crate claim_mail;

use std::env::args_os;
use std::fs::File;
use std::io::prelude::*;

use claim_mail::config::{Config, example_config};

fn main() {
    let mut args = args_os();
    let bin = match args.next().map(|bin| bin.into_string()) {
        Some(Ok(bin)) => bin,
        _ => "claim-mail".to_string(),
    };
    let config_filename = match args.next() {
        Some(filename) => filename,
        None => {
            println!("Syntax: {} <config.toml>\n", bin);
            println!("where config.toml is a file formatted like this:");
            println!("{}", toml::to_string(&example_config()).unwrap());
            return;
        }
    };
    let mut config_file = File::open(config_filename).expect("config file not found");
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)
        .expect("something went wrong reading the config file");
    let config: Config = toml::from_str(&config_string)
        .unwrap();
    println!("{:?}", config);

}
