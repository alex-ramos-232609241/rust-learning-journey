use num_bigint::{BigInt, ToBigInt};

fn factorial(n: i64) -> BigInt {
    let mut factorial = 1.to_bigint().unwrap();
    for i in 2..=n {
        factorial = factorial * i.to_bigint().unwrap();
    }
    factorial
}

fn main() {
    let num = 1000;
    let result = factorial(num);

    println!("The factorial of {} is:\n{}", num, result);
}
