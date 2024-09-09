// Functions
// Entry point
// any function / _07-variables should be written in snake case
fn main() {
    hello_world();
    tell_height(182);
    human_id("Ahmed Habib", 23, 185.0);
    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("x price * qty = {}", _x);
    let y =add(5,6);
    println!("y is {}", y);
}

// Hoisting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, world!");
}

// you can insert input values
fn tell_height(height: i32) {
    println!("My height {} cm.", height);
}

// you can insert more than one value
fn human_id(name: &str, age: u32, height: f32) {
    "test".to_string();
    println!("my name is {}, my age is {} and my height is {} cm.", name, age,height);
}

fn add(a: i32, b:i32) -> i32 {
     a + b
}

