use std::io::Write;



fn main() {
    
    println!("Simple calculation across every type");

    loop {
        println!("\nEnter your expression: (e.g., 5 + 3: ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read user input");

        // we expect 3 different expressions plus a whitespace
        let tokens: Vec<&str> = user_input.trim().split_whitespace().collect();
        if tokens.len() != 3 {
            println!("Invalid input. follow the format: number operator number");
            return;
        }

        let num1: f64 = match tokens[0].trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid first number.");
                return;
            }
        };

        let operator = tokens[1];

        let num2: f64 = match tokens[2]/*.trim()*/.parse()/*there is no leading or trailing whitespace here */ {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid second number. ");
                return;
            }
        };


        let resulting_value = match operator {
            "+" => add(num1, num2),
            "-" => sub(num1, num2),
            "*" => multiply(num1, num2),
            "/" => divide(num1, num2),
            _ => {
                println!("Invalid operator. Use +, -, *, or /");
                return;
            }
        };

        println!("Result: {:.2}", resulting_value);
        break;
    }
}



fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sub<T: std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

fn multiply<T: std::ops::Mul<Output = E>, E>(a: T, b: T) -> E {
    a * b
}

fn divide<T: PartialEq +Default + std::ops::Div<Output = E>, E>(a: T, b: T) -> E {
    if b == T::default() {
        println!("Division by 0 is not allowed:");
        std::process::exit(1);
    }

    a / b
}