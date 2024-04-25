#[derive(Debug)]
enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // data associated with named field
    Write(String),              // a string
    ChangeColor(i32, i32, i32), //a tuple
}

fn main() {
    let msg1 = Message::Write("the message".to_string());
    // println!("msg1 {:?}", msg1);
    let msg2 = Message::Quit;
    // println!("msg2 {:?}", msg2);
    let msg3 = Message::Move { x: 12, y: 32 };
    let msg4 = Message::ChangeColor(32, 155, 33);
    process_messge(&msg1);
    println!("{:?}", msg1); //msg1 is valid because we didn't move the ownership
    process_messge(&msg2);
    process_messge(&msg3);
    process_messge(&msg4);

    let five = Some(5);
    let six = plus_one(five);
    println!("plus_one five {:?}", six);
    let none = plus_one(None);
    println!("none is {:?}", none);

    let coin = Coin::Quarter(UsState::Alabama);
    let coin = value_in_cents(&coin);
    println!("this coin value {coin}");

    let penny = Coin::Penny;
    let penny = value_in_cents(&penny);
    println!("penny value {penny}");

    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    println!(
        "nickel + dime = {}",
        value_in_cents(&nickel) + value_in_cents(&dime)
    );

    if let Message::Write(str1) = msg1 {
        println!("msg1 is Message::Write(string) with \"{}\"", str1);
    }
    // println!("{:?}",msg1); msg1 is invalid because the value is moved into the above scope
}

fn process_messge(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Write(text) => println!("The message is \"{}\"", text),
        Message::ChangeColor(r, g, b) => println!("The color is {r},{g},{b}"),
        Message::Move { x, y } => println!("Move to point ({x},{y})"),
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    // Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } //use curly brackets, and the comma following the arm is then optional.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("this quater is minted from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
