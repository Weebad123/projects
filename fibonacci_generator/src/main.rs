use std::io;

fn main() {
    println!("Fibonacci Generator CLI");
    let user_input = parse_user_input_as_u32();
// can match on the `parse_user_input_as_u32` instead of using if/else
    if user_input.is_some() {
        let fibonacci_sequence = generate_fibonacci(
            user_input.unwrap()
        );
        println!("Fibonacci Sequence: {:?}", fibonacci_sequence);

        let (even, odd) = split_even_odd(&fibonacci_sequence);
        println!("Evens: {:?}", even);
        println!("Odds: {:?}", odd);
    } else {
        // Handling None case
        println!("Invalid input. Please enter a positive integer!");
    }


}



fn parse_user_input_as_u32() -> Option<u32> {

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    // parsing as u32 might fail, so handle via match
    match user_input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn generate_fibonacci(number: u32) -> Vec<u64> {
    // the sequence will be a vector
    let mut sequence = Vec::<u64>::with_capacity(number as usize);
    if number >= 1 {
        sequence.push(0);
    }

    if number >= 2 {
        sequence.push(1);
    }

    // for loop 
    for i in 2..number {
        // 0, 1, 1, 2, 3, 5, 8, 13
        // sum of immediate previous and immediate double previous
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }

    sequence
}


fn split_even_odd(seq: &[u64]) -> (Vec<u64>, Vec<u64>) {
    let mut evens = 
    Vec::<u64>::with_capacity(seq.len());

    let mut odds = Vec::<u64>::with_capacity(seq.len());

    // iterate over the sequence
    for each_num in seq.iter() {
        if each_num % 2 == 0 {
            evens.push(*each_num);
        } else {
            odds.push(*each_num);
        }
    }

    (evens, odds)
}