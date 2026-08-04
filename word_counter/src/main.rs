
use std::io::Read;

fn main() {
    
    // Take args
    let arguments: Vec<String> = std::env::args().collect();
    // for arg in &arguments {
    //     println!("Argument is: {}", arg);
    // }
    if arguments.len() != 2 {
        println!("Usage: cargo run --release <file_path>");
        return;
    }

    let file_path = &arguments[1];

    let mut file = match std::fs::File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file contents: {}", err);
        return;
    }


    let words = count_words(&contents);
    let lines = count_lines(&contents);
    let chars = count_chars(&contents);

    println!(
        "This file has {} words, {} lines and {} chars",
        words, lines, chars
    );
}



fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

fn count_chars(text: &str) -> usize {
    text.chars().count()
}