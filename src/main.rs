extern crate yaml_rust;
use std::fs;
use yaml_rust::YamlLoader;

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

    let rsync_location = config["rsync"]["location"].as_str().unwrap_or("rsync");
    let rsync_args = config["rsync"]["args"].as_str().unwrap_or("-a");
    println!("rsync command: {rsync_location} {rsync_args} <src> <dst>");

    let snapper_enabled = config["snapper"]["enabled"].as_bool().unwrap_or(false);
    let snapper_location = config["snapper"]["location"].as_str().unwrap_or("snapper");
    let snapper_cleanup = config["snapper"]["cleanup"].as_str().unwrap_or("number");
    if snapper_enabled {
        println!("snapper command: {snapper_location} -c <volume> create -c {snapper_cleanup}");
    } else {
        println!("snapper disabled");
    }

    println!(" ");
    for volume in config["volumes"].clone().into_iter() {
        let name = volume["name"].as_str().unwrap();
        let src = volume["src"].as_str().unwrap();
        let dst = volume["dst"].as_str().unwrap();
        println!("{name} [{src} ===> {dst}]");

        let rsync_test = format!("{rsync_location} -an --out-format=\"%f\" {src} {dst}");
        println!("{rsync_test}");

        let rsync_command = format!("{rsync_location} {rsync_args} {src} {dst}");
        println!("{rsync_command}");

        if snapper_enabled {
            let snapper_command = format!("{snapper_location} -c {name} create -c {snapper_cleanup}");
            println!("{snapper_command}");

        }

        println!(" ")
    }
    
    println!("Backvol completed.");
}
