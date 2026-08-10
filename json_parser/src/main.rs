use std::{env, fs};
use serde_json::Value;


fn main() {
    println!("Simple JSON Parser");
    parse_json_file();
}


fn parse_json_file()  {

    // collect command line arguments
    let args  = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <path_to_json_file> ");
        return;
    }

    // path_to_json_file via the command-line argument
    let path_to_file = &args[1];

    match fs::read_to_string(path_to_file) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json) => println!("Parsed JSON:\n{}", serde_json::to_string_pretty(&json).unwrap()),
            Err(e) => eprintln!("Invalid JSON: {}", e),
        },
        Err(err) => eprintln!("Failed to read file: {}", err),
    }
}