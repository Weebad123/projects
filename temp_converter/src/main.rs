use std::io::Write;



fn main() {
    println!("Temperature Converter CLI");

    // println!("Enter name: ");
    // let mut name = String::new();
    // std::io::stdin().read_line(&mut name).expect("Failed to get name");
    // println!("Hello, {}", name.trim());

    loop {
        println!("\nSelect conversion: ");
        println!("1: Celsius to Fahrenheit");
        println!("2: Fahrenheit to Celsius");
        println!("3: Kelvin to Fahrenheit");
        println!("4: Fahrenheit to Kelvin");
        println!("5: Celsius to Kelvin");
        println!("6: kelvin to Celsius");
        println!("Please select an option ( 1 - 6): ");
        std::io::stdout().flush().expect("Please retype a new option");


        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read your input");

        let user_choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Enter a valid number");
                return;
            }
        };

        match user_choice {
            1 => celsius_to_fahrenheit(),
            2 => fahrenheit_to_celsius(),
            3 => kelvin_to_fahrenheit(),
            4 => fahrenheit_to_kelvin(),
            5 => celsius_to_kelvin(),
            6 => kelvin_to_celsius(),
            _ => {
                println!("Invalid choice. Enter a number between 1 and 6");
                break;
            }
        }

        break;// To break the loop after first input is evaluated
    };
    
}



// 3 standards or UNITS
/*
1. Celsius <=> Fahrenheit
2. Kelvin <=> Celsius
3. Fahrenheit <=> Kelvin
*/

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius: ");

    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("Failed to read input");

    let temp: f64 = match temperature.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };
    let fahrenheit = (temp * 9.0/5.0) + 32.0;
    println!("{:.2}C is {:.2}F", temp, fahrenheit);
    println!("---------");
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit: ");
  
    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("Failed to Read user input");

    let temp: f64 = match temperature.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input. Please enter a valid number. ");
            return;
        }
    };
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2}F is {:.2}C", temp, celsius);
    println!("----------");
}

fn kelvin_to_celsius() {
    println!("Enter temperature in kelvin");

    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("Failed to read user input");

    let temp: f64 = match temperature.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid value. Please enter a valid number.");
            return;
        }
    };

    let celsius = temp - 273.15;
    println!("{:.2}K is {:.2}C", temp, celsius);
    println!("-----------------------");
}

fn celsius_to_kelvin() {
    println!("Enter temperature in Celsius: ");

    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("Failed to capture user input");

    let temp: f64 = match temperature.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid value. Please enter a valid number. ");
            return;
        }
    };

    let kelvin = temp + 273.15;
    println!("{:.2}C is {:.2}K", temp, kelvin);
}

fn fahrenheit_to_kelvin() {
    println!("Enter temperature in Fahrenheit: ");

    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("Failed to capture user input.");

    let temp: f64 = match temperature.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid value. Please provide a valid number. ");
            return;
        }
    };

    let kelvin = (temp - 32.0) * (5.0 / 9.0) + 273.15;
    println!("{:.2}F is {:.2}K", temp, kelvin);
    println!("--------------");
}

fn kelvin_to_fahrenheit() {
    println!("Enter temperature in kelvin: ");

    let mut temperature = String::new();
    std::io::stdin().read_line(&mut temperature).expect("Failed to read user input");

    let temp: f64 = match temperature.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid value. Enter a valid number. ");
            return;
        }
    };
    let fahrenheit = (temp - 273.15) * (9.0 / 5.0) + 32.0;
    println!("{:.2}K is {:.2}F", temp, fahrenheit);
    
}