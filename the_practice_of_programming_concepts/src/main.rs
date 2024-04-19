use std::io;

fn main() {
    println!("Please enter Celsius degree :");
    let mut degree = String::new();
    match io::stdin().read_line(&mut degree) {
        Ok(_) => {}
        Err(_) => {
            println!("Error reading input");
            std::process::exit(1);
        }
    }

    let degree: f64 = match degree.trim().parse::<f64>() {
        Ok(degree) => degree,
        Err(_) => {
            println!("please enter a valid number");
            std::process::exit(1);
        }
    };
    let fahrenheit = convert_celsius_to_fahrenheit(degree, true);
    println!("{degree} Celsius is {fahrenheit} Fahrenheit");

    let mut level = String::new();
    println!("Please enter the level of fabonacci :");
    match io::stdin().read_line(&mut level) {
        Ok(_) => {}
        Err(_) => {
            println!("Error reading input");
            std::process::exit(1);
        }
    };

    let level: i32 = match level.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("please enter a valid number");
            std::process::exit(1);
        }
    };
    println!("Fabonacci of {level} is {}", fabonacci(level));
}

fn fabonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        fabonacci(n - 1) + fabonacci(n - 2)
    }
}

fn convert_celsius_to_fahrenheit(degree: f64, c2f: bool) -> f64 {
    if c2f {
        degree * 1.8 + 32.0
    } else {
        (degree - 32.0) / 1.8
    }
}
