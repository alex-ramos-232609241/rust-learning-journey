fn split(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Err: Split by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match split(10, 2) {
        Ok(value) => println!("The result is: {}", value),
        Err(e) => println!("{}", e),
    }
}
