use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");

    loop {
        println!("\nSelect an option:");
        println!("1. Convert Celsius to Fahrenheit");
        println!("2. Convert Fahrenheit to Celsius");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let celsius = get_temperature("Celsius");
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.2} °C = {:.2} °F", celsius, fahrenheit);
            }
            2 => {
                let fahrenheit = get_temperature("Fahrenheit");
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{:.2} °F = {:.2} °C", fahrenheit, celsius);
            }
            3 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Please enter a valid option (1, 2, or 3)."),
        }
    }
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Function to get temperature input from the user
fn get_temperature(unit: &str) -> f64 {
    println!("Enter the temperature in {}:", unit);

    let mut temp_input = String::new();
    loop {
        io::stdin().read_line(&mut temp_input).expect("Failed to read input");
        match temp_input.trim().parse() {
            Ok(temp) => return temp,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                temp_input.clear();
            }
        }
    }
}
