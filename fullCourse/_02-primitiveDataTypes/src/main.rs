// Primitive data types
// int, float, bol

// Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: UnSigned integers.
fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("UnSigned Integer: {}", y);
    // diff bet i32 (32bits) and i64(64 bits)
    // range:
    // i32 - 2,147,483,647
    // i64 - 9,223,372,036,854,775,807
    let e: i32 = 2_147_483_647;
    let i: i64 = 9_223_372_036_854_775_807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
    //================================================
    // Floats [Floating Point Types]

    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);
    //================================================
    // Boolean Values: true, false
    let is_snowing: bool = true;
    println!("Is it snowing ? {}", is_snowing);


}
