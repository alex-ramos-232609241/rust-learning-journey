fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i64, b: i64) -> i64 {
    a*b
}

fn main() {
    let result = add(5, 10);
    println!("The sum is: {}", result);
    
    let value = multiply(50000, 200000);
    println!("The multiply is: {}", value);
    
}
