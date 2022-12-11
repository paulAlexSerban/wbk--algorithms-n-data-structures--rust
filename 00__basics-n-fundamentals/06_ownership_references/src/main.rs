// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables, unused_doc_comments)]

use ownership_references::inspect;
use ownership_references::change;
use ownership_references::eat;
use ownership_references::bedazzle;

fn main() {
    /**
     * get the first argument as a String and exists ir argument was not supplied to
     */
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    let result:bool = inspect(&arg);
    change(result, &mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
