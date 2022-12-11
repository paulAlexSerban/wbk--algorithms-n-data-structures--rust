// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;
    // loop from 7 to 23 *inclusive*
    for i in 7..=32 {
        sum += i;
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x < 500 {
        count += 1;
        x *= 2;
    }
    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    for i in 0..8 {
        print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
    }
    println!(); // This will output just a newline at the end for cleanliness.
}
