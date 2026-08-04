use std::{cmp::Ordering, io::Write};


fn main() {
    
    loop {

        secret_and_guess();

        if play_again() {
            continue;
        } else {
            break;
        }
    }


}


fn choose_level() -> u32 {

    println!("\nSelect difficulty level: ");
    println!("1: Easy");
    println!("2: Medium");
    println!("3: Hard");
    println!("4: Exit");
    println!("Enter your choice: (1 - 4) ");
    // flush out the user input after this
    std::io::stdout().flush().unwrap();

    // accepting new input after flushing out buffered stream
    let mut level_input = String::new();
    std::io::stdin().read_line(&mut level_input).expect("Failed to read user input");
    // ensure user entered 1 to 4
    let level: u32 = match level_input.trim().parse() {
        Ok(n) if (1..=4).contains(&n) => n,
        _ => {
            println!("Invalid choice. Enter 1 or 2 or 3 or 4");
            return 0;
        }
    };

    if level == 4 {
        println!("Goodbye!");
        std::process::exit(1);
    }

    level

    
}

fn secret_and_guess() {
    let selected_level = choose_level();
 
    let (max_number, max_attempts) = match selected_level {
        1 => (20, 10),
        2 => (50, 8),
        3 => (100, 5),
        _ => unreachable!()
    };

    let secret_number = rand::random_range(0..=max_number);
    
    println!(
        "I am thinking of a number between 1 and {}. You have {} attempts to guess it.",
        max_number, max_attempts
    );

    let mut guessed_correctly = false;

    for attempt in 1..=max_attempts {
        print!("\nAttempt {}/{} - Please input your guess: ", attempt, max_attempts);
        std::io::stdout().flush().unwrap();// Flush above input stream

        let mut guess_user_input = String::new();
        std::io::stdin().read_line(&mut guess_user_input).expect("Failed to read user input");
        let guess: u32 = match guess_user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Enter a valid number");
                continue;// as long as user hasnot exhausted attempts
            }
        };

        if guess < 1 || guess > max_number {
            println!("Your guess is out of range ( 1 -{}).", max_number);
            continue;
        }

        println!("You guessed: {}", guess);

        // Match guess to secret number

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again"),
            Ordering::Greater => println!("Too much! Try again"),
            Ordering::Equal =>  {
                println!("Congratulations! YOu guessed the number in {} attempts.", attempt);
                
                guessed_correctly = true;
                break;
            }
        }
    }

    if !guessed_correctly {
        println!("\nYou have used all your {} attempts. The number was {}", max_attempts, secret_number);
    }
}


fn play_again() -> bool {

    println!("\nPlay Again?? Yes/ No");

    std::io::stdout().flush().unwrap();
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).expect("Failed to read user input");

    match answer.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => {
            println!("Thanks for playing.");
            false
        }
    }
}