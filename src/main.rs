extern crate yaml_rust;
use std::fs;
use yaml_rust::{YamlLoader};

fn main() {
    // Parse command line arguments
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    
    // Get command line argument values
    let configfile = arguments.get::<String>("config").unwrap_or("./example-config.yml".to_string());

    println!("Backvol starting up...");
    println!("Config file location: {configfile}");

    // Load yaml from config file
    let filecontents = fs::read_to_string(configfile).expect("Should have been able to read the file");
    let config_yaml = YamlLoader::load_from_str(filecontents.as_str()).unwrap();
    let config = &config_yaml[0];

    // For debugging
    println!("{:?}", config);

    let rsync_location = config["rsync_location"].as_str().unwrap_or("rsync");
    println!("rsync location: {rsync_location}");
    
    println!("Backvol completed.");
}
