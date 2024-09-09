// Ownership
// [Stopping/Resuming the program]
// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is _05-ownership?
// Every value has a single owner [every variable has one
fn main() {
    // Rules
    // * Each value in Rust has a variable that's its owner.
    let s1 = String::from("hello");
    println!("s1 is {}", calculate_length(&s1));
    // * There can be only one owner at a time;
    let s2 = s1;
    // println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    // * When the owner goes out of scope, the value will be dropped.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
