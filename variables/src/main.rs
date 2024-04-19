use std::io;
fn main() {
    let x = 5;
    println!("The value of x is {x}");
    // x = 6; // error: cannot assign twice to immutable variable `x`

    let mut y = 5;
    println!("The value of mut y is {y}");
    y = 6;
    println!("The value of mut y is {y}");

    let x = x + 1;
    println!("The vaule of shadowing x in the outter scope is {x}");
    {
        let x = x * 2;
        println!("The value of shadowing x in inner scope is {x}");
    }
    println!("The vaule of shadowing x after leaving the inner scope is {x}");

    // mutable can not change the data type
    // let mut spaces = "    ";
    // spaces = spaces.len(); // error[E0308]: mismatched types

    let spaces = "    ";
    let spaces = spaces.len(); // use shadowing to change the type of spaces
    println!("The value of spaces is {spaces}");

    let fx = 2.0; //f64
    let fy: f32 = 3.0; //f32
    println!("the value of fx is {fx}, fy is {fy}");
    let fy = fy as f64; // convert f32 to f64
    let sum: f64 = fx + fy;
    print!("the value of sum is {sum}");
    let dif = fx - fy;
    println!("the value of dif is {dif}");
    let mut truncated = -59 / 30;
    println!("-59/30 is truncated to {truncated}"); //-1
    truncated = 59 / 30;
    println!("59/30 is truncated to {truncated}"); //1
    let quotient = 59.0 / 30.0;
    println!("59.0/30.0 is {quotient}"); //1.9666666666666667
    let remainder = 59 % 30;
    println!("59%30 is {remainder}"); //29

    let t = true;
    let f: bool = false;
    println!("the value of t is {t}, f is {f}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!(
        "the value of a is {a}, b is {b}, c is {c} and tup ({},{},{})",
        tup.0, tup.1, tup.2
    );
    let arr = [1,2,3,4,5];
    println!("the value of arr is {:?}", arr);
    let first = arr[0];
    let second = arr[1];
    println!("the value of first is {first}, second is {second}");
    let arr2 : [f64; 5] = [1.1,2.2,3.3,4.4,5.5];
    println!("the value of arr2 is {:?}", arr2);
    let arr3 = [3.0; 5];
    println!("the value of arr3 is {:?}", arr3);

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index : usize = index.trim().parse().expect("Please type a number!");
    let element = arr[index];

    println!("the element at index {index} is {element}");

    
}
