fn main() {
    // // loop keyword
    // loop {
    //     println!("Hello, world!");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result);

    // // While loop
    // let mut counter = 0;
    // while counter < 10 {
    //     counter += 1;
    //     println!("counter: {}", counter);
    //     break;
    // }

    // Loop with collection
    let elements = [1,2,3,4,5,6];
    for element in elements {
        println!("Element: {}", element);
    }
}
