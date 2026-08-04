use std::io;

fn main() {
    println!("Prime number checker");
    let value = match parse_user_input_as_u32() {
        Some(val) => val,
        None => {
            println!("Invalid input. Enter a Positive Number");
            return;
        }
    };

    println!("Is prime number: {}", is_prime(value as u32));
    println!("Prime Numbers up to {}: {:?}", value, prime_numbers(value as u32));
}


fn parse_user_input_as_u32() -> Option<u64> {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim().parse::<u64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}


fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    // Even numbers are not prime
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32 + 1;// to handle range
    // exclusiveness at the end
    for i in 3..limit {
        if n % i == 0 {// if divisible by any num other than 1 and n, it ain't prime    
            return false;
        }
    }
    
    true
}


fn prime_numbers(n: u32) -> Vec<u32> {
    let mut prime_numbers = Vec::<u32>::with_capacity(n as usize);
    for num in 2..=n {
        if is_prime(num) {
            prime_numbers.push(num);
        }
    }
    prime_numbers
}

