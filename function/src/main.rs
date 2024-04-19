fn main() {
    println!("Hello, world!");
    global_function();
    {
        fn inside_function() {
            println!("inside_function");
        }

        inside_function();
        global_function(); //legal
    }
    // inside_function(); compile error
    global_function();

    another_function(5, 'A');

    let y = {
        let x = 3;
        println!("x :{x}");
        x + 1 //an expression to give value to y
    };
    // println!("x :{x}"); // compile error
    println!("y :{y}");

    let x = five();
    println!("x :{x}");
    let x = plus_one(x);
    println!("x :{x}");
}

fn global_function() {
    println!("global_function");
}

fn another_function(value: i32, letter: char) {
    println!("value : {value}, letter :{letter}");
}

fn five() -> i32 {
    5
}

fn plus_one(x : i32) -> i32 {
    x + 1
}