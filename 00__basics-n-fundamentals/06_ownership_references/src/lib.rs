// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables, unused_doc_comments)]

/**
 * inspect() takes s as a reference to a String,
 * and then calls s.ends_with("s"),
 * the `.` dereferences down to the value and calls the ends_with value on the value *
 * @return void
 */

pub fn inspect(s: &String) -> bool {
    if s.ends_with("s") {
        println!("It's a plural word");
        return true;
    } else {
        println!("The word is on singular!");
        return false;
    }
}

/**
 * change() take a mutable reference to a string
 * check if the string ends_with s
 * if not, add an s using push_str(), this only works on mutable Strings
 * @return void
 */
pub fn change(resp: bool, s: &mut String) {
    if !resp {
        s.push_str("s"); // only works on mutable strings
    } else {
        println!("No, just not...");
    }
}

/**
 * eat() takes a variable by value and it is going to consume it, that mean tha variable passed to it will not be useable after the function call
 * @return bool
 */

pub fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        return true;
    } else {
        return false;
    }
}

/**
 * bedazzle() takes a mutable reference to a String, ignores that it is a string and replaces what is in the string with "sparkly"
 * @return void
 */
pub fn bedazzle(s: &mut String) {
    *s = String::from("sparkly"); // s is dereferenced in order to assign a new value
}
