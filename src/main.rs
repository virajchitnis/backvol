extern crate yaml_rust;
use std::fs;
use yaml_rust::{YamlLoader};

fn main() {
    // Parse command line arguments
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    
    // Get command line argument values
    let configfile = arguments.get::<String>("config").unwrap_or("./example-config.yml".to_string());

    println!("Backvol starting...");
    println!("Config file location: {configfile}");

    // Load yaml from config file
    let filecontents = fs::read_to_string(configfile).expect("Should have been able to read the file");
    let config_yaml = YamlLoader::load_from_str(filecontents.as_str()).unwrap();
    let config = &config_yaml[0];

    println!("rsync: ");
    let rsync_location = config["rsync"]["location"].as_str().unwrap_or("rsync");
    println!("\tlocation: {rsync_location}");

    let rsync_args = config["rsync"]["args"].as_str().unwrap_or("-a");
    println!("\targs: {rsync_args}");

    println!("snapper: ");
    let snapper_enabled = config["snapper"]["enabled"].as_bool().unwrap_or(false);
    println!("\tenabled: {snapper_enabled}");

    if snapper_enabled {
        let snapper_location = config["snapper"]["location"].as_str().unwrap_or("snapper");
        println!("\tlocation: {snapper_location}");

        let snapper_cleanup = config["snapper"]["cleanup"].as_str().unwrap_or("number");
        println!("\tcleanup: {snapper_cleanup}");
    }

    println!("Volumes: ");
    for volume in config["volumes"].clone().into_iter() {
        let name = volume["name"].as_str().unwrap();
        let src = volume["src"].as_str().unwrap();
        let dst = volume["dst"].as_str().unwrap();
        println!("\t{name} [{src} ===> {dst}]");
    }
    
    println!("Backvol completed.");
}
