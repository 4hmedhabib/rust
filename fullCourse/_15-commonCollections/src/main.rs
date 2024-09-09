use std::collections::HashMap;

fn main() {
    //======================== VECTORS
    // let _v:Vec<i32> = Vec::new();
    // let mut _the_vec: Vec<i32> = vec![1, 2, 3];
    //
    // _the_vec.push(4);
    //
    // println!("{:?}", _the_vec);

    // let _v = vec![1, 2, 3, 4, 5];
    // let third = &_v[2];
    // println!("The third element is {third}");
    //
    // let third = _v.get(3);
    // println!("The third element is {third}");

    // let third = _v.get(4);
    // match third {
    //     None => println!("There is no third element."),
    //     Some(value) => println!("The third element is {value}."),
    // }

    //======================== UTF-8'
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // s.push('!');
    // println!("string value is: {}", s);
    //
    // let salam = "Salam".to_string();
    // let name = "Ahmed".to_string();
    // let full_message = format!("{} {}", salam, name);
    // println!("{}", full_message);

    //======================== HashMap'
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team Name is {} and score is {}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}
