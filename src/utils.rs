use std::io;

pub fn parse_u16_input(prompt: &str) -> u16 {
    let mut input = String::new();
    println!("{}", prompt);
    let number = match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let number: u16 = input.trim().parse().unwrap_or_else(|error| {
                println!("error: {}", error);
                0
            });
            number
        },
        Err(error) => {
            println!("error: {}", error);
            0
        }
    };
    number
}

pub fn parse_f64_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    let number = match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let number: f64 = input.trim().parse().unwrap_or_else(|error| {
                println!("error: {}", error);
                0.00
            });
            number
        },
        Err(error) => {
            println!("error: {}", error);
            0.00
        }
    };
    number
}