fn main() {
    // s is not valid here, it’s not yet declared
    {
        let s = "hello"; // s is valid from this point forward
                         // s = "world"; // s can not be reassigned because it is immutable
        let mut ss = "hello";
        println!("original ss : {ss}");
        ss = " world"; // ss can be reassigned because it is mutable
        println!("ss after reassign : {ss}");
        // ss += "new word"; // &str can not be appended because compiler can not infer the length of ss.

        // do stuff with s
        println!("I can print s : {s}");
    }
    // this scope is now over, and s is no longer valid

    let mut s2 = String::from("hello"); // Allocate a new mutable string from the heap.
    s2.push_str(", world!"); // Push a string slice to the string.
    s2 += " append word";
    println!("I can print s2 : {s2}"); // Print the string.

    {
        let s3 = String::from("hello"); // s3 is valid from this point forward
        println!("I can print s3 : {s3}");
        let mut s4 = s3; // s3 is no longer valid.
                         // println!("s3 is no longer valid {s3}");
        s4 += " append word";
        println!("s4 hold the memory of s3 and become mutable {s4}");

        let mut s5 = s4.clone();
        println!("s5 is a clone of s4 and become mutable : {s5}");
        println!("s4 is still valid {s4}");

        s5 += " different word";
        println!("s5: {s5} is differnt from \ns4: {s4}");

        takes_ownership(s4); //s4 ownersher move into the function
                             // println!("{s4} is invalid");
                             // s4 = gives_ownerhip(); //return a String ownership to s4 doesn't work because s4 is invalid
        let s4 = gives_ownerhip();
        println!("s4 is valid again because it is a new variable {s4}");

        let (s5, len) = takes_and_gives_back(s4); //s5 is moved into the function and then returned to s5
        println!("s5 become the owner of s4 {s5} with string length {len}");
    } // s5 goes out of scope and `drop` is called. The backing memory is freed.
    let x = 5; // x is in the stack
    makes_copy(x);
    println!("{x} is still valid");

    let s6 = String::from("s6");
    let len_s6 = get_string_length(&s6); //used reference to refer to the value without taking ownership of it.

    println!("The length of \"{s6}\" is {len_s6}"); //s6 still valid.

    let mut s7 = String::from("s7");
    let len_s7 = mut_string_and_get_length(&mut s7);
    println!("The length of \"{s7}\" is {len_s7}"); //s6 still valid.

    let mut s8 = String::from("s8");
    let r1 = &s8; // no problem
    let r2 = &s8; // no problem
    println!("s8 ref1 = \"{r1}\", ref2 =\"{r2}\"");
    let r3 = &mut s8; // no problem
                      // println!("we can not use {r1}, {r2} after r3 because r3 is mutable reference");
                      // let r4 = &mut s8; // we can not have multiple mutable reference to the same value.
    let len_s8 = mut_string_and_get_length(r3);
    // println!("The length of \"{s8}\" is {len_s8}"); // {s8} is a immutable reference to s8
    println!("The length of \"{r3}\" is {len_s8}"); // use r3 instead of s8
    r3.push_str(" modified by ref3");
    println!("s8 after modification by ref3 : \"{s8}\""); // s8 is valid because no longer used mutable reference of s8

    let word = first_word(&s8);
    // s8.clear(); //word is immutable but s8 in clear() method is mutable
    println!("The first word of \"{s8}\" is \"{word}\"");
    s8.clear(); //no other references used after this line.

    let word = first_word(&s7); //&String is not flexible because it can only be used with String.
    println!("The first word of \"{s7}\" is \"{word}\"");
    // let word = first_word("not work");
    let string_literal = "this can work";
    let word = first_word_in_stringslice(string_literal); //instead of using &String, use &str more flexible.
    println!("The first word of \"{string_literal}\" is \"{word}\"");
    let word = first_word_in_stringslice(&string_literal[..5]);
    println!(
        "The first word of \"{}\" is \"{word}\"",
        &string_literal[..5]
    );
    let word = first_word_in_stringslice(&string_literal[5..]);
    println!(
        "The first word of \"{}\" is \"{word}\"",
        &string_literal[5..]
    );
}

fn takes_ownership(some_string: String) {
    println!("I took ownership of the string : {some_string}");
} //som_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer} is a copy of the integer passed to the function");
}

fn gives_ownerhip() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> (String, usize) {
    let length = a_string.len();
    (a_string, length)
}

fn get_string_length(s: &String) -> usize {
    // s.push_str(string); //can not modify the value of s because it is a reference to a immutable value.
    s.len()
}

fn mut_string_and_get_length(s: &mut String) -> usize {
    s.push_str(" append word");
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_in_stringslice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
