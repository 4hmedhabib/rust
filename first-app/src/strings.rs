pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push Char
    hello.push('W');

    // Push String
    hello.push_str("orld");

    // Campacity in bytes
    println!("Campacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assertion for String
    assert_eq!(2, s.len());

    println!("{}", hello);
}
