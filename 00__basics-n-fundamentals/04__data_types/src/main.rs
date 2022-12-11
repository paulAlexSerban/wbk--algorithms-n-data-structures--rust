fn main() {
    // INTEGERS
    // implicitly defining int
    let x = 6;
    // explicitly defining unsigned int
    let y: u16 = 9;
    // explicitly defining signed int
    let z: i16 = -7;
    println!("{} {} {}", x, y, z);

    // FLOATING POINTS NUMBERS
    // implicit floating number type
    let a = 200.000;
    // explicitly floating point type
    let b: f64 = 9.894;
    let c: f64 = -7.4;
    println!("{} {} {}", a, b, c);

    // BOOLEAN
    let is_active: bool = true;
    println!("{}", is_active);

    // CHARACTER
    let s = 'z';
    let c = 'ℤ';
    println!("{} {}", s, c);

    // TUPLE
    let gfg_tup: (&str, &str, &str) = ("Geeks", "For", "Geeks");

    // accessing tuple data using positional argument
    println!("{} {} {}", gfg_tup.0, gfg_tup.1, gfg_tup.2);

    // creating another tuple
    let article = ("geeksforgeeks", "kushwanthreddy", 14, 12, 2020);
    let (a, b, c, d, e) = article;

    // accessing tuple using variables
    println!(
        "This article is written by {} at {} on {}/{}/{}",
        b, a, c, d, e
    );

    // ARRAY
    let gfg_arr = ["Geeks", "For", "Geeks"];
    // accessing array data using positional argument
    println!("{} {} {}", gfg_arr[0], gfg_arr[1], gfg_arr[2]);

    // STRINGS
    
}
