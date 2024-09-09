 fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
       None
    }else {
        Some(numerator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result = divide_option(3.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by 0")
    }

    let result = divide_result(3.0, 0.0);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(err) => println!("Error: {}", err)
    }
}
