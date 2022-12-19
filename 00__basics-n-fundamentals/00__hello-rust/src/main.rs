// Silence some warnings so they don't distract from the exercise.
#![allow(unused_assignments)]

fn main() {
    hello_world("Paul")
}

fn hello_world(name: &str) {
    let str = String::from("Hello, world!");
    let lng = String::from("Rust");

    // create new string
    let mut str_date = String::new();

    // declare the string literal
    let date = "December 2022 ;)";

    // create a String from a string literal
    str_date = date.to_string();
    println!("{} I am {}, and I am learning {} in {}", str, name, lng, str_date);
}

// in VSC, right click + Run Code