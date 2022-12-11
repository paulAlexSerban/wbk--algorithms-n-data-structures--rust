const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
const EXTRA_MISSILES: i32 = 3;
const COMING_MISSILES: i32 = 4;

fn main() {
    let mut missiles: i32 = STARTING_MISSILES; // Fix error by doing: let mut missiles = 8
    let ready: i32 = READY_AMOUNT;
    // you can also assign variables on same line
    let (extra, coming): (i32, i32) = (EXTRA_MISSILES, COMING_MISSILES);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready; // Error! if missiles is missing mut (mutability)
    println!("{} missiles left", missiles);
    println!(
        "The ship get {} extra missiles, and awaits {} from deposit",
        extra, coming
    );
}
