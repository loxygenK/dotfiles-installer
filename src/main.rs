mod config;

use std::io::prelude::*;
use std::fs::File;
use config::InstallConfig;

fn main() {
    let target = std::env::args()
        .skip(1)
        .next()
        .expect("File name to parse");

    let mut config_file = String::new();
    File::open(&target)
        .expect("Error occurred when opening file: not existing file?")
        .read_to_string(&mut config_file)
        .expect("Error occurred when reading the file");

    let config: InstallConfig = serde_yaml::from_str(&&config_file)
        .expect("Unable to parse yaml file");
    
    println!("{:?}", config);
}
