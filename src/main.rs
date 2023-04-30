fn main() {
    // Parse command line arguments
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    
    // Get command line argument values
    let configfile = arguments.get::<String>("config").unwrap_or("./config.yml".to_string());

    // Start execution logic
    println!("Backvol starting up...");
    println!("Config file location: {configfile}");
    println!("Backvol completed.");
}
