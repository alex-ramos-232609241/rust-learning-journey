fn divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        return Err("Cannot divide by zero.".to_string());
    }
    Ok(dividend / divisor)
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
