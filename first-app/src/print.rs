pub fn  run() {
    // Print to console
    println!("Hello, from print.rs file");

    //Basic formatting
    println!("{}",1);

    println!("{} is from {}", "Ahmed", "Hargeisa");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2} ", "Ahmed", "Hargeisa", "code");

    // Named Arguments
    println!("{name} likes to play {activitiy}", name = "Ahemd",  activitiy="football");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10,10,10);
    
    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10)
    
}