// Variables hold primitive data or references  to data
// Variables are immutable by default 
// Rust is a block-scoped language

pub fn run(){
    let name = "Ahmed Habib";
    let mut age = 37;

    // haddii aad doonayso inaad mar labaad labadali karo variable value ga waa inaad raaciya (mut)

    println!(" My name is {} and I am {}", name, age);

    age = 20;
    println!(" My name is {} and I am {}", name, age);

    // Define constant variable
    // - marka const key word samaynayso wa inay ahaadaan capital letters
    const ID: i32 = 0010;
    println!("ID: {}", ID);


    // Assign multi variables
    let (my_name, my_age) = ("Ahmed Habib", 21);
    println!("{} is {} year old", my_name , my_age);
}