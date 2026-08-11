use std::io::{self, Write};

fn main() {
    println!("String manipulation tool");

    loop {
        println!("\nChoose an operation");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
        println!("4. Trim");
        println!("5. Find Substring");
        println!("6. Replace Text");
        println!("7. Exit");


        let prompt = user_prompt("Enter your choice: ");
        let parsed_input = match prompt.trim().parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Failed to parse user input: {}", e);
                return;
            }
        };

        match parsed_input {
            1 => {
                println!("Reverse the String");
                let s = user_prompt("Enter a String: ");
                println!("Reversed: {}", s.chars().rev().collect::<String>());
            },
            2 => {
                println!("Change to Uppercase");
                let s = user_prompt("Enter a String: ");
                println!("Uppercase: {}", s.to_uppercase());
            },
            3 => {
                println!("Change to Lowercase");
                let s = user_prompt("Enter a String: ");
                println!("Lowercase: {}", s.to_lowercase());
            },
            4 => {
                println!("Trim a String");
                let s = user_prompt("Enter string to trim: ");
                println!("Trimmed: {}", s.trim());
            },
            5 => {
                println!("Find substring");
                let s = user_prompt("Enter a string: ");
                let sub = user_prompt("Enter substring to find: ");
                if s.contains(&sub) {
                    println!("Substring found: {}", sub);
                } else {
                    println!("No substring found!");
                }
            },
            6 => {
                println!("Replace a string");
                let main_s = user_prompt("Enter main string: ");
                let text_to_replace = user_prompt("text to replace: ");
                let new = user_prompt("Enter replacement text: ");
                println!("Result: {}", main_s.replace(&text_to_replace, &new));
            }, 
            7 => {
                println!("Exiting.......");
                break;
            }
            _ => {
                println!("Invalid choice. Please select from the options");
            }
        }
    }
}



fn user_prompt(input: &str) -> String {
    print!("{}", input);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}
