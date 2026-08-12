use std::io::{Write, self};

pub fn get_input(prompt: &str) -> String {
    println!("Prompt: {}", prompt);
    io::stdout().flush().unwrap();
    let mut new_input = String::new();
    io::stdin().read_line(&mut new_input).expect("Failed to capture user input");
    new_input.trim().to_string()
}

pub fn get_input_number(prompt: &str) -> Option<usize> {
    let input = get_input(prompt);
    if input.is_empty() {
        return None;
    }

    match input.parse::<usize>() {
        Ok(val) => return Some(val),
        Err(e) =>  {
            println!("Invalid input. Input must be a number: {}", e);
            return None;
        },
    }
}