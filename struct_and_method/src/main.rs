fn main() {
    let user1 = User {
        active: true,
        username: String::from("John"),
        email: String::from("john@john.com"),
        sign_in_count: 1,
    }; //the order of the fields does not matter
    println!("user1: {}", user1.username);

    let mut user2 = User {
        active: true,
        username: String::from("Adam"),
        email: String::from("adam@adam.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("adam2@adam.com");
    println!("user2 email is changed to \"{}\"", user2.email);

    let user3 = build_user("Amy@amy.com".to_string(), "Amy".to_string());
    println!("user3 username is {}", user3.username);

    let user4 = User {
        active: user1.active,
        username: String::from("Teddy"),
        email: "teddy@teddy.com".to_string(),
        sign_in_count: user1.sign_in_count,
    };
    println!(
        "user4 name {}, email {}, active_status {}, sign_in_count {}",
        user4.username, user4.email, user4.active, user4.sign_in_count
    );

    let mut new_address = user2.email[..user2.username.len()].to_string();
    new_address.push_str("@new.com");
    println!("user2 email {}", user2.email);
    println!("new_address \"{}\"", new_address);
    let user5 = User {
        email: new_address,
        ..user2
    };
    //the ownership of user2 is moved to user5 and become immutable
    //because user5 still used the reference of user2's username.
    println!(
        "user5 name \"{}\", email \"{}\"",
        user5.username, user5.email
    );
    // println!("user2 is invvalid {}",user2.username);

    let user6 = User {
        email: String::from("user6@user6.com"),
        username: String::from("user6"),
        ..user5
    };
    println!(
        "user6's name \"{}\" and email \"{}\"",
        user6.username, user6.email
    );
    println!("user5 {} is valid ", user5.username);
    //the ownership of user5 not moved because the sign_in_count nad active are on the stack
    //they can automatically copy from the user5.
    println!("user6 information {:#?}", user6);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black value ({},{},{})", black.0, black.1, black.2);
    println!("origin point is ({},{},{})", origin.0, origin.1, origin.2);
    // let new_color = add_color(black, origin); //doesn't work! even two tuple struct used same data types with same amount.
    let red = Color(255, 0, 0);
    let new_color = add_color(red, black);
    println!(
        "new color is ({},{},{})",
        new_color.0, new_color.1, new_color.2
    );

    let scale = 2;
    let rec1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );
    println!("rec1 is {:?}", rec1);
    dbg!(&rec1);

    println!("the area of rec1 is {}", rec1.area());

    let rec2 = Rectangle {
        width: 12,
        height: 25,
    };

    let rec3 = Rectangle {
        width: 100,
        height: 22,
    };

    println!("Can rec1 hold rec2 ? {} ", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3 ? {} ", rec1.can_hold(&rec3));

    let squ1 = Rectangle::square(30);
    println!("squ1 are = {}", squ1.area());
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(edge_length: u32) -> Rectangle{
        Rectangle{
            width: edge_length,
            height: edge_length,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn add_color(color1: Color, color2: Color) -> Color {
    Color(
        color1.0 + color2.0,
        color1.1 + color2.1,
        color1.2 + color2.2,
    )
}

#[derive(Debug)]
struct User {
    sign_in_count: u64,
    active: bool,
    username: String,
    email: String,
}

fn build_user(email: String, username: String) -> User {
    User {
        sign_in_count: 1,
        active: true,
        username,
        email,
    }
}
