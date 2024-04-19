use std::io;

fn main() {
    let mut number = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to readline");
    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter a number");
            std::process::exit(1);
        }
    };

    if number < 5 {
        println!("number {number} < 5, condition was true");
    } else {
        println!("number {number} >= 5, condition was false");
    }

    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    // let number = if number < 5 {
    //     number * 2
    // } else {
    //     "number is too big" // compile error because the type of the value is different
    // };
    let number = if number < 5 { number * 2 } else { number / 2 };
    println!("The value of number is {number} after the condition");

    let mut counter = 0;
    let number = loop {
        counter += 1;
        if counter == 10 {
            break number + counter;
        }
    };
    println!("The value of number is {number} after loop ");

    let mut count = 0;
    'outer_loop_counting_up: loop {
        println!("counting up loop");
        let mut remaining = 10;
        loop {
            //inner loop
            println!("{remaining} remaining");
            if remaining == 9 {
                break; //this break the inner loop
            }
            if count == 2 {
                break 'outer_loop_counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = number;
    println!("while loop countdown number");
    while number != 0 {
        println!("{number} != 0");
        number -= 1;
    }
    println!("End of while loop");

    let arr = [1, 2, 3, 4, 5];

    for element in arr {
        println!("the value is {element}");
    }

    for index in 0..5 {
        println!("for loop index is {index}");
        println!("the number is {number}");
    }
}
