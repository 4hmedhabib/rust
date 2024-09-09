fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers Array: {:?}", numbers);
    let fruits: [&str; 3] = ["apple", "orange", "banana"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {:?}", fruits[0]);
    println!("Fruits Array 2nd element: {:?}", fruits[1]);
    println!("Fruits Array 3rd element: {:?}", fruits[2]);
    // Tuples
    let human: (String, i32, bool) = ("Ahmed".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Ahmed Habib", 23, true, [1,2,3,4,5]);
    println!("my_mix_tuple: {:?}", my_mix_tuple);
    // Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string()];
    println!("Book slices: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str(" Yeah!");
    println!("Stone Cold: {}", stone_cold);

    // B- &str (String slice)
    let string: String = String::from("hello, world");
    let slice: &str = &string[0..5];
    println!("Slice value: {}", slice);
}

